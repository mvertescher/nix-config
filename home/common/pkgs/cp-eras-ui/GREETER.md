# The login screen as a greetd greeter

Written 2026-09-06, when `cp-eras-ui-login` grew a live password
field and a `--greet` mode. What was built, what the era tables now
say, what was checked, and what was not.

## Shape

- **`Slot.value` is gone; `Slot.entry: Option<Entry>` replaces it**
  (`src/style.rs`). An `Entry` is the live field's typography and
  behaviour as table data: `rest` (the run the trace draws at rest,
  a `Legend`), `mask` (the glyph one typed character shows as),
  `tail` (what follows the typed run), `caret: Fixed | Trails`, and
  the two words the prompt says while greetd is asked (`busy`) and
  after it refused (`failed`). Every slot with a `field` has an
  `entry` and no other slot does (tested).
- **At rest the screen draws `rest.text` verbatim.** "At rest" means
  the keyboard has not been touched (`Login.awake == false`), not
  "the secret is empty": after a Backspace to nothing or an Escape
  the field shows the empty run (`tail` alone), because a field
  that shows ten stars when nothing is typed is not a password field.
  The idle frame is therefore exactly what the goldens hold.
- **Awake, the field shows `mask × typed + tail`.** A trailing caret
  is the table's caret plate moved right by the measured width of
  the masks alone (same shaper, same tracking, same stretch as
  `rest`), so at zero typed it is where the trace put it.
- **The notice takes an existing legend's place.** `busy`/`failed`
  are drawn in the prompt legend's face and position when the era
  sets a prompt (entropism, neomil), else in the action label's
  (kitsch, neokitsch). No new geometry.
- **The keyboard goes to the first slot with an entry.** Neokitsch
  offers A and B, both live in the trace; B stays at rest.

## Per era

| era | `rest.text` (idle, unchanged from before) | mask | tail | caret | busy / failed |
|---|---|---|---|---|---|
| entropism | `***********` (bold 22, x 578, baseline 439) | `*` | `` | Trails (underline, 577..594 at y 439.5) | `VERIFYING:` / `ACCESS DENIED:` in the USERNAME: prompt |
| neomil | `**********  __` (12, x 381, baseline 626) | `*` | `  __` | Fixed (it is a text caret on the Login button) | `verifying:` / `access denied:` in the `password:` prompt |
| kitsch | `` (new: medium 22 in the cursor's mint, x 272, baseline 439) | `*` | `` | Trails (the mint block at x 266) | `WAIT` / `DENIED` on the ENTER bar |
| neokitsch | `` (new: medium 19.5 in Fg, tracked 0.15, x 429 / 849, baseline 389) | `*` | `` | Fixed (there is none) | `VERIFYING` / `ACCESS DENIED` on the ENTER / LOGIN bar |

Kitsch and neokitsch had no value legend before because their traces
show an empty well; their `rest` runs carry no text and exist to say
where a typed run goes. The positions were derived from the field
box and the neighbouring cursor/name (kitsch: the cursor's 22px
height, y 421..443; neokitsch: the name's face, a baseline centred in
the 42px well) and are the one thing on this screen that is not a
measurement off a trace. If a vision pass ever puts a typed state in
the traces, replace them.

Four typed characters in neomil read `****  __`; ten read the trace.
The invariant that `rest.text == mask × k + tail` for some k is
tested, so a golden is one state of the live field rather than a
picture the field replaces.

## The greeter

- `src/greetd.rs`: a blocking client for the greetd IPC (native-endian
  `u32` length + JSON) with a hand-rolled writer and a small flat
  JSON reader. No new crates: `greetd_ipc` would pull serde_json into
  a tree that has neither it nor serde's derive, and move the
  `Cargo.lock` that crane vendors from. Conversation:
  `create_session{username}` → answer the `secret` prompt once
  (`info`/`error`/`visible` prompts get `null`; a second secret
  prompt is refused) → on `success`, `start_session{cmd, env: []}` →
  `success`. Anything else is `Refusal::Denied` (an `auth_error`) or
  `Refusal::Broken`, and the session is cancelled before returning.
  Tested against a fake greetd on a unix socket in a temp dir (right
  password, wrong password, no socket).
- `Secret`: the typed password. `Debug` prints `Secret(..)`, the
  bytes are volatile-zeroed on clear and on drop, and there is one
  copy, moved onto the greetd thread on Enter.
- `screens::login`: `Login` holds the secret, `awake`, a `Phase`
  (`Idle | Submitting | Failed | Success`) and an optional `Greeter
  { user, cmd }`. `Message` is `Typed(String) | Backspace | Clear |
  Submit | Outcome(Result<(), Refusal>)`, with a hand-written `Debug`
  that hides the text. The keyboard arrives through
  `iced::event::listen_with` (printable text appends; Enter submits;
  Backspace deletes; Escape and Ctrl-U clear; chords are dropped).
  Enter in greeter mode moves the secret to a `std::thread` that
  calls `greetd::login`, and the answer comes back through an
  `async_channel` a `Task::perform` awaits -- the frame never blocks
  on PAM. `Outcome(Ok)` is `iced::exit()`, so the process exits 0.
  Keys are ignored while `Submitting`. In demo mode (no `--greet`)
  Enter clears the field.
- `examples/cp-eras-ui-login.rs`: `--greet` requires `--user` and
  `--cmd` (exit 2 with usage otherwise). `shell::flag`/`shell::switch`
  are the argument readers; `era_from` now uses `flag`.

## Nix

`system/wm/hyprland.nix` gained `custom.greetd.greeter` (`"tuigreet"`,
the default, or `"cp-eras-ui"`), `custom.greetd.era` (default
`neomil`) and `custom.greetd.user` (default `custom.greetd.lastUser`;
asserted non-null when the greeter is cp-eras-ui). The cp-eras-ui
session is

    cage -s -d -- cp-eras-ui-login --greet --era <era> --user <user> \
        --cmd 'uwsm start hyprland-uwsm.desktop'

`pkgs.cp-eras-ui` and `pkgs.cage` are reachable from the module: both
resolve in `nixosConfigurations.terra.pkgs`, and the branch was
evaluated through `extendModules` to the expected command string.
Terra is unchanged (tuigreet), and its toplevel builds.

cage has no "exit when the application exits" flag because that is
its default behaviour; `-s` allows VT switching so a broken greeter
leaves a tty reachable, `-d` asks for no client-side decorations.

The greeter user is given `video` when the cp-eras-ui greeter is
selected. Under greetd, cage's seat comes from logind (greetd opens a
PAM session for `greeter` on the VT; libseat's logind backend hands
wlroots the devices), which is the documented greetd + cage setup and
needs no groups; the render node wgpu opens is world-readable under
udev's defaults. `video` is belt-and-braces for libseat's direct
fallback. **None of this has been exercised on a real seat.**

