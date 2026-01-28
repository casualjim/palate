#!/usr/bin/env -S bun
/**
 * Merge grammars.json and nvim-treesitter parsers by matching repository URLs
 */

const grammarsJsonPath = "/home/ivan/github/casualjim/breeze-tree-sitter-parsers/grammars.json";
const parsersLuaPath = "/home/ivan/.local/share/nvim/lazy/nvim-treesitter/lua/nvim-treesitter/parsers.lua";
const outputPath = "/home/ivan/github/casualjim/palate/target/grammars-mapping.json";
const cachePath = "/home/ivan/github/casualjim/palate/target/url-redirect-cache.json";

// Read grammars.json (ground truth)
const grammarsData = JSON.parse(await Bun.file(grammarsJsonPath).text());
const grammars = grammarsData.grammars;

console.log(`📦 Loaded ${grammars.length} grammars from grammars.json`);

// Read parsers.lua to get parser info
const parsersContent = await Bun.file(parsersLuaPath).text();

interface NvimParser {
  name: string;
  url: string;
  filetype: string | null;
}

// Map from nvim filetype to canonical filetype (what we serialize to)
const CANONICAL_FILETYPES: Record<string, string> = {
  "cs": "csharp",
  "typescriptreact": "tsx",
  "fsharp": "fsharp",  // already correct
  "confini": "ini",
  "dsp": "faust",
  "vlang": "v",
};

// Get canonical filetype for a given nvim filetype
function getCanonicalFiletype(nvimFiletype: string | null): string | null {
  if (!nvimFiletype) return null;
  return CANONICAL_FILETYPES[nvimFiletype] ?? nvimFiletype;
}

// Parse nvim-treesitter parsers to extract name, url, and filetype
const nvimParsers: NvimParser[] = [];
const parserBlocks = parsersContent.split(/\nlist\./).slice(1); // Skip first empty element

for (const block of parserBlocks) {
  // Extract parser name (first word before =)
  const nameMatch = block.match(/^([\w]+)\s*=/);
  if (!nameMatch) continue;
  const parserName = nameMatch[1];

  // Extract URL from install_info.url
  const urlMatch = block.match(/url\s*=\s*["']([^"']+)["']/);
  if (!urlMatch) continue; // Skip parsers without URL
  const url = urlMatch[1];

  // Look for filetype in this block
  const filetypeMatch = block.match(/filetype\s*=\s*["']([^"']+)["']/);
  const filetype = filetypeMatch ? filetypeMatch[1] : parserName; // Default to parser name

  nvimParsers.push({ name: parserName, url, filetype });
}

console.log(`📝 Loaded ${nvimParsers.length} parsers from nvim-treesitter`);

// Load or create redirect cache
let urlCache: Record<string, string> = {};
try {
  urlCache = JSON.parse(await Bun.file(cachePath).text());
  console.log(`📦 Loaded ${Object.keys(urlCache).length} cached redirects`);
} catch {
  console.log("📦 No cache found, starting fresh");
}

/**
 * Resolve URL to its canonical form by following redirects (with caching)
 */
async function resolveCanonicalUrl(url: string): Promise<string> {
  // Check cache first
  if (urlCache[url]) {
    return urlCache[url];
  }

  try {
    const response = await fetch(url, { method: "HEAD", redirect: "follow" });
    const canonicalUrl = response.url;
    // Cache the result
    urlCache[url] = canonicalUrl;
    return canonicalUrl;
  } catch {
    // Cache failures as the original URL
    urlCache[url] = url;
    return url;
  }
}

/**
 * Save the cache
 */
async function saveCache() {
  await Bun.write(cachePath, JSON.stringify(urlCache, null, 2));
}

// Resolve canonical URLs for all nvim parsers concurrently
console.log("Resolving canonical URLs for nvim-treesitter parsers...");
const nvimParsersWithCanonical = await Promise.all(
  nvimParsers.map(async (parser) => ({
    ...parser,
    canonicalUrl: await resolveCanonicalUrl(parser.url),
  }))
);
console.log("✅ Resolved all URLs");

