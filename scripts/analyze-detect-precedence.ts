#!/usr/bin/env -S bun
/**
 * Analyze detect precedence/overlaps. This is meant to catch:
 * - "dead" PATH_SUFFIX entries (globs/brace patterns that can never match Path::ends_with)
 * - PATTERN phase differences (pre-extension vs post-extension) between palate and tft
 * - duplicate patterns with conflicting resolvers
 *
 * Usage:
 *   bun scripts/analyze-detect-precedence.ts [tft_root] [palate_root] [--all] [--limit=N]
 *
 * Defaults:
 *   tft_root    = /home/ivan/github/rubixdev/tft   (optional; if missing/unreadable we only analyze palate)
 *   palate_root = current working directory
 */

import { readFileSync, existsSync } from "fs";
import { resolve } from "path";

type Resolver =
  | { kind: "Static"; value: string }
  | { kind: "Dynamic"; value: string };

type PatternEntry = {
  // Index in the file (source) order.
  index: number;
  // Index in the runtime order after the `vec.sort_unstable_by(...)` in pattern.rs.
  runtimeIndex: number;
  matchFullPath: boolean;
  regex: string;
  resolver: Resolver;
  // priority as written in pattern.rs (None | Some(n)); starsetf(None) becomes negative at runtime.
  priorityRaw: { kind: "None" } | { kind: "Some"; value: number };
  ctor: "new" | "starsetf";
  phase: "pre" | "post";
};

type Parsed = {
  pathSuffix: Map<string, Resolver>;
  filename: Map<string, Resolver>;
  extension: Map<string, Resolver>;
  patterns: PatternEntry[];
  patternSplitIndex: number;
};

const args = process.argv.slice(2);
const tftRoot = args.find((a) => !a.startsWith("--")) ?? "/home/ivan/github/rubixdev/tft";
const palateRoot = args.filter((a) => !a.startsWith("--"))[1] ?? process.cwd();
const showAll = args.includes("--all");
const limitArg = args.find((a) => a.startsWith("--limit="));
const limit = limitArg ? Number(limitArg.split("=")[1]) : 50;

function readFile(path: string): string {
  return readFileSync(path, "utf8");
}

function normalizeDynamicExpr(expr: string): string {
  const collapsed = expr.trim().replace(/\s+/g, " ");
  if (collapsed.includes("|")) {
    const detects = Array.from(collapsed.matchAll(/detect::([A-Za-z0-9_]+)/g)).map((m) => m[1]);
    if (detects.length > 0) {
      return `closure(detect::${detects.join(",detect::")})`;
    }
    return "closure";
  }
  const direct = collapsed.match(/^detect::([A-Za-z0-9_]+)$/);
  if (direct) return `detect::${direct[1]}`;
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

function parseSimpleMap(content: string, kind: "filename" | "extension" | "path_suffix"): Map<string, Resolver> {
  const map = new Map<string, Resolver>();
  for (const line of content.split("\n")) {
    let key: string | null = null;
    if (kind === "filename" || kind === "extension") {
      const m = line.match(/^\s*"([^"]+)"\s*=>/);
      if (m) key = m[1];
    } else {
      const m = line.match(/^\s*\("([^"]+)",\s*FileTypeResolver::/);
      if (m) key = m[1];
    }
    if (!key) continue;
    const resolver = parseResolverFromLine(line);
    if (!resolver) continue;
    map.set(key, resolver);
  }
  return map;
}

function extractBalancedCallArgs(line: string, callStart: number): string | null {
  // callStart points to the character after the opening '(' of a call.
  let depth = 1;
  let i = callStart;
  for (; i < line.length; i++) {
    const ch = line[i];
    if (ch === "(") depth += 1;
    else if (ch === ")") {
      depth -= 1;
      if (depth === 0) break;
    }
  }
  if (depth !== 0) return null;
  return line.slice(callStart, i);
}

function splitTopLevelArgs(argStr: string): string[] {
  const out: string[] = [];
  let depth = 0;
  let start = 0;
  for (let i = 0; i < argStr.length; i++) {
    const ch = argStr[i];
    if (ch === "(") depth += 1;
    else if (ch === ")") depth -= 1;
    else if (ch === "," && depth === 0) {
      out.push(argStr.slice(start, i).trim());
      start = i + 1;
    }
  }
  out.push(argStr.slice(start).trim());
  return out.filter((s) => s.length > 0);
}

