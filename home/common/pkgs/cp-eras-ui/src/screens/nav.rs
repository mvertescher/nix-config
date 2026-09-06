//! Keyboard navigation over a scene's plates.
//!
//! Every trace-driven screen is a set of plates -- hit boxes at frame
//! coordinates -- and none of them is a grid: neomil staggers its
//! diamonds, kitsch fans blades at +-30 degrees, neokitsch steps its
//! cards up a cascade, and the store hangs a column of categories
//! beside a shelf of cards. So the keyboard does not walk an index; it
//! walks the *plane*. `h`/`j`/`k`/`l` (and the arrows) move to the
//! nearest plate in that direction, judged from the plates' centres,
//! which is the one rule that reads right on all of those shapes
//! without any screen knowing which era drew it.
//!
//! [`Stroke`] is the whole vocabulary: a move, `Enter` to open, `Esc`
//! to go back. [`strokes`] is the subscription; a screen maps them into
//! its own messages through its `stroke`, and the hub (`screens::hub`)
//! is the one that acts on `Open` and `Back`.

use iced::keyboard::{self, key::Named, Key};
use iced::{Point, Subscription};

/// A direction on the frame, `h j k l` in that order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Left,
    Down,
    Up,
    Right,
}

/// What a key press means to a navigable screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stroke {
    Move(Dir),
    Open,
    Back,
}

/// The stroke a key is, if it is one. Chords are not: a `Ctrl-l` is
/// the terminal's, not ours.
pub fn stroke(key: &Key, modifiers: keyboard::Modifiers) -> Option<Stroke> {
    if modifiers.control() || modifiers.alt() || modifiers.logo() {
        return None;
    }
    Some(match key.as_ref() {
        Key::Character("h") | Key::Named(Named::ArrowLeft) => Stroke::Move(Dir::Left),
        Key::Character("j") | Key::Named(Named::ArrowDown) => Stroke::Move(Dir::Down),
        Key::Character("k") | Key::Named(Named::ArrowUp) => Stroke::Move(Dir::Up),
        Key::Character("l") | Key::Named(Named::ArrowRight) => Stroke::Move(Dir::Right),
        Key::Named(Named::Enter) | Key::Named(Named::Space) => Stroke::Open,
        Key::Named(Named::Escape) => Stroke::Back,
        _ => return None,
    })
}

/// Every key press on the window that is a stroke. A screen on its
/// own takes what it wants with `.filter_map(Screen::stroke)` -- the
/// screen's `stroke` as a fn item, since iced's `filter_map` wants a
/// zero-sized mapper and a fn *pointer* is not one.
pub fn strokes() -> Subscription<Stroke> {
    iced::event::listen_with(|event, _status, _window| match event {
        iced::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
            stroke(&key, modifiers)
        }
        _ => None,
    })
}

/// The plate to land on from `from` going `dir`: of the candidates
/// ahead in that direction, the one with the least distance along it
/// plus twice the distance across it, so a plate straight ahead beats
/// a nearer one off to the side. `None` when nothing lies that way,
/// which is where the cursor stays.
///
/// Twice, and not more, because the traces' menus are not aligned: on
/// neokitsch's cascade every card is as far up as it is across, and a
/// heavier cross weight would leave `k` and `l` both dead there. On
/// neomil's staggered rows `l` from a top diamond has the next top
/// diamond at 196 across and the bottom one at 97 across, 133 down --
/// scoring 196 against 363, so the row is walked before the stagger.
pub fn step<T: Copy>(candidates: impl IntoIterator<Item = (T, Point)>, from: Point, dir: Dir) -> Option<T> {
    let (ax, ay) = match dir {
        Dir::Left => (-1.0, 0.0),
        Dir::Right => (1.0, 0.0),
        Dir::Up => (0.0, -1.0),
        Dir::Down => (0.0, 1.0),
    };
    let mut best: Option<(T, f32)> = None;
    for (id, centre) in candidates {
        let (dx, dy) = (centre.x - from.x, centre.y - from.y);
        let along = dx * ax + dy * ay;
        let across = (dx * ay - dy * ax).abs();
        // Half a pixel of slack so two centres on one row do not count
        // as "ahead" of each other vertically through float noise.
        if along <= 0.5 {
            continue;
        }
        let score = along + 2.0 * across;
        if best.map_or(true, |(_, s)| score < s) {
            best = Some((id, score));
        }
    }
    best.map(|(id, _)| id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::keyboard::Modifiers;

    fn p(x: f32, y: f32) -> Point {
        Point::new(x, y)
    }

    #[test]
    fn hjkl_and_the_arrows_are_moves_and_enter_and_esc_the_rest() {
        let none = Modifiers::empty();
        assert_eq!(stroke(&Key::Character("h".into()), none), Some(Stroke::Move(Dir::Left)));
        assert_eq!(stroke(&Key::Character("j".into()), none), Some(Stroke::Move(Dir::Down)));
        assert_eq!(stroke(&Key::Character("k".into()), none), Some(Stroke::Move(Dir::Up)));
        assert_eq!(stroke(&Key::Character("l".into()), none), Some(Stroke::Move(Dir::Right)));
        assert_eq!(stroke(&Key::Named(Named::ArrowDown), none), Some(Stroke::Move(Dir::Down)));
        assert_eq!(stroke(&Key::Named(Named::Enter), none), Some(Stroke::Open));
        assert_eq!(stroke(&Key::Named(Named::Escape), none), Some(Stroke::Back));
        assert_eq!(stroke(&Key::Character("x".into()), none), None);
        assert_eq!(stroke(&Key::Character("l".into()), Modifiers::CTRL), None);
    }

    /// A 2x2 grid: each direction lands on the neighbour, and off the
    /// edge stays put.
    #[test]
    fn a_grid_walks_like_a_grid() {
        let grid = [(0, p(0.0, 0.0)), (1, p(100.0, 0.0)), (2, p(0.0, 100.0)), (3, p(100.0, 100.0))];
        assert_eq!(step(grid, grid[0].1, Dir::Right), Some(1));
        assert_eq!(step(grid, grid[0].1, Dir::Down), Some(2));
        assert_eq!(step(grid, grid[3].1, Dir::Left), Some(2));
        assert_eq!(step(grid, grid[3].1, Dir::Up), Some(1));
        assert_eq!(step(grid, grid[0].1, Dir::Left), None);
        assert_eq!(step(grid, grid[0].1, Dir::Up), None);
    }

    /// Neomil's stagger: the top row is walked before the diamond
    /// tucked below and between.
    #[test]
    fn a_staggered_row_is_walked_along_the_row() {
        let units = [
            (0, p(334.0, 460.0)),
            (1, p(530.0, 460.0)),
            (2, p(725.0, 460.0)),
            (3, p(431.0, 593.0)),
            (4, p(628.0, 592.0)),
            (5, p(822.0, 592.0)),
        ];
        assert_eq!(step(units, units[0].1, Dir::Right), Some(1));
        assert_eq!(step(units, units[0].1, Dir::Down), Some(3));
        assert_eq!(step(units, units[3].1, Dir::Right), Some(4));
        assert_eq!(step(units, units[5].1, Dir::Up), Some(2));
    }
}
