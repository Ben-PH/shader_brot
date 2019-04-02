// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use ggez::event::KeyCode;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

mod test {

    #[test]
    fn key_code_to_direction() {
        
        assert_eq!(Some(Direction::Up), Direction::from_keycode(KeyCode::Up));
        assert_eq!(Some(Direction::Down), Direction::from_keycode(KeyCode::Down));
        assert_eq!(Some(Direction::Left), Direction::from_keycode(KeyCode::Left));
        assert_eq!(Some(Direction::Right), Direction::from_keycode(KeyCode::Right));
        assert_eq!(None, Direction::from_keycode(KeyCode::A));
    }
}
