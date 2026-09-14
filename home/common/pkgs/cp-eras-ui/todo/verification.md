[All workstreams and current status](../TODO.md). File paths in these
records are relative to the crate root. Dated notes retain their original
reasoning; later completion entries supersede earlier open-item lists.

## Headless check: feasibility settled (2026-08-22)

Both unknowns blocking "visual regression as a nix checkPhase" were
probed inside an actual nix build sandbox, not just on a headless box.

- [x] **Weston runs in the build sandbox.** `--backend=headless
  --renderer=pixman --shell=kiosk --no-config` starts and enables its
  output, with `XDG_RUNTIME_DIR` pointed at a mode-700 dir under
  `$TMPDIR` and `HOME` set. No seat, no /dev/dri needed.
- [x] **The app renders, but the recorded recipe was incomplete.** A
  software Vulkan ICD alone is not sufficient. Without forcing the
  backend, wgpu picks GLES and panics in
  `wgpu-hal-0.19.5/src/gles/egl.rs:789` — `unwrap()` on `None`, i.e. no
  EGL display in the sandbox. The fix is **`WGPU_BACKEND=vulkan`**
  alongside
  `VK_ICD_FILENAMES=<mesa>/share/vulkan/icd.d/lvp_icd.x86_64.json`
  (mesa 26.1.2 ships it at that path). With both set the app runs
  clean.

Remaining for the checkPhase itself:

- [x] Capture with `weston-screenshooter` and compare (done). Note the
  comparison target has to change: `images/` is **gitignored**, so the
  downloaded Behance references are not in the repo and cannot be in a
  hermetic build. Diff against the tracked `docs/target-*.svg`
  rasterised instead — which is also the cleaner answer, since those
  are our own design targets rather than someone else's copyrighted
  artwork.
- [x] Wired as `passthru.tests` rather than a gating `checkPhase`: a GPU-less compositor is exactly the kind of thing that fails
  for environmental reasons, and it should not block every build of the
  toolkit until it has proven stable.

## Black window on the nvidia/Hyprland session (2026-08-22)

- [x] **Fixed 2026-08-22: wgpu was choosing a non-presenting adapter.**
  This machine exposes three Vulkan adapters (discrete nvidia, the
  Ryzen's integrated RADV, llvmpipe) and wgpu picked one that cannot
  present, so the app really was rendering - somewhere the compositor
  never shows. `WGPU_POWER_PREF=high` alone fixes it (1 unique colour
  in the window without, 815 with); set via --set-default in the
  wrapper of both Iced crates. Original symptom description below.

- [x] ~~The app renders headless but not on the real desktop.~~ Under
  weston headless with the llvmpipe Vulkan ICD it draws correctly (that
  is what tests.visual captures). Launched on a live Hyprland session
  with the nvidia driver it starts, stays alive, logs *nothing* to
  stderr, and presents a solid black window. Forcing
  `WGPU_BACKEND=vulkan` does not change it, and `nvidia_icd.json` is
  present in /run/opengl-driver/share/vulkan/icd.d, so the ICD is
  findable.

  Worth noting the visual test cannot catch this: it exercises the
  software path only. A green build does not mean the app works on the
  machine you use.

  Things not yet tried: `WGPU_BACKEND=gl`, running under
  `nixGL`/`nixglhost`, checking whether the wrapper's LD_LIBRARY_PATH
  shadows the driver's libvulkan, and whether the compositor reporting
  `explicit sync: no` matters for wgpu presentation on nvidia.

## Goldens and git history (watch item, 2026-08-24)

- Every verification round that re-renders the 21-case matrix writes
  ~1.5 MB of PNGs into permanent git history. Fine today (repo `.git` is
  ~6.6 MB, and the byte-identical policy means a golden only changes
  when pixels genuinely do), but the escape hatches — git-lfs, or
  splitting this crate into its own repo — both fight the nix fetchers
  and the wrapper's pin, so the cheap moment to act is early. Notice
  this at 100 MB, not at 1 GB. The wrapper's TODO carries the design
  context under *Design review (2026-08-24)*.
