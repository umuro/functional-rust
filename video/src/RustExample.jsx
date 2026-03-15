import React from 'react';
import { useCurrentFrame, useVideoConfig, interpolate, spring, Easing } from 'remotion';

// ── Colour palette ────────────────────────────────────────────────────────────
const COLORS = {
  bg: '#0f1117',
  card: '#1a1d27',
  border: '#2d3148',
  accent: '#f74c00',       // Rust orange
  accentSoft: '#ff7c3a',
  text: '#e8eaf0',
  textDim: '#8b92a8',
  green: '#4ec994',
  blue: '#58a6ff',
  yellow: '#e3b341',
  purple: '#bc8cff',
  keyword: '#ff7b72',
  string_: '#a5d6ff',
  comment: '#6e7681',
  number: '#79c0ff',
  type_: '#ffa657',
};

// ── Tiny syntax highlighter ───────────────────────────────────────────────────
function tokenize(code) {
  const tokens = [];
  const patterns = [
    { type: 'comment',  re: /\/\/[^\n]*/g },
    { type: 'string_',  re: /"(?:[^"\\]|\\.)*"/g },
    { type: 'keyword',  re: /\b(fn|let|match|if|else|return|pub|use|struct|impl|enum|for|in|while|loop|break|continue|mut|ref|move|async|await|trait|where|type|const|static)\b/g },
    { type: 'type_',    re: /\b(Option|Result|Vec|String|str|i32|u32|i64|u64|usize|bool|Some|None|Ok|Err|Self|T)\b/g },
    { type: 'number',   re: /\b\d+\b/g },
  ];

  const parts = [{ start: 0, end: code.length, text: code, type: 'plain' }];

  function splitPart(partIdx, re, type) {
    const part = parts[partIdx];
    if (part.type !== 'plain') return;
    re.lastIndex = 0;
    let m;
    const newParts = [];
    let last = part.start;
    while ((m = re.exec(part.text)) !== null) {
      const abs = part.start + m.index;
      if (abs > last) newParts.push({ start: last, end: abs, text: code.slice(last, abs), type: 'plain' });
      newParts.push({ start: abs, end: abs + m[0].length, text: m[0], type });
      last = abs + m[0].length;
    }
    if (last < part.start + part.text.length)
      newParts.push({ start: last, end: part.start + part.text.length, text: code.slice(last, part.start + part.text.length), type: 'plain' });
    if (newParts.length) parts.splice(partIdx, 1, ...newParts);
  }

  // We just do a simple line-by-line approach instead:
  return code.split('\n').map(line => {
    const segs = [];
    let rest = line;
    let key = 0;

    // Comment
    const ci = rest.indexOf('//');
    if (ci !== -1) {
      const before = rest.slice(0, ci);
      highlightLine(before, segs, key);
      key += 100;
      segs.push({ text: rest.slice(ci), type: 'comment', key: key++ });
      return segs;
    }
    highlightLine(rest, segs, key);
    return segs;
  });
}

function highlightLine(text, segs, startKey) {
  const kw = /\b(fn|let|match|if|else|return|pub|use|struct|impl|enum|for|in|while|loop|break|continue|mut|ref|move|async|await|trait|where|type|const|static)\b/;
  const ty = /\b(Option|Result|Vec|String|str|i32|u32|i64|u64|usize|bool|Some|None|Ok|Err|Self|T)\b/;
  const str = /"(?:[^"\\]|\\.)*"/;
  const num = /\b\d+\b/;

  const combined = new RegExp(`(${str.source})|(${kw.source})|(${ty.source})|(${num.source})`, 'g');
  let last = 0;
  let m;
  let k = startKey;
  while ((m = combined.exec(text)) !== null) {
    if (m.index > last) segs.push({ text: text.slice(last, m.index), type: 'plain', key: k++ });
    let type = 'plain';
    if (m[1]) type = 'string_';
    else if (m[2]) type = 'keyword';
    else if (m[7]) type = 'type_';
    else if (m[8]) type = 'number';
    segs.push({ text: m[0], type, key: k++ });
    last = m.index + m[0].length;
  }
  if (last < text.length) segs.push({ text: text.slice(last), type: 'plain', key: k++ });
}