## Verified / not verified

Verified:

- `cargo build --release --bins` and `cargo test --release` green
  (69 lib tests, including the fake-greetd conversations), no
  warnings.
- `scripts/run_test_matrix.sh 'login\.'`: 4/4 pass at 100.000%
  before and after, and the four `render.png`s from the run after
  the change are byte-identical to those from the run before it.
- `nix build '.?submodules=1#nixosConfigurations.terra.config.system.build.toplevel'`
  from the wrapper succeeds; terra's greetd command is still tuigreet.

Not verified:

- greetd itself, PAM, cage on a real seat, wgpu under the `greeter`
  account, the handover on `start_session`. All of it needs a switch
  on a host set to `custom.greetd.greeter = "cp-eras-ui"`, and the
  recovery path if it fails is a VT (`cage -s`) or SSH.
- The awake states of the screen (typed runs, trailing carets, the
  notices) have no golden and were not rendered: the golden harness
  cannot type. Their geometry is measured through the same shaper
  the rest of the screen uses, and the unit tests cover the strings.

## Things the brief had wrong

- `iced::keyboard::on_key_press` does not exist in iced 0.14; the
  subscription is `iced::event::listen_with` (or `keyboard::listen`).
- The `greetd_ipc` crate was not used; see above for why. The
  protocol is as the brief describes it, except that the
  `post_auth_message_response` after an `info`/`error` prompt carries
  `"response": null`, and the success after `start_session` is a
  second `success` reply that must be read.
- cage does not need a flag to exit with its application.

## Seen along the way

- `docs/neomil/login-trace.svg` line 225 sets the masked run with
  `letter-spacing="3"`; the table's legend has no `.tracked(3.0)`.
  Left alone -- the goldens hold the untracked run -- but a
  fidelity pass on that field would find it.
