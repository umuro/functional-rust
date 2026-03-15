#!/usr/bin/env node
/**
 * Render a Functional Rust LinkedIn video
 * Usage:
 *   node render.js 001            → render example #001
 *   node render.js random         → random example from library
 *   node render.js daily          → today's example (deterministic from date)
 *   node render.js list           → list available examples
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const EXAMPLES_DIR = path.resolve(__dirname, '../examples');
const REMOTION = path.resolve(__dirname, 'node_modules/.bin/remotion');

// ── Parse example metadata ────────────────────────────────────────────────────
function parseExample(dir) {
  const readme = fs.readFileSync(path.join(EXAMPLES_DIR, dir, 'README.md'), 'utf8');
  const rs     = fs.readFileSync(path.join(EXAMPLES_DIR, dir, 'example.rs'), 'utf8');

  // Title
  const titleMatch = readme.match(/^#\s+\d+\s+[—–-]+\s+(.+)$/m) || readme.match(/^#\s+(.+)$/m);
  const title = titleMatch ? titleMatch[1].trim() : dir;

  // Category
  const catMatch = readme.match(/\*\*Category\*\*[:\s]+(.+)/);
  const category = catMatch ? catMatch[1].trim() : 'Functional Rust';

  // Difficulty (count ⭐)
  const diffMatch = readme.match(/\*\*Difficulty\*\*[:\s]+(⭐+)/);
  const difficulty = diffMatch ? diffMatch[1].length : 1;

  // Code — first fn block (up to ~20 lines for readability)
  const codeLines = rs.split('\n');
  const fnStart = codeLines.findIndex(l => l.match(/^(pub )?fn /));
  const code = fnStart >= 0
    ? codeLines.slice(Math.max(0, fnStart - 2), fnStart + 20).join('\n')
    : codeLines.slice(0, 20).join('\n');

  // Output — extract from problem statement examples or test assertions
  let output = '';
  const exBlock = readme.match(/```\n([\s\S]+?)\n```/);
  if (exBlock) {
    output = exBlock[1].trim().split('\n').slice(0, 4).join('\n');
  } else {
    // Try to extract from test assertions
    const asserts = [...rs.matchAll(/assert_eq!\(([^)]+)\)/g)].slice(0, 3);
    output = asserts.map(m => m[1].replace(/\s+/g, ' ')).join('\n');
  }

  return { title, category, difficulty, code, output };
}

// ── List available examples ───────────────────────────────────────────────────
function listExamples() {
  return fs.readdirSync(EXAMPLES_DIR)
    .filter(d => fs.statSync(path.join(EXAMPLES_DIR, d)).isDirectory())
    .filter(d => fs.existsSync(path.join(EXAMPLES_DIR, d, 'example.rs')))
    .sort();
}

// ── Select example ────────────────────────────────────────────────────────────
function selectExample(arg) {
  const all = listExamples();
  if (!arg || arg === 'random') {
    return all[Math.floor(Math.random() * all.length)];
  }
  if (arg === 'daily') {
    const day = Math.floor(Date.now() / 86400000);
    return all[day % all.length];
  }
  if (arg === 'list') {
    all.forEach(d => console.log(d));
    process.exit(0);
  }
  // Find by prefix match
  const match = all.find(d => d.startsWith(arg.padStart(3, '0')) || d.startsWith(arg));
  if (!match) { console.error(`Example "${arg}" not found`); process.exit(1); }
  return match;
}

// ── Main ──────────────────────────────────────────────────────────────────────
const arg = process.argv[2] || 'random';
const exDir = selectExample(arg);
const meta = parseExample(exDir);

console.log(`\n🦀 Rendering: ${meta.title}`);
console.log(`   Category:   ${meta.category}`);
console.log(`   Difficulty: ${'⭐'.repeat(meta.difficulty)}`);
console.log(`   Code lines: ${meta.code.split('\n').length}`);
console.log(`   Output:     ${meta.output.split('\n')[0].slice(0, 60)}`);

// Output goes into the example directory itself
const outFile = path.join(EXAMPLES_DIR, exDir, 'video.mp4');
const props = JSON.stringify(meta);

console.log(`\n⏳ Rendering 1080×1080 @ 30fps (15s) …`);

try {
  execSync(
    `${REMOTION} render src/index.jsx RustExample "${outFile}" --props='${props.replace(/'/g, "'\\''")}' --frames=0-449 --concurrency=2`,
    { stdio: 'inherit', cwd: __dirname }
  );
  console.log(`\n✅ Done: ${outFile}`);
  console.log(`   Size: ${(fs.statSync(outFile).size / 1024 / 1024).toFixed(1)} MB`);
} catch (e) {
  console.error('Render failed:', e.message);
  process.exit(1);
}