function parsePatternPriority(line: string): { ctor: "new" | "starsetf"; priorityRaw: PatternEntry["priorityRaw"] } | null {
  const newIdx = line.indexOf("Pattern::new(");
  const starIdx = line.indexOf("Pattern::starsetf(");
  let ctor: "new" | "starsetf";
  let start: number;
  if (newIdx !== -1 && (starIdx === -1 || newIdx < starIdx)) {
    ctor = "new";
    start = newIdx + "Pattern::new(".length;
  } else if (starIdx !== -1) {
    ctor = "starsetf";
    start = starIdx + "Pattern::starsetf(".length;
  } else {
    return null;
  }
  const args = extractBalancedCallArgs(line, start);
  if (!args) return null;
  const parts = splitTopLevelArgs(args);
  const last = parts[parts.length - 1]?.trim();
  if (!last) return null;
  if (last === "None") return { ctor, priorityRaw: { kind: "None" } };
  const m = last.match(/^Some\((-?\d+)\)$/);
  if (m) return { ctor, priorityRaw: { kind: "Some", value: Number(m[1]) } };
  // Unexpected, but keep it conservative.
  return null;
}

function runtimePriorityValue(ctor: "new" | "starsetf", priorityRaw: PatternEntry["priorityRaw"]): number {
  if (priorityRaw.kind === "Some") return priorityRaw.value;
  // Pattern::new(..., None) => None which sorts as 0 in `unwrap_or(0)`
  if (ctor === "new") return 0;
  // Pattern::starsetf(..., None) => Some(isize::MIN). We only need relative ordering for analysis.
  return Number.MIN_SAFE_INTEGER;
}

function isRuntimeNegativePriority(ctor: "new" | "starsetf", priorityRaw: PatternEntry["priorityRaw"]): boolean {
  // Mirrors tft/palate detect pipeline boundary: `pat.priority.map_or(false, |prio| prio < 0)`
  // Pattern::new(..., None) => no (None)
  if (ctor === "new" && priorityRaw.kind === "None") return false;
  // Everything else is Some(prio) where prio is either explicit or MIN for starsetf(None)
  return runtimePriorityValue(ctor, priorityRaw) < 0;
}

