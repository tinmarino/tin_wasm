#![allow(dead_code)]
pub use std::f32::consts::PI as PI;

pub const FIELD_OF_VIEW: f32 = 45. * PI / 180.; //in radians
pub const GRID_SIZE: usize = 100;
pub const Z_FAR: f32 = 100.;
pub const Z_NEAR: f32 = 0.1;
pub const Z_PLANE: f32 = -2.414213; //-1 / tan(pi/8)

pub const JS_KEY_CANCEL: u32 = 3;
pub const JS_KEY_HELP: u32 = 6;
pub const JS_KEY_BACK_SPACE: u32 = 8;
pub const JS_KEY_TAB: u32 = 9;
pub const JS_KEY_CLEAR: u32 = 12;
pub const JS_KEY_RETURN: u32 = 13;
pub const JS_KEY_ENTER: u32 = 14;
pub const JS_KEY_SHIFT: u32 = 16;
pub const JS_KEY_CONTROL: u32 = 17;
pub const JS_KEY_ALT: u32 = 18;
pub const JS_KEY_PAUSE: u32 = 19;
pub const JS_KEY_CAPS_LOCK: u32 = 20;
pub const JS_KEY_ESCAPE: u32 = 27;
pub const JS_KEY_SPACE: u32 = 32;
pub const JS_KEY_PAGE_UP: u32 = 33;
pub const JS_KEY_PAGE_DOWN: u32 = 34;
pub const JS_KEY_END: u32 = 35;
pub const JS_KEY_HOME: u32 = 36;
pub const JS_KEY_LEFT: u32 = 37;
pub const JS_KEY_UP: u32 = 38;
pub const JS_KEY_RIGHT: u32 = 39;
pub const JS_KEY_DOWN: u32 = 40;
pub const JS_KEY_PRINTSCREEN: u32 = 44;
pub const JS_KEY_INSERT: u32 = 45;
pub const JS_KEY_DELETE: u32 = 46;
pub const JS_KEY_0: u32 = 48;
pub const JS_KEY_1: u32 = 49;
pub const JS_KEY_2: u32 = 50;
pub const JS_KEY_3: u32 = 51;
pub const JS_KEY_4: u32 = 52;
pub const JS_KEY_5: u32 = 53;
pub const JS_KEY_6: u32 = 54;
pub const JS_KEY_7: u32 = 55;
pub const JS_KEY_8: u32 = 56;
pub const JS_KEY_9: u32 = 57;
pub const JS_KEY_SEMICOLON: u32 = 59;
pub const JS_KEY_EQUALS: u32 = 61;
pub const JS_KEY_A: u32 = 65;
pub const JS_KEY_B: u32 = 66;
pub const JS_KEY_C: u32 = 67;
pub const JS_KEY_D: u32 = 68;
pub const JS_KEY_E: u32 = 69;
pub const JS_KEY_F: u32 = 70;
pub const JS_KEY_G: u32 = 71;
pub const JS_KEY_H: u32 = 72;
pub const JS_KEY_I: u32 = 73;
pub const JS_KEY_J: u32 = 74;
pub const JS_KEY_K: u32 = 75;
pub const JS_KEY_L: u32 = 76;
pub const JS_KEY_M: u32 = 77;
pub const JS_KEY_N: u32 = 78;
pub const JS_KEY_O: u32 = 79;
pub const JS_KEY_P: u32 = 80;
pub const JS_KEY_Q: u32 = 81;
pub const JS_KEY_R: u32 = 82;
pub const JS_KEY_S: u32 = 83;
pub const JS_KEY_T: u32 = 84;
pub const JS_KEY_U: u32 = 85;
pub const JS_KEY_V: u32 = 86;
pub const JS_KEY_W: u32 = 87;
pub const JS_KEY_X: u32 = 88;
pub const JS_KEY_Y: u32 = 89;
pub const JS_KEY_Z: u32 = 90;
pub const JS_KEY_CONTEXT_MENU: u32 = 93;
pub const JS_KEY_NUMPAD0: u32 = 96;
pub const JS_KEY_NUMPAD1: u32 = 97;
pub const JS_KEY_NUMPAD2: u32 = 98;
pub const JS_KEY_NUMPAD3: u32 = 99;
pub const JS_KEY_NUMPAD4: u32 = 100;
pub const JS_KEY_NUMPAD5: u32 = 101;
pub const JS_KEY_NUMPAD6: u32 = 102;
pub const JS_KEY_NUMPAD7: u32 = 103;
pub const JS_KEY_NUMPAD8: u32 = 104;
pub const JS_KEY_NUMPAD9: u32 = 105;
pub const JS_KEY_MULTIPLY: u32 = 106;
pub const JS_KEY_ADD: u32 = 107;
pub const JS_KEY_SEPARATOR: u32 = 108;
pub const JS_KEY_SUBTRACT: u32 = 109;
pub const JS_KEY_DECIMAL: u32 = 110;
pub const JS_KEY_DIVIDE: u32 = 111;
pub const JS_KEY_F1: u32 = 112;
pub const JS_KEY_F2: u32 = 113;
pub const JS_KEY_F3: u32 = 114;
pub const JS_KEY_F4: u32 = 115;
pub const JS_KEY_F5: u32 = 116;
pub const JS_KEY_F6: u32 = 117;
pub const JS_KEY_F7: u32 = 118;
pub const JS_KEY_F8: u32 = 119;
pub const JS_KEY_F9: u32 = 120;
pub const JS_KEY_F10: u32 = 121;
pub const JS_KEY_F11: u32 = 122;
pub const JS_KEY_F12: u32 = 123;
pub const JS_KEY_F13: u32 = 124;
pub const JS_KEY_F14: u32 = 125;
pub const JS_KEY_F15: u32 = 126;
pub const JS_KEY_F16: u32 = 127;
pub const JS_KEY_F17: u32 = 128;
pub const JS_KEY_F18: u32 = 129;
pub const JS_KEY_F19: u32 = 130;
pub const JS_KEY_F20: u32 = 131;
pub const JS_KEY_F21: u32 = 132;
pub const JS_KEY_F22: u32 = 133;
pub const JS_KEY_F23: u32 = 134;
pub const JS_KEY_F24: u32 = 135;
pub const JS_KEY_NUM_LOCK: u32 = 144;
pub const JS_KEY_SCROLL_LOCK: u32 = 145;
pub const JS_KEY_COMMA: u32 = 188;
pub const JS_KEY_PERIOD: u32 = 190;
pub const JS_KEY_SLASH: u32 = 191;
pub const JS_KEY_BACK_QUOTE: u32 = 192;
pub const JS_KEY_OPEN_BRACKET: u32 = 219;
pub const JS_KEY_BACK_SLASH: u32 = 220;
pub const JS_KEY_CLOSE_BRACKET: u32 = 221;
pub const JS_KEY_QUOTE: u32 = 222;
pub const JS_KEY_META: u32 = 22;
