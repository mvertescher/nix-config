pub mod chrome;
pub mod ground;
pub mod surface;
pub mod text;

// What is left of the neo-militarism widget set, from before the
// toolkit was generalised.
//
// It used to be seven modules and is now two, and the rule that emptied
// it is worth keeping: a widget that draws *one era's* geometry and has
// no caller is not a spare part, it is a trap. The next person to want
// a chip or a badge finds it, calls it, and gets neomil's chamfer in
// all four eras. So each one was either wired into the shared
// vocabulary or deleted.
//
//   * `message_card` -- a mail row with a hardcoded 8px double
//     chamfer, unreachable since the entropism-ui retirement. Deleted;
//     `row::mail_row` poses the row and `panels::mail`'s `message_row`
//     is the clickable one. 213 lines, and not one golden pixel moved.
//   * `diamond_menu` -- neomil's cut-diamond hub, the `Menu::Diamonds`
//     arm. Deleted with that variant. It was the crate's own recorded
//     stand-in for a data table -- no neomil sheet draws a diamond
//     anywhere, and where `screens::dashboard` puts its menu the sheet
//     puts a services table -- and `widgets::table` became that table
//     (itself deleted since; see below).
//     Its own header said it was "the first thing to reconsider when
//     the table lands". Nothing was lost with it: the hit-testing it
//     was credited with is `mouse_area` in `panels::mail` and `bar`,
//     and canvas-level hit-testing is not needed by a table built out
//     of layout.
//   * `chip`, `level_badge`, `text_box`, `vertical_text` -- four
//     dressed rectangles with neomil's cut sizes baked in, no caller
//     between them since the retirement. Deleted; `surface::Surface`
//     draws all four of those shapes in four eras.
//   * `menu` (`Menu::Fan/Cascade/Tiles/Table`), `table`, `charts`,
//     `marker`, `pill` -- the widget-built dashboard's furniture, and
//     the two layouts the dashboard drew to misread sources. Deleted
//     2026-09-03 when the dashboard became a trace-driven scene like
//     the store (`screens::scene`): after that fold not one of them had
//     a caller outside `screens::dashboard`. The diamond, tile, fan and
//     cascade menus now live as `Prim` lists in each era's
//     `// --- dashboard ---` block, transcribed from the traces the
//     widgets had only approximated.
//   * `banner`, `bracket`, `card`, `glyph`, `input`, `ornament`, `row`,
//     `silhouette` -- the era-agnostic set the widget-built screens
//     were assembled from. Deleted 2026-09-05: every screen is a
//     `Prim` table now, and not one of the eight had a caller outside
//     the others. What the roadmap still wants of them is rebuilt from
//     `docs/<era>/components.svg` when a caller exists, not kept warm.
//
// `floppy_icon` and `floppy_vector` stay, and they are the one honest
// exception: they are *art*, not a dressed rectangle -- a traced
// vector with no era in it at all -- and `cp-eras-ui-floppy` is a
// built binary that draws them. Reachable, so not a trap.
pub mod floppy_icon;
pub mod floppy_vector;

pub use chrome::{footer, top_bar};
pub use ground::ground;
pub use surface::{layered, surface, Corners, Cut, Fill, Surface};

pub use floppy_icon::{floppy_icon, FloppyIcon};
