// Neomil desktop design mock-ups. One function per scene, drawn as SVG from
// the era's measured vocabulary (cp-eras-ui docs/neomil/line-weights.md).
// Units are design px (dp): the traces' 1600x900 frame, 2.4 native px per dp
// at 3840x2160. Nothing here is traced from reference pixels.
(function () {
  const INK = {
    bg: '#050304', glow: '#131f52', fg: '#ef3333', bright: '#fb3535',
    border: '#5e1112', dim: '#a32226', rest: '#2e1012', ghost: '#1a0607',
    lit: '#551719', tape: '#dedede', badgeEdge: '#701d39', badgeFace: '#2a0d16',
    tabEdge: '#68232e', tabFace: '#171a3c', onRed: '#19090a', win: '#0a0506',
  };
  // The two fittings. Everything a target changes is in this table.
  const TARGET = {
    desktop: { u: 1, line: 0.8333, hairline: 0.7, heavy: 1.5, margin: 40, gapOut: 10, gapIn: 6, focus: 0.8333 },
    tv: { u: 1.5, line: 1.25, hairline: 1.05, heavy: 2.25, margin: 80, gapOut: 16, gapIn: 10, focus: 2.25 },
  };
  const W = 1600, H = 900;
  const esc = s => String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;');
  const n = v => Math.round(v * 100) / 100;

  // Rectangle with per-corner chamfers, as a path.
  function plate(x, y, w, h, c = {}) {
    const { tl = 0, tr = 0, br = 0, bl = 0 } = c;
    return `M${n(x + tl)} ${n(y)}H${n(x + w - tr)}L${n(x + w)} ${n(y + tr)}V${n(y + h - br)}L${n(x + w - br)} ${n(y + h)}H${n(x + bl)}L${n(x)} ${n(y + h - bl)}V${n(y + tl)}Z`;
  }
  const path = (d, a) => `<path d="${d}" ${a}/>`;
  function text(x, y, s, o = {}) {
    const { size = 10, fill = INK.fg, font = 'ui', anchor = 'start', weight = 500, ls = 0.04, cls = '', op = 1 } = o;
    return `<text x="${n(x)}" y="${n(y)}" class="f-${font} ${cls}" font-size="${n(size)}" fill="${fill}" text-anchor="${anchor}" font-weight="${weight}" letter-spacing="${ls}em" opacity="${op}">${esc(s)}</text>`;
  }
  // The era's printed echo: one or two faint copies of an outline, offset.
  function echo(d, T, dx = 1, dy = 0, ink = INK.bright) {
    const o = 2.5 * T.u;
    return `<g class="echo">${path(d, `fill="none" stroke="${ink}" stroke-width="${T.line}" opacity=".2" transform="translate(${n(dx * o)} ${n(dy * o)})"`)}${path(d, `fill="none" stroke="${ink}" stroke-width="${T.line}" opacity=".09" transform="translate(${n(dx * o * 2)} ${n(dy * o * 2)})"`)}</g>`;
  }
  function ground(id) {
    return `<defs><linearGradient id="g${id}" x1="0" y1="0" x2="0.25" y2="1"><stop offset="0" stop-color="${INK.glow}"/><stop offset=".42" stop-color="#0a0a1c"/><stop offset=".7" stop-color="${INK.bg}"/></linearGradient>
<linearGradient id="v${id}" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="${INK.bg}" stop-opacity=".85"/><stop offset=".12" stop-color="${INK.bg}" stop-opacity="0"/><stop offset=".9" stop-color="${INK.bg}" stop-opacity="0"/><stop offset="1" stop-color="${INK.bg}" stop-opacity=".8"/></linearGradient></defs>
<rect width="${W}" height="${H}" fill="${INK.bg}"/><rect width="${W}" height="${H * 0.62}" fill="url(#g${id})"/><rect width="${W}" height="${H}" fill="url(#v${id})"/>`;
  }

  // A chamfered badge cell: the header's LEVEL badge, used for workspaces
  // and status. on = the solid red (selected) form.
  function badge(x, y, w, h, T, { cap, val, on = false, cls = '' }) {
    const d = plate(x, y, w, h, { bl: 6 * T.u });
    const ink = on ? INK.onRed : INK.fg;
    let s = `<g class="badge ${cls}">` + path(d, `fill="${on ? INK.fg : INK.badgeFace}" stroke="${on ? INK.bright : INK.badgeEdge}" stroke-width="${T.line}"`);
    if (cap) s += text(x + w / 2, y + h * 0.36, cap, { size: h * 0.2, fill: ink, anchor: 'middle', ls: 0.12, op: on ? 0.9 : 0.75 });
    s += text(x + w / 2, y + (cap ? h * 0.8 : h * 0.72), val, { size: cap ? h * 0.42 : h * 0.52, fill: ink, anchor: 'middle', font: 'wide', weight: 600 });
    return s + '</g>';
  }
  function barcode(x, y, w, h, ink) {
    let s = '', cx = x, i = 0; const pat = [2, 1, 1, 3, 1, 2, 2, 1, 3, 1, 1, 2, 1, 1, 3, 2, 1, 2];
    while (cx < x + w) { const bw = pat[i % pat.length] * (h / 14); if (i % 2 === 0) s += `<rect x="${n(cx)}" y="${n(y)}" width="${n(bw)}" height="${n(h)}" fill="${ink}"/>`; cx += bw; i++; }
    return s;
  }

  // The top bar. kind 'reference' keeps the art's header: captions over a
  // badge row, the tab plate, the full-width rule. 'slim' packs the same
  // parts into a normal bar height.
  function bar(T, o, title) {
    const u = T.u, ref = o.bar === 'reference', bh = (ref ? 62 : 30) * u, mx = T.margin;
    const ch = (ref ? 32 : 20) * u, cw = (ref ? 36 : 24) * u, gap = 3 * u, cy = ref ? 20 * u : 4 * u;
    let s = `<g class="bar">`;
    if (ref) s += text(mx, 13 * u, 'WORKSPACE', { size: 7.5 * u, ls: 0.1 });
    ['1', '2', '3', '4', '5'].forEach((v, i) => {
      s += badge(mx + i * (cw + gap), cy, cw, ch, T, { cap: ref ? 'LEVEL' : null, val: ref ? 'T' + v : v, on: i === 1, cls: i === 1 ? 'a-ws' : '' });
    });
    // window title: the dashboard's tab plate
    const tw = 250 * u, th = (ref ? 16 : 16) * u, tx = (W - tw) / 2, ty = ref ? 28 * u : 6 * u;
    s += path(plate(tx, ty, tw, th), `fill="${INK.tabFace}" fill-opacity=".7" stroke="${INK.tabEdge}" stroke-width="${T.line}"`);
    s += text(W / 2, ty + th * 0.72, title, { size: 8.5 * u, anchor: 'middle', ls: 0.08 });
    // status, right
    const cells = ref ? [['CPU', '12'], ['MEM', '41'], ['TEMP', '47'], ['VOL', '60']] : [['', '12'], ['', '41'], ['', '47'], ['', '60']];
    const sw = (ref ? 36 : 28) * u; let rx = W - mx - 78 * u - cells.length * (sw + gap) - (ref ? 0 : 0);
    if (ref) s += text(rx, 13 * u, 'SYSTEM', { size: 7.5 * u, ls: 0.1 });
    cells.forEach((c, i) => { s += badge(rx + i * (sw + gap), cy, sw, ch, T, { cap: c[0] || null, val: c[1] }); });
    if (!ref) ['CPU', 'MEM', '°C', 'VOL'].forEach((c, i) => 0);
    // clock + tape
    const kx = W - mx;
    s += text(kx, cy + ch * (ref ? 0.62 : 0.74), '14:07', { size: (ref ? 19 : 13) * u, anchor: 'end', font: 'wide', weight: 600 });
    if (ref) { s += text(kx, cy + ch * 0.98, 'SAT 14 NOV', { size: 7 * u, anchor: 'end', ls: 0.12, fill: INK.tape, op: 0.85 }); s += barcode(kx - 70 * u, 8 * u, 70 * u, 5 * u, INK.dim); }
    // the header rule, with its echo band beneath
    const ry = bh - 1;
    s += `<rect x="${mx}" y="${n(ry + 2 * u)}" width="${W - 2 * mx}" height="${n(T.hairline)}" fill="${INK.bright}" opacity=".16"/>`;
    s += `<rect class="rule" x="${mx}" y="${n(ry)}" width="${W - 2 * mx}" height="${n(T.hairline)}" fill="${INK.bright}"/>`;
    return { svg: s + '</g>', h: bh };
  }

  // Window contents: plain stand-ins, in the theme's inks.
  function lines(x, y, w, rows, T, kind) {
    let s = ''; const lh = 9.5 * T.u, fs = 7 * T.u;
    const src = {
      term: ['$ ./check lock', '   PASS  lock, pointer and submodule HEAD agree', 'all 1 checks passed', '$ nix build .#nixosConfigurations.terra', '   evaluating derivation ...', '   building cp-eras-ui-0.0.0', '$ _'],
      code: ['pub struct Weights {', '    /// the full-width header rule', '    pub hairline: f32,', '    /// every plate outline', '    pub line: f32,', '    /// inset contour and tab seam', '    pub inset: f32,', '    /// the footer tape frame', '    pub heavy: f32,', '}', '', 'pub const WEIGHTS: Weights = Weights {', '    hairline: 0.7,', '    line: 0.8333,', '    inset: 1.0,', '    heavy: 1.5,', '};', '', 'impl DarkPlate {', '    pub fn rest() -> Self { REST_PLATE }', '    pub fn lit() -> Self { LIT_PLATE }', '}'],
    }[kind];
    for (let i = 0; i < rows && i < src.length; i++) {
      const l = src[i], yy = y + (i + 1) * lh;
      if (kind === 'code') s += text(x, yy, String(i + 1).padStart(2, ' '), { size: fs, font: 'mono', fill: INK.border, weight: 400, ls: 0 });
      const c = l.trim().startsWith('//') ? INK.dim : (l.includes('PASS') || l.startsWith('pub') || l.startsWith('impl') ? INK.fg : (kind === 'term' && !l.startsWith('$') ? INK.dim : INK.tape));
      s += `<text x="${n(x + (kind === 'code' ? 18 * T.u : 0))}" y="${n(yy)}" class="f-mono" font-size="${n(fs)}" fill="${c}" xml:space="preserve" opacity=".92">${esc(l)}</text>`;
    }
    return s;
  }
  function browser(x, y, w, h, T) {
    const u = T.u; let s = '';
    s += path(plate(x + 8 * u, y + 8 * u, w - 16 * u, 16 * u, { br: 5 * u }), `fill="${INK.ghost}" stroke="${INK.border}" stroke-width="${T.line}"`);
    s += text(x + 16 * u, y + 19.5 * u, 'terra.lan / status', { size: 7.5 * u, fill: INK.dim });
    const cw = (w - 16 * u - 2 * 6 * u) / 3;
    for (let i = 0; i < 3; i++) {
      const cx = x + 8 * u + i * (cw + 6 * u), cyy = y + 32 * u, chh = Math.max(20, h - 40 * u);
      s += path(plate(cx, cyy, cw, chh, { tr: 8 * u }), `fill="${i === 0 ? INK.rest : INK.ghost}" stroke="${i === 0 ? INK.bright : INK.border}" stroke-width="${T.line}"`);
      s += text(cx + 8 * u, cyy + 16 * u, ['THERMAL', 'BUILDS', 'BACKUPS'][i], { size: 9 * u, weight: 600 });
      s += text(cx + 8 * u, cyy + 28 * u, ['47 C  fans 31%', '25 of 25 pass', 'last run 02:10'][i], { size: 7 * u, fill: INK.tape, op: 0.8 });
    }
    return s;
  }

  // A tiled window. Hyprland draws a rectangular border; the title plate
  // (windows: 'red') and the echo are the parts that need more than stock
  // Hyprland (see README).
  function win(x, y, w, h, T, o, { title, focused = false, kind, cls = '' }) {
    const u = T.u, d = plate(x, y, w, h), red = o.windows === 'red';
    const th = red && focused ? 15 * u : 0;
    let s = `<g class="win ${cls} ${focused ? 'is-focus' : ''}">`;
    if (focused) s += echo(d, T, 1, 1);
    s += `<clipPath id="c${n(x)}-${n(y)}"><rect x="${n(x)}" y="${n(y)}" width="${n(w)}" height="${n(h)}"/></clipPath>`;
    s += path(d, `fill="${INK.win}"`);
    s += `<g clip-path="url(#c${n(x)}-${n(y)})">`;
    if (th) { s += `<rect x="${n(x)}" y="${n(y)}" width="${n(w)}" height="${n(th)}" fill="${INK.fg}"/>`; s += text(x + 8 * u, y + th * 0.74, title, { size: 8.5 * u, fill: INK.onRed, weight: 600, ls: 0.08 }); s += barcode(x + w - 60 * u, y + 4 * u, 52 * u, th - 8 * u, INK.onRed); }
    const ix = x + 10 * u, iy = y + th + 6 * u;
    if (kind === 'browser') s += browser(x, y + th, w, h - th, T); else s += lines(ix, iy, w, Math.floor((h - th - 14 * u) / (9.5 * u)), T, kind);
    s += '</g>';
    s += path(d, `class="edge" fill="none" stroke="${focused ? INK.bright : INK.border}" stroke-width="${focused ? T.focus : T.line}"`);
    return s + '</g>';
  }

  function tiled(T, o, extraCls = '') {
    const b = bar(T, o, 'KITTY  //  NIX-CONFIG-PRIVATE'), u = T.u;
    const x0 = T.gapOut, y0 = b.h + T.gapOut, w = W - 2 * T.gapOut, h = H - y0 - T.gapOut;
    const lw = Math.round(w * 0.58), rw = w - lw - T.gapIn, rh = Math.round((h - T.gapIn) / 2);
    return b.svg + `<g class="tiles ${extraCls}">`
      + win(x0, y0, lw, h, T, o, { title: 'EDITOR  //  STYLE.RS', focused: true, kind: 'code', cls: 'w1' })
      + win(x0 + lw + T.gapIn, y0, rw, rh, T, o, { title: 'KITTY', kind: 'term', cls: 'w2 a-win' })
      + win(x0 + lw + T.gapIn, y0 + rh + T.gapIn, rw, h - rh - T.gapIn, T, o, { title: 'BROWSER', kind: 'browser', cls: 'w3' }) + '</g>';
  }

  function sideTape(T) {
    const u = T.u;
    return `<g opacity=".8">${text(0, 0, 'TERRA  NEOMIL  3840 2160', { size: 6.5 * u, ls: 0.14, fill: INK.dim }).replace('<text ', `<text transform="translate(${n(T.margin * 0.6)} ${H * 0.62}) rotate(-90)" `)}</g>`;
  }
  function footTape(T) {
    const u = T.u, w = 150 * u, h = 22 * u, x = W - T.margin - w, y = H - T.margin * 0.7 - h;
    const d = plate(x, y, w, h);
    return echo(d, T, 1, 1) + path(d, `fill="${INK.ghost}" stroke="${INK.bright}" stroke-width="${T.heavy}"`)
      + `<rect x="${n(x + w * 0.42)}" y="${n(y)}" width="${T.heavy}" height="${n(h)}" fill="${INK.bright}"/>`
      + text(x + 6 * u, y + h * 0.62, '0001 0110 0019', { size: 6.5 * u, weight: 600 }) + text(x + w * 0.42 + 6 * u, y + h * 0.45, 'NIXOS 26.11', { size: 6 * u, weight: 600 }) + text(x + w * 0.42 + 6 * u, y + h * 0.8, 'HYPRLAND', { size: 6 * u, weight: 600 });
  }
  function empty(T, o) { return bar(T, o, 'NO WINDOW').svg + sideTape(T) + footTape(T); }

  // Launcher: the GO HOME panel as a container, the password field as the
  // query, store nav rows as results (rest plate / solid selected row).
  function launcher(T, o) {
    const u = T.u, pw = 380 * u, rowH = 30 * u, rows = ['FIREFOX', 'KITTY', 'FILES', 'SETTINGS', 'CP ERAS  //  DASHBOARD'], ph = 86 * u + rows.length * (rowH + 4 * u);
    const px = (W - pw) / 2, py = (H - ph) / 2 - 10 * u, d = plate(px, py, pw, ph, { tr: 8 * u, bl: 22 * u });
    let s = `<g opacity=".3">${tiled(T, o)}</g><rect width="${W}" height="${H}" fill="${INK.bg}" opacity=".45"/><g class="a-launch" style="transform-origin:${n(px)}px ${n(py)}px">`;
    s += echo(d, T, 1, 0) + path(d, `fill="#12080a" fill-opacity=".96" stroke="${INK.bright}" stroke-width="${T.line}"`);
    s += `<rect x="${n(px + pw - 5 * u)}" y="${n(py + ph * 0.3)}" width="${n(5 * u)}" height="${n(ph * 0.32)}" fill="${INK.fg}"/>`;
    s += text(px + 14 * u, py + 26 * u, 'RUN', { size: 17 * u, weight: 700, ls: 0.02 });
    s += barcode(px + pw - 90 * u, py + 14 * u, 70 * u, 6 * u, INK.dim);
    const fx = px + 14 * u, fy = py + 38 * u, fw = pw - 34 * u, fh = 26 * u;
    s += path(plate(fx, fy, fw, fh), `fill="${INK.ghost}" stroke="#760e0f" stroke-width="${T.line}"`);
    s += text(fx + 8 * u, fy + fh * 0.66, 'fi', { size: 12 * u, fill: INK.tape }) + `<rect class="a-caret" x="${n(fx + 8 * u + 11 * u)}" y="${n(fy + 6 * u)}" width="${n(1.5 * u)}" height="${n(fh - 12 * u)}" fill="${INK.bright}"/>`;
    rows.forEach((r, i) => {
      const ry = fy + fh + 10 * u + i * (rowH + 4 * u), rd = plate(fx, ry, fw, rowH, { bl: 9 * u });
      const on = i === 0;
      s += `<g class="a-row" style="animation-delay:${120 + i * 50}ms">` + path(rd, `fill="${INK.rest}" stroke="${INK.border}" stroke-width="${T.line}"`) + text(fx + 14 * u, ry + rowH * 0.66, r, { size: 12 * u, weight: 500 }) + '</g>';
      if (on) s += `<g class="a-sel">` + path(rd, `fill="${INK.fg}" stroke="${INK.bright}" stroke-width="${T.focus}"`) + text(fx + 14 * u, ry + rowH * 0.66, r, { size: 12 * u, weight: 700, fill: INK.onRed }) + `<rect x="${n(fx - 7 * u)}" y="${n(ry + 4 * u)}" width="${n(3 * u)}" height="${n(rowH - 8 * u)}" fill="${INK.fg}"/>` + '</g>';
    });
    s += '</g>';
    // notification: a small bright panel with the heavy tape as its head
    const nw = 250 * u, nh = 58 * u, nx = W - T.gapOut - nw - 6 * u, ny = (o.bar === 'reference' ? 62 : 30) * u + T.gapOut + 6 * u, nd = plate(nx, ny, nw, nh, { tr: 8 * u });
    s += `<g class="a-note">` + echo(nd, T, -1, 0) + path(nd, `fill="#12080a" stroke="${INK.bright}" stroke-width="${T.line}"`)
      + `<rect x="${n(nx)}" y="${n(ny)}" width="${n(4 * u)}" height="${n(nh)}" fill="${INK.fg}"/>`
      + text(nx + 14 * u, ny + 20 * u, 'BUILD FINISHED', { size: 11 * u, weight: 700 }) + text(nx + 14 * u, ny + 34 * u, 'terra toplevel, 4 m 12 s', { size: 8.5 * u, fill: INK.tape, op: 0.85 }) + text(nx + 14 * u, ny + 47 * u, '25 of 25 checks pass', { size: 8.5 * u, fill: INK.dim }) + '</g>';
    return s;
  }

  // Lock / greeter: the login recreation's card, field and button.
  function lock(T, o) {
    const u = T.u, b = bar(T, Object.assign({}, o, { bar: 'reference' }), 'LOCKED');
    const cw = 190 * u, ch = 250 * u, cx = (W - cw) / 2, cy = b.h + (H - b.h - ch - 80 * u) / 2, d = plate(cx, cy, cw, ch, { tr: 34 * u, bl: 16 * u });
    let s = b.svg + sideTape(T);
    s += text(T.margin + 60 * u, H * 0.56, '14:07', { size: 64 * u, font: 'wide', weight: 600, ls: 0 }) + text(T.margin + 62 * u, H * 0.56 + 20 * u, 'SATURDAY 14 NOVEMBER', { size: 9 * u, ls: 0.16, fill: INK.tape, op: 0.85 });
    s += `<g class="a-card">` + echo(d, T, 1, 0) + path(d, `fill="#3a1218" fill-opacity=".85" stroke="${INK.bright}" stroke-width="${T.line}"`);
    const aw = 84 * u, ax = cx + (cw - aw) / 2, ay = cy + 58 * u;
    s += path(plate(ax, ay, aw, aw, { tl: 0 }), `fill="${INK.fg}"`) + path(`M${n(ax)} ${n(ay)}h${n(aw * 0.5)}l${n(6 * u)} ${n(-7 * u)}h${n(-aw * 0.5)}z`, `fill="${INK.fg}"`);
    s += `<circle cx="${n(ax + aw / 2)}" cy="${n(ay + aw * 0.4)}" r="${n(aw * 0.17)}" fill="${INK.onRed}"/>` + path(`M${n(ax + aw * 0.2)} ${n(ay + aw)}c0 ${n(-aw * 0.42)} ${n(aw * 0.6)} ${n(-aw * 0.42)} ${n(aw * 0.6)} 0z`, `fill="${INK.onRed}"`);
    s += text(cx + cw / 2, cy + ch - 28 * u, 'MVERTE', { size: 17 * u, anchor: 'middle', weight: 500, ls: 0.06 }) + '</g>';
    const fw = 250 * u, fh = 26 * u, fx = (W - fw) / 2, fy = cy + ch + 22 * u;
    s += path(plate(fx, fy, fw - 70 * u, fh), `fill="${INK.ghost}" stroke="#760e0f" stroke-width="${T.line}"`) + text(fx + 10 * u, fy + fh * 0.7, '• • • • • •', { size: 11 * u, fill: INK.tape }) + `<rect class="a-caret" x="${n(fx + 76 * u)}" y="${n(fy + 6 * u)}" width="${n(1.5 * u)}" height="${n(fh - 12 * u)}" fill="${INK.bright}"/>`;
    const bx = fx + fw - 64 * u, bd = plate(bx, fy, 64 * u, fh, { br: 9 * u });
    s += path(bd, `fill="${INK.fg}" stroke="${INK.bright}" stroke-width="${T.line}"`) + text(bx + 32 * u, fy + fh * 0.68, 'ENTER', { size: 10.5 * u, anchor: 'middle', fill: INK.onRed, weight: 700, ls: 0.1 });
    return s + footTape(T);
  }

  const SCENES = { tiled: (T, o) => tiled(T, o), empty, launcher, lock };
  let uid = 0;
  function render(scene, opts = {}) {
    const o = Object.assign({ target: 'desktop', bar: 'reference', windows: 'dark' }, opts), T = TARGET[o.target];
    return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${W} ${H}" class="neomil-scene s-${scene} t-${o.target}" role="img" aria-label="Neomil ${scene} mock-up, ${o.target}">${ground(++uid)}${SCENES[scene](T, o)}</svg>`;
  }

  const META = {
    tiled: ['Tiled work desktop', 'Editor focused, terminal and browser beside it. The focused window takes the bright outline and a printed echo; the others the dim outline.'],
    empty: ['Empty desktop', 'The bar on the era\'s ground: blue haze falling to black, a side tape and the footer tape.'],
    launcher: ['Launcher and notification', 'The launcher is the GO HOME panel with the password field as its query and menu rows as results. The notification is a small bright panel.'],
    lock: ['Lock and login', 'The login screen\'s user card, password field and ENTER button, with the clock.'],
  };
  const CHOICES = { target: [['desktop', 'Desktop'], ['tv', 'TV']], bar: [['reference', 'Reference header'], ['slim', 'Slim bar']], windows: [['dark', 'Dark, red lines'], ['red', 'Red title on focus']] };
  const LABEL = { target: 'Target', bar: 'Top bar', windows: 'Windows' };

  // Mount the review UI: one set of switches, then each scene.
  function mount(root, scenes) {
    const state = { target: 'desktop', bar: 'reference', windows: 'dark', motion: true, actual: false };
    try { Object.assign(state, JSON.parse(localStorage.getItem('neomil-design') || '{}')); } catch (e) {}
    const ctl = document.createElement('div'); ctl.className = 'nd-controls';
    const stage = document.createElement('div'); stage.className = 'nd-stage';
    root.append(ctl, stage);
    function draw() {
      try { localStorage.setItem('neomil-design', JSON.stringify(state)); } catch (e) {}
      ctl.innerHTML = '';
      Object.keys(CHOICES).forEach(k => {
        const g = document.createElement('div'); g.className = 'nd-group'; g.setAttribute('role', 'group'); g.setAttribute('aria-label', LABEL[k]);
        const l = document.createElement('span'); l.className = 'nd-label'; l.textContent = LABEL[k]; g.append(l);
        CHOICES[k].forEach(([v, name]) => { const b = document.createElement('button'); b.type = 'button'; b.textContent = name; b.setAttribute('aria-pressed', state[k] === v); b.onclick = () => { state[k] = v; draw(); }; g.append(b); });
        ctl.append(g);
      });
      const g = document.createElement('div'); g.className = 'nd-group';
      [['motion', 'Motion'], ['actual', 'Actual size']].forEach(([k, name]) => { const b = document.createElement('button'); b.type = 'button'; b.textContent = name; b.setAttribute('aria-pressed', !!state[k]); b.onclick = () => { state[k] = !state[k]; draw(); }; g.append(b); });
      ctl.append(g);
      stage.innerHTML = '';
      scenes.forEach(sc => {
        const sec = document.createElement('section'); sec.className = 'nd-scene'; sec.id = sc;
        const h = document.createElement('h2'); h.textContent = META[sc][0];
        const p = document.createElement('p'); p.textContent = META[sc][1];
        const frame = document.createElement('div'); frame.className = 'nd-frame' + (state.actual ? ' is-actual' : '') + (state.motion ? ' play' : '');
        frame.innerHTML = render(sc, state);
        const replay = document.createElement('button'); replay.type = 'button'; replay.className = 'nd-replay'; replay.textContent = 'Replay motion';
        replay.onclick = () => { frame.classList.remove('play'); void frame.offsetWidth; frame.classList.add('play'); };
        sec.append(h, p, frame, replay); stage.append(sec);
      });
    }
    draw();
  }
  window.NeomilDesign = { render, mount, INK, TARGET, scenes: Object.keys(SCENES) };
})();