function parsePatterns(content: string): { patterns: Omit<PatternEntry, "phase">[]; splitIndex: number } {
  const patterns: Omit<PatternEntry, "phase" | "runtimeIndex">[] = [];
  for (const line of content.split("\n")) {
    if (!line.includes("regex!(")) continue;
    const mfp = line.match(/^\s*\((true|false),/);
    const rm = line.match(/regex!\(r\"([^\"]+)\"/);
    if (!mfp || !rm) continue;
    const resolver = parseResolverFromLine(line);
    if (!resolver) continue;
    const pr = parsePatternPriority(line);
    if (!pr) continue;
    patterns.push({
      index: patterns.length,
      matchFullPath: mfp[1] === "true",
      regex: rm[1],
      resolver,
      priorityRaw: pr.priorityRaw,
      ctor: pr.ctor,
    });
  }

  // pattern.rs sorts by priority at runtime. Pre/post phase is determined *after sorting*,
  // so we must compute split index based on that runtime order, not the source order.
  const runtimeOrder = patterns
    .map((p) => ({
      ...p,
      // stable tie-breaker: keep source order for equal priorities (sort_unstable isn't stable,
      // but this is good enough for diagnostics).
      _prio: runtimePriorityValue(p.ctor, p.priorityRaw),
    }))
    .sort((a, b) => {
      if (a._prio !== b._prio) return b._prio - a._prio;
      return a.index - b.index;
    });

  let splitIndex = runtimeOrder.length;
  for (let i = 0; i < runtimeOrder.length; i++) {
    const p = runtimeOrder[i];
    if (isRuntimeNegativePriority(p.ctor, p.priorityRaw)) {
      splitIndex = i;
      break;
    }
  }

  // Assign runtimeIndex according to computed runtime order.
  const runtimeIndexBySourceIndex = new Map<number, number>();
  for (let i = 0; i < runtimeOrder.length; i++) {
    runtimeIndexBySourceIndex.set(runtimeOrder[i].index, i);
  }

  const withRuntimeIndex: Omit<PatternEntry, "phase">[] = patterns.map((p) => ({
    ...p,
    runtimeIndex: runtimeIndexBySourceIndex.get(p.index) ?? p.index,
  }));

  return { patterns: withRuntimeIndex, splitIndex };
}

function parseDetectRoot(root: string): Parsed | null {
  const base = (p: string) => resolve(root, p);
  const files = {
    pathSuffix: base("src/detect/path_suffix.rs"),
    filename: base("src/detect/filename.rs"),
    extension: base("src/detect/file_extension.rs"),
    pattern: base("src/detect/pattern.rs"),
  };
  for (const p of Object.values(files)) {
    if (!existsSync(p)) return null;
  }

  const pathSuffix = parseSimpleMap(readFile(files.pathSuffix), "path_suffix");
  const filename = parseSimpleMap(readFile(files.filename), "filename");
  const extension = parseSimpleMap(readFile(files.extension), "extension");
  const patParsed = parsePatterns(readFile(files.pattern));

  const patterns: PatternEntry[] = patParsed.patterns.map((p) => ({
    ...p,
    phase: p.runtimeIndex < patParsed.splitIndex ? "pre" : "post",
  }));

  return {
    pathSuffix,
    filename,
    extension,
    patterns,
    patternSplitIndex: patParsed.splitIndex,
  };
}

function hasGlobMeta(s: string): boolean {
  // Things that almost certainly indicate this isn't a literal suffix for Path::ends_with.
  return /[*?{}\[\]]/.test(s) || s.includes("**");
}

function printLimited(title: string, items: string[]): void {
  const show = showAll ? items.length : Math.min(limit, items.length);
  if (show === 0) return;
  console.log(title);
  for (const it of items.slice(0, show)) console.log(`  ${it}`);
  if (!showAll && items.length > show) console.log(`  ... (${items.length - show} more)`);
}

const palate = parseDetectRoot(palateRoot);
if (!palate) {
  console.error(`Could not read detect files from: ${palateRoot}`);
  process.exit(1);
}

const tft = parseDetectRoot(tftRoot);

console.log("Detect precedence analysis");
console.log(`palate root: ${palateRoot}`);
console.log(`tft root: ${tft ? tftRoot : "(not found; skipped)"}`);
console.log("");

// 1) PATH_SUFFIX suspicious entries (these are effectively dead unless the filename literally contains '*' etc).
const suspiciousSuffix = Array.from(palate.pathSuffix.keys())
  .filter(hasGlobMeta)
  .sort();
console.log(
  `path_suffix: total=${palate.pathSuffix.size}, suspicious(glob/brace/etc)=${suspiciousSuffix.length}`
);
printLimited("suspicious path_suffix entries:", suspiciousSuffix);
console.log("");

// 2) Pattern split (what actually runs before extension vs after).
console.log(
  `pattern: total=${palate.patterns.length}, split_index=${palate.patternSplitIndex} (pre=${palate.patternSplitIndex}, post=${palate.patterns.length - palate.patternSplitIndex})`
);
if (tft) {
  console.log(
    `tft pattern: total=${tft.patterns.length}, split_index=${tft.patternSplitIndex} (pre=${tft.patternSplitIndex}, post=${tft.patterns.length - tft.patternSplitIndex})`
  );
}
console.log("");

// 3) Duplicate patterns (exact same regex+match_full_path) with conflicting resolvers.
type DupeKey = string;
function dupeKey(p: PatternEntry): DupeKey {
  return `${p.matchFullPath ? "full" : "name"}::${p.regex}`;
}
const dupes = new Map<DupeKey, PatternEntry[]>();
for (const p of palate.patterns) {
  const k = dupeKey(p);
  const arr = dupes.get(k) ?? [];
  arr.push(p);
  dupes.set(k, arr);
}
const conflictingDupes: string[] = [];
for (const [k, ps] of dupes.entries()) {
  if (ps.length < 2) continue;
  const uniq = new Set(ps.map((p) => formatResolver(p.resolver)));
  if (uniq.size > 1) {
    const parts = ps
      .map((p) => `${p.index}:${p.phase}:${formatResolver(p.resolver)}`)
      .join(" | ");
    conflictingDupes.push(`${k} -> ${parts}`);
  }
}
conflictingDupes.sort();
console.log(`pattern: duplicate keys with conflicting resolvers=${conflictingDupes.length}`);
printLimited("conflicting duplicates:", conflictingDupes);
console.log("");

// 4) Phase differences vs tft for same regex key (this is the big precedence gotcha).
if (tft) {
  const tftByKey = new Map<DupeKey, PatternEntry>();
  for (const p of tft.patterns) {
    // Keep the first occurrence (earliest precedence) for comparisons.
    const k = dupeKey(p);
    if (!tftByKey.has(k)) tftByKey.set(k, p);
  }

  const phaseDiff: string[] = [];
  const resolverDiff: string[] = [];

  for (const p of palate.patterns) {
    const k = dupeKey(p);
    const tp = tftByKey.get(k);
    if (!tp) continue;
    if (p.phase !== tp.phase) {
      phaseDiff.push(
        `${k} palate=${p.runtimeIndex}:${p.phase} tft=${tp.runtimeIndex}:${tp.phase} resolver(palate)=${formatResolver(p.resolver)} resolver(tft)=${formatResolver(tp.resolver)}`
      );
    } else if (
      p.resolver.kind !== tp.resolver.kind ||
      p.resolver.value !== tp.resolver.value
    ) {
      resolverDiff.push(
        `${k} phase=${p.phase} palate=${formatResolver(p.resolver)} tft=${formatResolver(tp.resolver)}`
      );
    }
  }

  phaseDiff.sort();
  resolverDiff.sort();

  console.log(`pattern vs tft: phase differences=${phaseDiff.length}`);
  printLimited("phase diffs:", phaseDiff);
  console.log("");

  console.log(`pattern vs tft: resolver differences (same phase)=${resolverDiff.length}`);
  printLimited("resolver diffs:", resolverDiff);
  console.log("");
}
