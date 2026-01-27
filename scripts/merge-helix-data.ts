#!/usr/bin/env -S bun
/**
 * Merge Helix's languages.toml data with grammars-mapping.json
 * Adds file-types and shebangs from Helix's comprehensive language definitions
 */

const helixUrl = "https://raw.githubusercontent.com/helix-editor/helix/master/languages.toml";
const grammarsMappingPath = "/home/ivan/github/casualjim/palate/target/grammars-mapping.json";
const grammarsJsonPath = "/home/ivan/github/casualjim/breeze-tree-sitter-parsers/grammars.json";
const tempTomlPath = "/home/ivan/github/casualjim/palate/target/helix-languages.toml";
const outputPath = "/home/ivan/github/casualjim/palate/target/grammars-mapping-enhanced.json";

// Fetch Helix's languages.toml and save to temp file
console.log("Fetching Helix languages.toml...");
const helixResponse = await fetch(helixUrl);
const helixContent = await helixResponse.text();
await Bun.write(tempTomlPath, helixContent);

// Import TOML using Bun's built-in support
const helixData = await import(tempTomlPath);

// Load existing data
const grammarsMapping = JSON.parse(await Bun.file(grammarsMappingPath).text());
const grammarsData = JSON.parse(await Bun.file(grammarsJsonPath).text());

console.log(`Loaded ${grammarsMapping.length} existing mappings`);
console.log(`Loaded ${grammarsData.grammars.length} grammars`);
console.log(`Parsed ${helixData.language?.length || 0} languages from Helix`);

// Helper: extract owner/repo from GitHub URL
function getRepoId(url: string): string {
  const match = url.match(/github\.com[\/:]([^\/]+\/[^\/\.]+)/);
  return match ? match[1] : "";
}

// Build maps for matching
const helixByLanguageName: Record<string, any> = {};
if (helixData.language) {
  for (const lang of helixData.language) {
    helixByLanguageName[lang.name] = lang;
  }
}

const helixByRepoUrl: Record<string, any> = {};
if (helixData.grammar) {
  for (const grammar of helixData.grammar) {
    const repoUrl = grammar.source?.git;
    if (repoUrl) {
      helixByRepoUrl[repoUrl] = { grammar, language: helixByLanguageName[grammar.name] };
    }
  }
}

// Build enhanced mapping
const enhancedMapping = grammarsMapping.map((entry: any) => {
  const result = { ...entry };

  // Try to match by grammar name first
  const helixLangByName = helixByLanguageName[entry.grammar];
  if (helixLangByName) {
    if (helixLangByName["file-types"]?.length > 0) {
      result.helix_file_types = helixLangByName["file-types"];
    }
    if (helixLangByName.shebangs?.length > 0) {
      result.helix_shebangs = helixLangByName.shebangs;
    }
    result.helix_language_name = helixLangByName.name;
  }

  // Try to match by nvim parser name
  if (entry.nvim_parser) {
    const helixLangByParser = helixByLanguageName[entry.nvim_parser];
    if (helixLangByParser && !result.helix_file_types) {
      if (helixLangByParser["file-types"]?.length > 0) {
        result.helix_file_types = helixLangByParser["file-types"];
      }
      if (helixLangByParser.shebangs?.length > 0) {
        result.helix_shebangs = helixLangByParser.shebangs;
      }
      if (!result.helix_language_name) {
        result.helix_language_name = helixLangByParser.name;
      }
    }
  }

  // Try to match by repository URL
  const grammarRepoId = getRepoId(entry.grammar_repo);
  for (const [gitUrl, helixData] of Object.entries(helixByRepoUrl)) {
    const helixRepoId = getRepoId(gitUrl);
    if (helixRepoId && grammarRepoId === helixRepoId) {
      const lang = helixData.language;
      if (lang) {
        if (!result.helix_file_types && lang["file-types"]?.length > 0) {
          result.helix_file_types = lang["file-types"];
        }
        if (!result.helix_shebangs && lang.shebangs?.length > 0) {
          result.helix_shebangs = lang.shebangs;
        }
        if (!result.helix_language_name) {
          result.helix_language_name = lang.name;
        }
      }
    }
  }

  return result;
});

// Write output
await Bun.write(outputPath, JSON.stringify(enhancedMapping, null, 2));

console.log(`\n✅ Enhanced ${enhancedMapping.length} mappings with Helix data`);

// Stats
const withFileTypes = enhancedMapping.filter((m: any) => m.helix_file_types?.length > 0);
const withShebangs = enhancedMapping.filter((m: any) => m.helix_shebangs?.length > 0);
const withoutHelix = enhancedMapping.filter((m: any) => !m.helix_file_types && !m.helix_shebangs);

console.log(`\n📊 Stats:`);
console.log(`  With Helix file-types: ${withFileTypes.length}`);
console.log(`  With Helix shebangs: ${withShebangs.length}`);
console.log(`  Without Helix data: ${withoutHelix.length}`);

// Show examples
console.log(`\n🔍 Examples with file-types:`);
withFileTypes.slice(0, 10).forEach((m: any) => {
  console.log(`  ${m.grammar}: ${m.helix_file_types?.slice(0, 5).join(", ")}${m.helix_file_types?.length > 5 ? "..." : ""}`);
});

console.log(`\n🔍 Examples with shebangs:`);
withShebangs.slice(0, 10).forEach((m: any) => {
  console.log(`  ${m.grammar}: ${m.helix_shebangs.join(", ")}`);
});

console.log(`\n📋 Without Helix data (first 20):`);
withoutHelix.slice(0, 20).forEach((m: any) => {
  console.log(`  - ${m.grammar}`);
});
