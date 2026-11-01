//Keymap
use rmk::types::action::KeyAction;
use rmk::{a, k, layer, mo};
pub(crate) const COL: usize = 5;
pub(crate) const ROW: usize = 8;
pub(crate) const NUM_LAYER: usize = 3;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW*2]; NUM_LAYER] {
  [
    [
      // Left
      [k!(Slash), k!(Z), k!(G), k!(W), k!(MouseBtn1)],
      [k!(V), k!(C), k!(M), k!(K), k!(No)],       // Col 1 (now Row 1)
      [k!(Backspace), k!(E), k!(Space), k!(R), k!(No)], // Col 2 (now Row 2)
      [k!(Delete), k!(O), k!(Dot), k!(I), k!(No)],      // Col 3 (now Row 3)
      [k!(LCtrl), k!(U), k!(Comma), k!(Quote), k!(No)], // Col 4 (now Row 4)
      [k!(LAlt), k!(RAlt), mo!(1), k!(LShift), k!(No)], // Col 5 (now Row 5)
      [k!(Equal), k!(LGui), k!(Escape), k!(Slash), k!(LGui)], // Col 6 (now Row 6)
      [k!(Up), k!(Down), k!(Right), k!(Left),  k!(No)],  // Col 7 (now Row 7)
      // Right
      [k!(X), k!(Q), k!(B), k!(MouseBtn1), k!(No)],
      [k!(P), k!(D), k!(F), k!(H), k!(No)],       // Col 1 (now Row 1)
      [k!(Enter), k!(T), k!(A), k!(Space), k!(No)], // Col 2 (now Row 2)
      [k!(Tab), k!(N), k!(L), k!(J), k!(No)],      // Col 3 (now Row 3)
      [k!(RCtrl), k!(S), k!(Y), k!(Semicolon), k!(No)], // Col 4 (now Row 4)
      [k!(RAlt), k!(MouseBtn2), k!(RShift), mo!(1), k!(No)], // Col 5 (now Row 5)
      [k!(Minus), mo!(2), k!(Backslash), k!(Escape), k!(LGui)], // Col 6 (now Row 6)
      [k!(MouseUp), k!(MouseDown), k!(MouseLeft), k!(MouseRight), k!(MouseBtn1)],  // Col 7 (now Row 7)
    ],
    [
      // Left
      [k!(KpDot), k!(Minus), k!(No), k!(No), k!(No)],
      [k!(Kc8), k!(Kc0), k!(Kc9), k!(Kc7), k!(No)],       // Col 1 (now Row 1)
      [k!(Backspace), k!(Kc2), k!(Kc3), k!(Kc1), k!(No)], // Col 2 (now Row 2)
      [k!(Delete), k!(Kc5), k!(Kc6), k!(Kc4), k!(No)],      // Col 3 (now Row 3)
      [k!(LCtrl), k!(Backslash), k!(Equal), k!(Grave), k!(No)], // Col 4 (now Row 4)
      [k!(LAlt), k!(RAlt), mo!(1), k!(LShift), mo!(3)], // Col 5 (now Row 5)
      [k!(PageUp), k!(PageDown), k!(Home), k!(End), k!(No)], // Col 6 (now Row 6)
      [k!(Up), k!(Down), k!(Left), k!(Right), k!(No)],  // Col 7 (now Row 7)
      // Right
      [k!(LeftBracket), k!(RightBracket), k!(Kc0), k!(Kc9), k!(No)],
      [k!(Kc8), k!(Kc0), k!(Kc7), k!(Kc9), k!(No)],       // Col 1 (now Row 1)
      [k!(No), k!(Kc2), k!(Kc1), k!(Kc3), k!(No)], // Col 2 (now Row 2)
      [k!(No), k!(Kc5), k!(Kc4), k!(Kc6), k!(No)],      // Col 3 (now Row 3)
      [k!(No), k!(Dot), k!(Equal), k!(No), k!(No)], // Col 4 (now Row 4)
      [k!(No), k!(MouseBtn2), k!(RShift), mo!(1), mo!(1)], // Col 5 (now Row 5)
      [k!(Execute), k!(Escape), k!(No), k!(No), k!(No)], // Col 6 (now Row 6)
      [k!(MouseWheelUp), k!(MouseWheelDown), k!(MouseWheelLeft), k!(MouseWheelRight), k!(No)],  // Col 7 (now Row 7)
    ],
    [
        // Left
        [k!(F11), k!(No), k!(F12), k!(No), k!(No)],       // Col 1 (now Row 0)
        [k!(F8), k!(F10), k!(F9), k!(F7), k!(No)],            // Col 2 (now Row 1)
        [k!(No), k!(F2), k!(F3), k!(F1), k!(No)],           // Col 3 (now Row 2)
        [k!(No), k!(F5), k!(F6), k!(F4), k!(No)],           // Col 4 (now Row 3)
        [k!(No), k!(No), k!(No), k!(No), k!(No)],     // Col 5 (now Row 4)
        [k!(No), k!(No), k!(No), k!(No), k!(No)],     // Col 6 (now Row 5)
        [k!(No), k!(No), k!(No), k!(No), k!(No)],     // Col 7 (now Row 6)
        [k!(No), k!(No), k!(No), k!(No), k!(No)],     // Col 8 (now Row 7)
        // Right
        [k!(F11), k!(No), k!(No), k!(F12), k!(No)],       // Col 9  (now Row 8)
        [k!(F8), k!(F10), k!(F7), k!(F9), k!(No)],            // Col 10 (now Row 9)
        [k!(No), k!(F2), k!(F1), k!(F3), k!(No)],           // Col 11 (now Row 10)
        [k!(No), k!(F5), k!(F4), k!(F6), k!(No)],           // Col 12 (now Row 11)
        [k!(No), k!(No), k!(No), k!(No), k!(No)],     // Col 13 (now Row 12)
        [k!(No), k!(No), k!(No), k!(No), k!(No)],     // Col 14 (now Row 13)
        [k!(No), k!(No), k!(No), k!(No), k!(No)],// Col 15 (now Row 14)
        [k!(No), k!(No), k!(No), k!(No), k!(No)], // Col 16 (now Row 15)
    ],
  ]
}
