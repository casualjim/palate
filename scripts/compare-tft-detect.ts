#!/usr/bin/env -S bun
/**
 * Compare detect mappings between this repo and tft (static + dynamic).
 *
 * Usage:
 *   bun scripts/compare-tft-detect.ts [tft_root] [palate_root] [--all] [--limit=N]
 *
 * Defaults:
 *   tft_root    = /home/ivan/github/rubixdev/tft
 *   palate_root = current working directory
 */

import { readFileSync } from "fs";
import { resolve } from "path";

type Diff = {
  key: string;
  palate: string;
  tft: string;
};

type Resolver =
  | { kind: "Static"; value: string }
  | { kind: "Dynamic"; value: string };

type CompareResult = {
  name: string;
  palateCount: number;
  tftCount: number;
  palateStatic: number;
  palateDynamic: number;
  tftStatic: number;
  tftDynamic: number;
  mismatched: Diff[];
  // Palate-only entries are usually expansions; keep for debugging but don't print by default.
  onlyPalate: string[];
  onlyTft: string[];
};

const args = process.argv.slice(2);
const tftRoot = args.find((a) => !a.startsWith("--")) ?? "/home/ivan/github/rubixdev/tft";
const palateRoot =
  args.filter((a) => !a.startsWith("--"))[1] ?? process.cwd();
const limitArg = args.find((a) => a.startsWith("--limit="));
const limit = limitArg ? Number(limitArg.split("=")[1]) : 50;
const showAll = args.includes("--all");

const targets = [
  { name: "filename", file: "src/detect/filename.rs", kind: "filename" },
  { name: "file_extension", file: "src/detect/file_extension.rs", kind: "extension" },
  { name: "path_suffix", file: "src/detect/path_suffix.rs", kind: "path_suffix" },
  { name: "pattern", file: "src/detect/pattern.rs", kind: "pattern" },
] as const;

function readFile(path: string): string {
  return readFileSync(path, "utf8");
}

function normalizeDynamicExpr(expr: string): string {
  // Keep comparisons stable across minor formatting differences.
  const collapsed = expr.trim().replace(/\s+/g, " ");
  // If this is a closure, keep it compact but still informative.
  if (collapsed.includes("|")) {
    const detects = Array.from(collapsed.matchAll(/detect::([A-Za-z0-9_]+)/g)).map((m) => m[1]);
    if (detects.length > 0) {
      return `closure(detect::${detects.join(",detect::")})`;
    }
    return "closure";
  }
  const direct = collapsed.match(/^detect::([A-Za-z0-9_]+)$/);
  if (direct) {
    return `detect::${direct[1]}`;
  }
  return collapsed;
}

function formatResolver(res: Resolver): string {
  if (res.kind === "Static") return `Static(${res.value})`;
  return `Dynamic(${res.value})`;
}

function extractResolverInner(line: string): { kind: "Static" | "Dynamic"; inner: string } | null {
  const sIdx = line.indexOf("FileTypeResolver::Static(");
  const dIdx = line.indexOf("FileTypeResolver::Dynamic(");
  let kind: "Static" | "Dynamic";
  let start: number;
  if (sIdx !== -1 && (dIdx === -1 || sIdx < dIdx)) {
    kind = "Static";
    start = sIdx + "FileTypeResolver::Static(".length;
  } else if (dIdx !== -1) {
    kind = "Dynamic";
    start = dIdx + "FileTypeResolver::Dynamic(".length;
  } else {
    return null;
  }

  // Extract until the matching ')' of the resolver call. This matters for patterns where
  // the resolver appears inside `Pattern::new(..., None)` and naive regex would over-capture.
  let depth = 1;
  let i = start;
  for (; i < line.length; i++) {
    const ch = line[i];
    if (ch === "(") depth += 1;
    else if (ch === ")") {
      depth -= 1;
      if (depth === 0) break;
    }
  }
  if (depth !== 0) return null;
  return { kind, inner: line.slice(start, i).trim() };
}

function parseResolverFromLine(line: string): Resolver | null {
  const extracted = extractResolverInner(line);
  if (!extracted) return null;
  const { kind } = extracted;
  const inner = extracted.inner.trim().replace(/,\s*$/, "");
  if (kind === "Static") {
    const ft = inner.match(/FileType::([A-Za-z0-9_]+)/);
    if (!ft) return null;
    return { kind: "Static", value: ft[1] };
  }
  return { kind: "Dynamic", value: normalizeDynamicExpr(inner) };
}