// Build maps using canonical URLs - handle multiple parsers per URL
const nvimByCanonicalUrl: Record<string, NvimParser[]> = {};
for (const parser of nvimParsersWithCanonical) {
  if (!nvimByCanonicalUrl[parser.canonicalUrl]) {
    nvimByCanonicalUrl[parser.canonicalUrl] = [];
  }
  nvimByCanonicalUrl[parser.canonicalUrl].push(parser);
}

// Also build repo name map as fallback - handle multiple parsers per repo name
const nvimByRepoName: Record<string, NvimParser[]> = {};
for (const parser of nvimParsersWithCanonical) {
  const repoName = parser.canonicalUrl.split("/").pop()?.replace(/\.git$/, "") || "";
  if (!nvimByRepoName[repoName]) {
    nvimByRepoName[repoName] = [];
  }
  nvimByRepoName[repoName].push(parser);
}

// Build original URL map for exact matches - handle multiple parsers per URL
const nvimByUrl: Record<string, NvimParser[]> = {};
for (const parser of nvimParsersWithCanonical) {
  if (!nvimByUrl[parser.url]) {
    nvimByUrl[parser.url] = [];
  }
  nvimByUrl[parser.url].push(parser);
  if (parser.url.endsWith(".git")) {
    const urlWithoutGit = parser.url.slice(0, -4);
    if (!nvimByUrl[urlWithoutGit]) {
      nvimByUrl[urlWithoutGit] = [];
    }
    nvimByUrl[urlWithoutGit].push(parser);
  }
}

// Resolve canonical URLs for all grammars.json repos concurrently
console.log("Resolving canonical URLs for grammars.json repos...");
const grammarsWithCanonical = await Promise.all(
  grammars.map(async (grammar: { name: string; repo: string }) => ({
    ...grammar,
    canonicalUrl: await resolveCanonicalUrl(grammar.repo),
  }))
);
console.log("✅ Resolved all URLs");

// Match grammars to nvim parsers by repo URL or repo name
const mergedMapping = grammarsWithCanonical.map((grammar) => {
  const grammarName = grammar.name;
  const repoUrl = grammar.repo;
  const canonicalUrl = grammar.canonicalUrl;
  const repoName = repoUrl.split("/").pop()?.replace(/\.git$/, "") || "";

  // Try exact URL match first
  let nvimParsersByUrl = nvimByUrl[repoUrl];
  let nvimParser = nvimParsersByUrl?.[0];
  let matchType: string | null = null;

  // Prefer parser with matching name
  if (nvimParsersByUrl) {
    const nameMatch = nvimParsersByUrl.find((p: NvimParser) => p.name === grammarName);
    if (nameMatch) {
      nvimParser = nameMatch;
      matchType = "exact_url_with_name";
    } else {
      nvimParser = nvimParsersByUrl[0];
      matchType = "exact_url";
    }
  }

  // If no exact match, try canonical URL match
  if (!nvimParser) {
    const nvimParsersByCanonical = nvimByCanonicalUrl[canonicalUrl];
    if (nvimParsersByCanonical) {
      const nameMatch = nvimParsersByCanonical.find((p: NvimParser) => p.name === grammarName);
      if (nameMatch) {
        nvimParser = nameMatch;
        matchType = "canonical_url_with_name";
      } else {
        nvimParser = nvimParsersByCanonical[0];
        matchType = "canonical_url";
      }
    }
  }

  // If no canonical match, try matching by repo name (handles different forks)
  if (!nvimParser && repoName) {
    const nvimParsersByRepoName = nvimByRepoName[repoName];
    if (nvimParsersByRepoName) {
      const nameMatch = nvimParsersByRepoName.find((p: NvimParser) => p.name === grammarName);
      if (nameMatch) {
        nvimParser = nameMatch;
        matchType = "repo_name_with_name";
      } else {
        nvimParser = nvimParsersByRepoName[0];
        matchType = "repo_name";
      }
    }
  }

  if (nvimParser) {
    return {
      grammar: grammarName,
      grammar_repo: repoUrl,
      nvim_parser: nvimParser.name,
      nvim_repo: nvimParser.url,
      nvim_filetype: nvimParser.filetype,
      effective_filetype: getCanonicalFiletype(nvimParser.filetype),
      match_type: matchType,
    };
  }

  // No match found
  return {
    grammar: grammarName,
    grammar_repo: repoUrl,
    nvim_parser: null,
    nvim_repo: null,
    nvim_filetype: null,
    effective_filetype: grammarName,
    match_type: null,
  };
});