const tokenColor = (type) => ({
  comment: COLORS.comment,
  keyword: COLORS.keyword,
  string_: COLORS.string_,
  type_: COLORS.type_,
  number: COLORS.number,
  plain: COLORS.text,
}[type] || COLORS.text);

// ── Scene 1 — Intro ───────────────────────────────────────────────────────────
function IntroScene({ title, category, difficulty, fps }) {
  const frame = useCurrentFrame();

  const logoScale = spring({ frame, fps, config: { damping: 12, stiffness: 200 } });
  const titleY = interpolate(frame, [10, 30], [40, 0], { extrapolateRight: 'clamp', easing: Easing.out(Easing.cubic) });
  const titleOp = interpolate(frame, [10, 30], [0, 1], { extrapolateRight: 'clamp' });
  const subOp   = interpolate(frame, [20, 40], [0, 1], { extrapolateRight: 'clamp' });

  const stars = '⭐'.repeat(difficulty || 1);

  return (
    <div style={{ width: '100%', height: '100%', background: COLORS.bg, display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', fontFamily: "'JetBrains Mono', 'Fira Code', monospace" }}>
      {/* Background grid */}
      <div style={{ position: 'absolute', inset: 0, backgroundImage: `linear-gradient(${COLORS.border}22 1px, transparent 1px), linear-gradient(90deg, ${COLORS.border}22 1px, transparent 1px)`, backgroundSize: '60px 60px' }} />

      {/* Logo pill */}
      <div style={{ transform: `scale(${logoScale})`, background: COLORS.accent, borderRadius: 12, padding: '12px 28px', marginBottom: 48, zIndex: 1 }}>
        <span style={{ color: '#fff', fontSize: 28, fontWeight: 700, letterSpacing: 1 }}>🦀 Functional Rust</span>
      </div>

      {/* Title */}
      <div style={{ opacity: titleOp, transform: `translateY(${titleY}px)`, zIndex: 1, textAlign: 'center', maxWidth: 800, padding: '0 40px' }}>
        <div style={{ color: COLORS.text, fontSize: 52, fontWeight: 700, lineHeight: 1.2, marginBottom: 24 }}>{title}</div>
      </div>

      {/* Meta */}
      <div style={{ opacity: subOp, display: 'flex', gap: 16, zIndex: 1, marginTop: 16 }}>
        <span style={{ background: COLORS.card, border: `1px solid ${COLORS.border}`, borderRadius: 8, padding: '8px 20px', color: COLORS.textDim, fontSize: 22 }}>{category}</span>
        <span style={{ background: COLORS.card, border: `1px solid ${COLORS.border}`, borderRadius: 8, padding: '8px 20px', color: COLORS.yellow, fontSize: 22 }}>{stars}</span>
      </div>

      {/* hightechmind watermark */}
      <div style={{ position: 'absolute', bottom: 36, color: COLORS.textDim, fontSize: 20, letterSpacing: 1, zIndex: 1 }}>hightechmind.io/rust/</div>
    </div>
  );
}

// ── Scene 2 — Code ────────────────────────────────────────────────────────────
function CodeScene({ code, title, fps }) {
  const frame = useCurrentFrame();
  const lines = tokenize(code);
  const totalChars = code.length;

  // Reveal characters over time
  const revealed = interpolate(frame, [0, 60], [0, totalChars], { extrapolateRight: 'clamp', easing: Easing.out(Easing.quad) });

  // Header slide in
  const headerOp = interpolate(frame, [0, 15], [0, 1], { extrapolateRight: 'clamp' });

  let charCount = 0;

  return (
    <div style={{ width: '100%', height: '100%', background: COLORS.bg, display: 'flex', flexDirection: 'column', fontFamily: "'JetBrains Mono', 'Fira Code', monospace", padding: '40px 48px', boxSizing: 'border-box' }}>
      {/* Header */}
      <div style={{ opacity: headerOp, display: 'flex', alignItems: 'center', gap: 12, marginBottom: 32 }}>
        <div style={{ width: 12, height: 12, borderRadius: '50%', background: '#ff5f57' }} />
        <div style={{ width: 12, height: 12, borderRadius: '50%', background: '#febc2e' }} />
        <div style={{ width: 12, height: 12, borderRadius: '50%', background: '#28c840' }} />
        <span style={{ color: COLORS.textDim, fontSize: 20, marginLeft: 12 }}>{title.toLowerCase().replace(/ /g, '-')}.rs</span>
      </div>

      {/* Code block */}
      <div style={{ background: COLORS.card, border: `1px solid ${COLORS.border}`, borderRadius: 16, padding: '32px 36px', flex: 1, overflow: 'hidden' }}>
        {lines.map((lineSegs, li) => {
          const lineText = lineSegs.map(s => s.text).join('');
          return (
            <div key={li} style={{ display: 'flex', minHeight: 28, lineHeight: '28px' }}>
              {/* Line number */}
              <span style={{ color: COLORS.comment, fontSize: 18, minWidth: 36, userSelect: 'none', marginRight: 24 }}>{li + 1}</span>
              {/* Tokens */}
              {lineSegs.map((seg) => {
                const segStart = charCount;
                charCount += seg.text.length;
                const visible = Math.max(0, Math.min(seg.text.length, revealed - segStart));
                return (
                  <span key={seg.key} style={{ color: tokenColor(seg.type), fontSize: 20, whiteSpace: 'pre', opacity: visible > 0 ? 1 : 0 }}>
                    {seg.text.slice(0, visible)}
                  </span>
                );
              })}
            </div>
          );
          charCount += 1; // newline
        })}
      </div>
    </div>
  );
}

// ── Scene 3 — Output + CTA ────────────────────────────────────────────────────
function OutroScene({ title, output, fps }) {
  const frame = useCurrentFrame();

  const cardScale = spring({ frame, fps, config: { damping: 14, stiffness: 180 } });
  const ctaOp = interpolate(frame, [20, 40], [0, 1], { extrapolateRight: 'clamp' });
  const arrowX = interpolate(frame, [40, 60], [0, 8], { extrapolateLeft: 'clamp', extrapolateRight: 'mirror' });

  return (
    <div style={{ width: '100%', height: '100%', background: COLORS.bg, display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', fontFamily: "'JetBrains Mono', 'Fira Code', monospace", padding: '40px 48px', boxSizing: 'border-box' }}>
      {/* Output card */}
      <div style={{ transform: `scale(${cardScale})`, width: '100%', background: COLORS.card, border: `1px solid ${COLORS.border}`, borderRadius: 16, padding: '32px 36px', marginBottom: 40 }}>
        <div style={{ color: COLORS.textDim, fontSize: 18, marginBottom: 16, letterSpacing: 1 }}>▶ OUTPUT</div>
        <div style={{ color: COLORS.green, fontSize: 28, fontWeight: 600, whiteSpace: 'pre-wrap', lineHeight: 1.6 }}>{output}</div>
      </div>

      {/* CTA */}
      <div style={{ opacity: ctaOp, textAlign: 'center' }}>
        <div style={{ color: COLORS.textDim, fontSize: 22, marginBottom: 12 }}>More Functional Rust examples at</div>
        <div style={{ color: COLORS.accent, fontSize: 32, fontWeight: 700 }}>
          hightechmind.io/rust/
          <span style={{ display: 'inline-block', transform: `translateX(${arrowX}px)`, marginLeft: 8 }}>→</span>
        </div>
      </div>

      {/* Logo */}
      <div style={{ position: 'absolute', top: 36, right: 48, background: COLORS.accent, borderRadius: 8, padding: '8px 18px' }}>
        <span style={{ color: '#fff', fontSize: 20, fontWeight: 700 }}>🦀 hightechmind.io</span>
      </div>
    </div>
  );
}

// ── Root composition ──────────────────────────────────────────────────────────
function RustExampleVideo({ title, category, difficulty, code, output }) {
  const { fps, durationInFrames } = useVideoConfig();
  const frame = useCurrentFrame();

  const INTRO_END  = fps * 3;          // 0–3s
  const CODE_END   = fps * 10;         // 3–10s
  const OUTRO_END  = durationInFrames; // 10–15s

  let scene;
  if (frame < INTRO_END) {
    scene = <IntroScene title={title} category={category} difficulty={difficulty} fps={fps} />;
  } else if (frame < CODE_END) {
    scene = <CodeScene code={code} title={title} fps={fps} />;
  } else {
    scene = <OutroScene title={title} output={output} fps={fps} />;
  }

  return (
    <div style={{ width: 1080, height: 1080, overflow: 'hidden', position: 'relative' }}>
      {scene}
    </div>
  );
}

export { RustExampleVideo };
