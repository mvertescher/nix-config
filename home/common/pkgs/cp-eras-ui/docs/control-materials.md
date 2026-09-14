# Native control materials

`widgets::controls::{button, field}` dress one native iced child with a passive
canvas backdrop. The constructors install the supplied callback; `None`
disables the native control and its material together. Configure padding,
width, input ID, secure text or submit behavior on the native widget before
passing it in. The wrapper replaces its style and activation/input callback.

The `control-states` example has static comparisons and live native controls
for every era. `preview(State::Pressed)` previews the field's focus material;
it does not manufacture text focus or a caret. Click a live field for the real
native caret, selection and IME. All four live fields share the example's value.

Material data lives in `eras/control_materials.rs`; the wrapper has no era
checks. Neomil and Entropism continue using the catalog's simple coats.
Kitsch and Neokitsch use the inferred recipes annotated in their component
sheets, with dimensions adapted to native content:

- Kitsch buttons retain the login ENTER bar's 7px step, 12px ramp and relative
  shoulder position. Hover repeats the silhouette at (+20,-20); a solid CTA
  keeps its fill, while the outlined secondary button becomes teal. A held
  button is flat amber with dark printing. Fields remain rounded 2px dark
  slots; hover adds the ghost, and focus removes it while native iced draws
  the caret. Adapting the button recipe to the outlined secondary is inferred.
- Neokitsch buttons retain the rounded silhouette and lower-left cut. The
  secondary button keeps its small bottom tab, including the held variant
  that projects 1px below the body. Hover adds seven outward echoes. Press
  uses the shared synthesized veneer and dark printing; this is an adaptation
  of the traced grain rather than a verbatim transcription of its strands.
  Fields remain plain chocolate rectangles; hover echoes them and focus
  removes the echo. Extending the secondary silhouette to the CTA is inferred.

The backdrop may extend beyond the control, but it never changes native layout
or hit bounds. Parent clipping remains in effect. Native input state is never
swapped when material changes: cursor, selection, focus, operations, overlays
and IME are delegated to the same child. Buttons remain native release-to-
activate controls. A window exit or focus loss cancels their pending native
press; changing enabled state also invalidates a held button. No animation
interpolation or synthetic focus policy is introduced.

Tests check focus and selection retention across rebuilds, visual cancellation,
and the separation of material state from shape and focused field fill.
Headless native-event tests also verify cancelled/disabled presses cannot
activate later, touch release/loss behavior, and a Unicode commit replacing
the preserved selection after input re-enabling. These exercise event
delegation, not an OS IME connection or preedit display.
Desktop input and IME behavior still need a live check after activation.