function parseMappings(content: string, kind: string): Map<string, Resolver> {
  const map = new Map<string, Resolver>();

  if (kind === "filename" || kind === "extension") {
    for (const line of content.split("\n")) {
      const key = line.match(/^\s*"([^"]+)"\s*=>/);
      if (!key) continue;
      const resolver = parseResolverFromLine(line);
      if (!resolver) continue;
      map.set(key[1], resolver);
    }
    return map;
  }

  if (kind === "path_suffix") {
    for (const line of content.split("\n")) {
      const key = line.match(/^\s*\("([^"]+)",\s*FileTypeResolver::/);
      if (!key) continue;
      const resolver = parseResolverFromLine(line);
      if (!resolver) continue;
      map.set(key[1], resolver);
    }
    return map;
  }

  if (kind === "pattern") {
    for (const line of content.split("\n")) {
      if (!line.includes("regex!(")) continue;
      const regexMatch = line.match(/regex!\((?:r)?\"([^\"]+)\"/);
      if (!regexMatch) continue;
      const resolver = parseResolverFromLine(line);
      if (!resolver) continue;
      map.set(regexMatch[1], resolver);
    }
    return map;
  }

  return map;
}

function compareMaps(
  name: string,
  palate: Map<string, Resolver>,
  tft: Map<string, Resolver>
): CompareResult {
  const mismatched: Diff[] = [];
  const onlyPalate: string[] = [];
  const onlyTft: string[] = [];

  let palateStatic = 0;
  let palateDynamic = 0;
  let tftStatic = 0;
  let tftDynamic = 0;

  for (const res of palate.values()) {
    if (res.kind === "Static") palateStatic += 1;
    else palateDynamic += 1;
  }
  for (const res of tft.values()) {
    if (res.kind === "Static") tftStatic += 1;
    else tftDynamic += 1;
  }

  for (const [key, pRes] of palate.entries()) {
    const tRes = tft.get(key);
    if (!tRes) {
      onlyPalate.push(`${key}: ${formatResolver(pRes)}`);
      continue;
    }
    if (pRes.kind !== tRes.kind || pRes.value !== tRes.value) {
      mismatched.push({ key, palate: formatResolver(pRes), tft: formatResolver(tRes) });
    }
  }

  for (const [key, tRes] of tft.entries()) {
    if (!palate.has(key)) {
      onlyTft.push(`${key}: ${formatResolver(tRes)}`);
    }
  }

  mismatched.sort((a, b) => a.key.localeCompare(b.key));
  onlyPalate.sort();
  onlyTft.sort();

  return {
    name,
    palateCount: palate.size,
    tftCount: tft.size,
    palateStatic,
    palateDynamic,
    tftStatic,
    tftDynamic,
    mismatched,
    onlyPalate,
    onlyTft,
  };
}

const results: CompareResult[] = [];

for (const target of targets) {
  const palatePath = resolve(palateRoot, target.file);
  const tftPath = resolve(tftRoot, target.file);

  let palateContent = "";
  let tftContent = "";
  try {
    palateContent = readFile(palatePath);
  } catch {
    console.error(`Missing palate file: ${palatePath}`);
    continue;
  }
  try {
    tftContent = readFile(tftPath);
  } catch {
    console.error(`Missing tft file: ${tftPath}`);
    continue;
  }

  const palateMap = parseMappings(palateContent, target.kind);
  const tftMap = parseMappings(tftContent, target.kind);
  results.push(compareMaps(target.name, palateMap, tftMap));
}

let totalMismatched = 0;
let totalOnlyPalate = 0;
let totalOnlyTft = 0;

for (const result of results) {
  totalMismatched += result.mismatched.length;
  totalOnlyPalate += result.onlyPalate.length;
  totalOnlyTft += result.onlyTft.length;
}

console.log("Detect mapping comparison (static + dynamic)");
console.log(`tft root: ${tftRoot}`);
console.log(`palate root: ${palateRoot}`);
console.log("");

for (const result of results) {
  console.log(
    `== ${result.name}: palate=${result.palateCount} (static=${result.palateStatic}, dynamic=${result.palateDynamic}), tft=${result.tftCount} (static=${result.tftStatic}, dynamic=${result.tftDynamic}), mismatched=${result.mismatched.length}, only-palate=${result.onlyPalate.length}, only-tft=${result.onlyTft.length}`
  );

  const show = showAll ? result.mismatched.length : Math.min(limit, result.mismatched.length);
  if (show > 0) {
    console.log("  mismatched:");
    for (const diff of result.mismatched.slice(0, show)) {
      console.log(`    ${diff.key}: palate=${diff.palate} tft=${diff.tft}`);
    }
    if (!showAll && result.mismatched.length > show) {
      console.log(`    ... (${result.mismatched.length - show} more)`);
    }
  }

  if (showAll) {
    const showOnlyPalate = result.onlyPalate.length;
    if (showOnlyPalate > 0) {
      console.log("  only in palate:");
      for (const key of result.onlyPalate) {
        console.log(`    ${key}`);
      }
    }
  }

  const showOnlyTft = showAll ? result.onlyTft.length : Math.min(limit, result.onlyTft.length);
  if (showOnlyTft > 0) {
    console.log("  only in tft:");
    for (const key of result.onlyTft.slice(0, showOnlyTft)) {
      console.log(`    ${key}`);
    }
    if (!showAll && result.onlyTft.length > showOnlyTft) {
      console.log(`    ... (${result.onlyTft.length - showOnlyTft} more)`);
    }
  }

  console.log("");
}

console.log(
  `Totals: mismatched=${totalMismatched}, only-palate=${totalOnlyPalate}, only-tft=${totalOnlyTft}`
);