// Sort by grammar name
mergedMapping.sort((a, b) => a.grammar.localeCompare(b.grammar));

// Write output
await Bun.write(outputPath, JSON.stringify(mergedMapping, null, 2));

// Save the redirect cache
await saveCache();

console.log(`✅ Merged ${mergedMapping.length} grammars with nvim-treesitter filetypes`);
console.log(`📝 Written to ${outputPath}`);

// Stats
const withNvimMatch = mergedMapping.filter(g => g.nvim_parser !== null);
const withoutNvimMatch = mergedMapping.filter(g => g.nvim_parser === null);

console.log("\n📊 Stats:");
console.log(`  With nvim-treesitter match: ${withNvimMatch.length}`);
console.log(`  Without nvim-treesitter match: ${withoutNvimMatch.length}`);

// Breakdown by match type
const byType: Record<string, number> = {};
withNvimMatch.forEach((m: any) => {
  byType[m.match_type] = (byType[m.match_type] || 0) + 1;
});
console.log("\n📋 Match types:");
for (const [type, count] of Object.entries(byType)) {
  console.log(`  ${type}: ${count}`);
}

// Show examples of matches
console.log("\n🔍 Matched examples:");
withNvimMatch.slice(0, 10).forEach(m => {
  const nameDiff = m.grammar !== m.nvim_parser ? ` (${m.nvim_parser})` : "";
  console.log(`  ${m.grammar}${nameDiff} → ${m.nvim_filetype}`);
});

// Show matches via repo name (different forks)
const repoNameMatches = withNvimMatch.filter((m: any) => m.match_type === "repo_name");
if (repoNameMatches.length > 0) {
  console.log(`\n🔄 Matched via repo name (different forks) (${repoNameMatches.length}):`);
  repoNameMatches.forEach(m => {
    console.log(`  ${m.grammar}:`);
    console.log(`    grammars.json: ${m.grammar_repo}`);
    console.log(`    nvim-treesitter: ${m.nvim_repo}`);
  });
}

// Show grammars without nvim match
if (withoutNvimMatch.length > 0) {
  console.log(`\n❌ Grammars without nvim-treesitter match (${withoutNvimMatch.length}):`);
  withoutNvimMatch.forEach(m => {
    console.log(`  - ${m.grammar} (${m.grammar_repo})`);
  });
}

// Show nvim parsers that didn't match any grammar
const matchedNvimNames = new Set(mergedMapping.filter((m: any) => m.nvim_parser).map((m: any) => m.nvim_parser));
const onlyInNvim = nvimParsers.filter(p => !matchedNvimNames.has(p.name));

if (onlyInNvim.length > 0) {
  console.log(`\n✅ nvim-treesitter parsers that didn't match any grammar (${onlyInNvim.length}):`);
  onlyInNvim.forEach(p => {
    const repoName = p.url.split("/").pop()?.replace(/\.git$/, "") || "";
    console.log(`  - ${p.name} (${p.url}) [repo: ${repoName}]`);
  });
}

// Debug: show any grammar with same name that didn't match
const grammarNames = new Set(grammars.map((g: { name: string }) => g.name));
const onlyInNvimThatHaveGrammarName = onlyInNvim.filter(p => grammarNames.has(p.name));
if (onlyInNvimThatHaveGrammarName.length > 0) {
  console.log(`\n⚠️  nvim parsers that share a name with a grammar but didn't match (${onlyInNvimThatHaveGrammarName.length}):`);
  onlyInNvimThatHaveGrammarName.forEach(p => {
    const grammar = grammars.find((g: { name: string }) => g.name === p.name);
    console.log(`  - nvim '${p.name}' (${p.url})`);
    console.log(`    vs grammar '${p.name}' (${grammar?.repo})`);
  });
}
