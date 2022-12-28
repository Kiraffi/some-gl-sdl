use std::convert::TryInto;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use window_state::MyKey;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum SdlKey
{
    InvalidKey = !0u32,
    Backspace = 8u32,
    Tab = 9u32,
    Return = 13u32,
    Escape = 27u32,
    Space = 32u32,
    Exclaim = 33u32,
    Quotedbl = 34u32,
    Hash = 35u32,
    Dollar = 36u32,
    Percent = 37u32,
    Ampersand = 38u32,
    Quote = 39u32,
    LeftParen = 40u32,
    RightParen = 41u32,
    Asterisk = 42u32,
    Plus = 43u32,
    Comma = 44u32,
    Minus = 45u32,
    Period = 46u32,
    Slash = 47u32,
    Num0 = 48u32,
    Num1 = 49u32,
    Num2 = 50u32,
    Num3 = 51u32,
    Num4 = 52u32,
    Num5 = 53u32,
    Num6 = 54u32,
    Num7 = 55u32,
    Num8 = 56u32,
    Num9 = 57u32,
    Colon = 58u32,
    Semicolon = 59u32,
    Less = 60u32,
    Equals = 61u32,
    Greater = 62u32,
    Question = 63u32,
    At = 64u32,
    LeftBracket = 91u32,
    Backslash = 92u32,
    RightBracket = 93u32,
    Caret = 94u32,
    Underscore = 95u32,
    Backquote = 96u32,
    A = 97u32,
    B = 98u32,
    C = 99u32,
    D = 100u32,
    E = 101u32,
    F = 102u32,
    G = 103u32,
    H = 104u32,
    I = 105u32,
    J = 106u32,
    K = 107u32,
    L = 108u32,
    M = 109u32,
    N = 110u32,
    O = 111u32,
    P = 112u32,
    Q = 113u32,
    R = 114u32,
    S = 115u32,
    T = 116u32,
    U = 117u32,
    V = 118u32,
    W = 119u32,
    X = 120u32,
    Y = 121u32,
    Z = 122u32,
    Delete = 127u32,
    CapsLock = 1073741881u32,
    F1 = 1073741882u32,
    F2 = 1073741883u32,
    F3 = 1073741884u32,
    F4 = 1073741885u32,
    F5 = 1073741886u32,
    F6 = 1073741887u32,
    F7 = 1073741888u32,
    F8 = 1073741889u32,
    F9 = 1073741890u32,
    F10 = 1073741891u32,
    F11 = 1073741892u32,
    F12 = 1073741893u32,
    PrintScreen = 1073741894u32,
    ScrollLock = 1073741895u32,
    Pause = 1073741896u32,
    Insert = 1073741897u32,
    Home = 1073741898u32,
    PageUp = 1073741899u32,
    End = 1073741901u32,
    PageDown = 1073741902u32,
    Right = 1073741903u32,
    Left = 1073741904u32,
    Down = 1073741905u32,
    Up = 1073741906u32,
    NumLockClear = 1073741907u32,
    KpDivide = 1073741908u32,
    KpMultiply = 1073741909u32,
    KpMinus = 1073741910u32,
    KpPlus = 1073741911u32,
    KpEnter = 1073741912u32,
    Kp1 = 1073741913u32,
    Kp2 = 1073741914u32,
    Kp3 = 1073741915u32,
    Kp4 = 1073741916u32,
    Kp5 = 1073741917u32,
    Kp6 = 1073741918u32,
    Kp7 = 1073741919u32,
    Kp8 = 1073741920u32,
    Kp9 = 1073741921u32,
    Kp0 = 1073741922u32,
    KpPeriod = 1073741923u32,
    //Application = 1073741925u32,
    //Power = 1073741926u32,
    KpEquals = 1073741927u32,
    //F13 = 1073741928u32,
    //F14 = 1073741929u32,
    //F15 = 1073741930u32,
    //F16 = 1073741931u32,
    //F17 = 1073741932u32,
    //F18 = 1073741933u32,
    //F19 = 1073741934u32,
    //F20 = 1073741935u32,
    //F21 = 1073741936u32,
    //F22 = 1073741937u32,
    //F23 = 1073741938u32,
    //F24 = 1073741939u32,

    //Execute = 1073741940u32,
    //Help = 1073741941u32,
    //Menu = 1073741942u32,
    //Select = 1073741943u32,
    //Stop = 1073741944u32,
    //Again = 1073741945u32,

    Undo = 1073741946u32,
    Cut = 1073741947u32,
    Copy = 1073741948u32,
    Paste = 1073741949u32,

    //Find = Keycode::Find as u32,

    Mute = 1073741950u32,
    VolumeUp = 1073741951u32,
    VolumeDown = 1073741952u32,

    KpComma = 1073741957u32,
    KpEqualsAS400 = 1073741958u32,

    //AltErase = 1073741977u32,
    //Sysreq = 1073741978u32,
    //Cancel = 1073741979u32,
    //Clear = 1073741980u32,
    //Prior = 1073741981u32,
    //Return2 = 1073741982u32,

    //Separator =  u32,
    //Out = u32,
    //Oper = u32,
    //ClearAgain = u32,
    //CrSel =  u32,
    //ExSel =  u32,
    //Kp00 =  u32,
    //Kp000 =  u32,
    //ThousandsSeparator =  u32,
    //DecimalSeparator =  u32,
    //CurrencyUnit =  u32,
    //CurrencySubUnit =  u32,
    //KpLeftParen =  u32,
    //KpRightParen =  u32,
    //KpLeftBrace =  u32,
    //KpRightBrace =  u32,
    //KpTab =  u32,
    //KpBackspace =  u32,
    //KpA =  u32,
    //KpB =  u32,
    //KpC =  u32,
    //KpD =  u32,
    //KpE =  u32,
    //KpF =  u32,
    //KpXor =  u32,
    //KpPower =  u32,
    //KpPercent =  u32,
    //KpLess =  u32,
    //KpGreater =  u32,
    //KpAmpersand =  u32,
    //KpDblAmpersand =  u32,
    //KpVerticalBar =  u32,
    //KpDblVerticalBar =  u32,
    //KpColon =  u32,
    //KpHash =  u32,
    //KpSpace =  u32,
    //KpAt =  u32,
    //KpExclam =  u32,
    //KpMemStore =  u32,
    //KpMemRecall =  u32,
    //KpMemClear =  u32,
    //KpMemAdd =  u32,
    //KpMemSubtract =  u32,
    //KpMemMultiply =  u32,
    //KpMemDivide =  u32,
    //KpPlusMinus =  u32,
    //KpClear =  u32,
    //KpClearEntry =  u32,
    //KpBinary =  u32,
    //KpOctal =  u32,
    //KpDecimal =  u32,
    //KpHexadecimal =  u32,



    LCtrl = 1073742048u32,
    LShift = 1073742049u32,
    LAlt = 1073742050u32,
    LGui = 1073742051u32,
    RCtrl = 1073742052u32,
    RShift = 1073742053u32,
    RAlt = 1073742054u32,
    RGui = 1073742055u32,

    Mode = 1073742081u32,
    AudioNext = 1073742082u32,
    AudioPrev = 1073742083u32,
    AudioStop = 1073742084u32,
    AudioPlay = 1073742085u32,
    AudioMute = 1073742086u32,
    MediaSelect = 1073742087u32,

    //Www =  u32,
    //Mail =  u32,
    //Calculator =  u32,
    //Computer =  u32,
    //AcSearch =  u32,
    //AcHome =  u32,
    //AcBack =  u32,
    //AcForward =  u32,
    //AcStop =  u32,
    //AcRefresh =  u32,
    //AcBookmarks =  u32,
    //BrightnessDown = u32,
    //BrightnessUp =  u32,
    //DisplaySwitch =  u32,
    //KbdIllumToggle =  u32,
    //KbdIllumDown =  u32,
    //KbdIllumUp =  u32,
    //Eject =  u32,
    //Sleep =  u32,
}


fn get_usize_from_mykey(keycode: MyKey) -> usize
{
    return keycode as usize;
    /*
    let key = match keycode
    {
        MyKey::InvalidKey => SdlKey::InvalidKey,
        MyKey::Backspace => SdlKey::Backspace,
        MyKey::Tab => SdlKey::Tab,
        MyKey::Return => SdlKey::Return,
        MyKey::Escape => SdlKey::Escape,
        MyKey::Space => SdlKey::Space,
        MyKey::Exclaim => SdlKey::Exclaim,
        MyKey::Quotedbl => SdlKey::Quotedbl,
        MyKey::Hash => SdlKey::Hash,
        MyKey::Dollar => SdlKey::Dollar,
        MyKey::Percent => SdlKey::Percent,
        MyKey::Ampersand => SdlKey::Ampersand,
        MyKey::Quote => SdlKey::Quote,
        MyKey::LeftParen => SdlKey::LeftParen,
        MyKey::RightParen => SdlKey::RightParen,
        MyKey::Asterisk => SdlKey::Asterisk,
        MyKey::Plus => SdlKey::Plus,
        MyKey::Comma => SdlKey::Comma,
        MyKey::Minus => SdlKey::Minus,
        MyKey::Period => SdlKey::Period,
        MyKey::Slash => SdlKey::Slash,
        MyKey::Num0 => SdlKey::Num0,
        MyKey::Num1 => SdlKey::Num1,
        MyKey::Num2 => SdlKey::Num2,
        MyKey::Num3 => SdlKey::Num3,
        MyKey::Num4 => SdlKey::Num4,
        MyKey::Num5 => SdlKey::Num5,
        MyKey::Num6 => SdlKey::Num6,
        MyKey::Num7 => SdlKey::Num7,
        MyKey::Num8 => SdlKey::Num8,
        MyKey::Num9 => SdlKey::Num9,
        MyKey::Colon => SdlKey::Colon,
        MyKey::Semicolon => SdlKey::Semicolon,
        MyKey::Less => SdlKey::Less,
        MyKey::Equals => SdlKey::Equals,
        MyKey::Greater => SdlKey::Greater,
        MyKey::Question => SdlKey::Question,
        MyKey::At => SdlKey::At,
        MyKey::LeftBracket => SdlKey::LeftBracket,
        MyKey::Backslash => SdlKey::Backslash,
        MyKey::RightBracket => SdlKey::RightBracket,
        MyKey::Caret => SdlKey::Caret,
        MyKey::Underscore => SdlKey::Underscore,
        MyKey::Backquote => SdlKey::Backquote,
        MyKey::A => SdlKey::A,
        MyKey::B => SdlKey::B,
        MyKey::C => SdlKey::C,
        MyKey::D => SdlKey::D,
        MyKey::E => SdlKey::E,
        MyKey::F => SdlKey::F,
        MyKey::G => SdlKey::G,
        MyKey::H => SdlKey::H,
        MyKey::I => SdlKey::I,
        MyKey::J => SdlKey::J,
        MyKey::K => SdlKey::K,
        MyKey::L => SdlKey::L,
        MyKey::M => SdlKey::M,
        MyKey::N => SdlKey::N,
        MyKey::O => SdlKey::O,
        MyKey::P => SdlKey::P,
        MyKey::Q => SdlKey::Q,
        MyKey::R => SdlKey::R,
        MyKey::S => SdlKey::S,
        MyKey::T => SdlKey::T,
        MyKey::U => SdlKey::U,
        MyKey::V => SdlKey::V,
        MyKey::W => SdlKey::W,
        MyKey::X => SdlKey::X,
        MyKey::Y => SdlKey::Y,
        MyKey::Z => SdlKey::Z,
        MyKey::Delete => SdlKey::Delete,
        MyKey::CapsLock => SdlKey::CapsLock,
        MyKey::F1 => SdlKey::F1,
        MyKey::F2 => SdlKey::F2,
        MyKey::F3 => SdlKey::F3,
        MyKey::F4 => SdlKey::F4,
        MyKey::F5 => SdlKey::F5,
        MyKey::F6 => SdlKey::F6,
        MyKey::F7 => SdlKey::F7,
        MyKey::F8 => SdlKey::F8,
        MyKey::F9 => SdlKey::F9,
        MyKey::F10 => SdlKey::F10,
        MyKey::F11 => SdlKey::F11,
        MyKey::F12 => SdlKey::F12,
        MyKey::PrintScreen => SdlKey::PrintScreen,
        MyKey::ScrollLock => SdlKey::ScrollLock,
        MyKey::Pause => SdlKey::Pause,
        MyKey::Insert => SdlKey::Insert,
        MyKey::Home => SdlKey::Home,
        MyKey::PageUp => SdlKey::PageUp,
        MyKey::End => SdlKey::End,
        MyKey::PageDown => SdlKey::PageDown,
        MyKey::Right => SdlKey::Right,
        MyKey::Left => SdlKey::Left,
        MyKey::Down => SdlKey::Down,
        MyKey::Up => SdlKey::Up,
        MyKey::NumLockClear => SdlKey::NumLockClear,
        MyKey::KpDivide => SdlKey::KpDivide,
        MyKey::KpMultiply => SdlKey::KpMultiply,
        MyKey::KpMinus => SdlKey::KpMinus,
        MyKey::KpPlus => SdlKey::KpPlus,
        MyKey::KpEnter => SdlKey::KpEnter,
        MyKey::Kp1 => SdlKey::Kp1,
        MyKey::Kp2 => SdlKey::Kp2,
        MyKey::Kp3 => SdlKey::Kp3,
        MyKey::Kp4 => SdlKey::Kp4,
        MyKey::Kp5 => SdlKey::Kp5,
        MyKey::Kp6 => SdlKey::Kp6,
        MyKey::Kp7 => SdlKey::Kp7,
        MyKey::Kp8 => SdlKey::Kp8,
        MyKey::Kp9 => SdlKey::Kp9,
        MyKey::Kp0 => SdlKey::Kp0,
        MyKey::KpPeriod => SdlKey::KpPeriod,
        MyKey::KpEquals => SdlKey::KpEquals,

        MyKey::Undo => SdlKey::Undo,
        MyKey::Cut => SdlKey::Cut,
        MyKey::Copy => SdlKey::Copy,
        MyKey::Paste => SdlKey::Paste,

        MyKey::Mute => SdlKey::Mute,
        MyKey::VolumeUp => SdlKey::VolumeUp,
        MyKey::VolumeDown => SdlKey::VolumeDown,

        MyKey::KpComma => SdlKey::KpComma,
        MyKey::KpEqualsAS400 => SdlKey::KpEqualsAS400,

        MyKey::LCtrl => SdlKey::LCtrl,
        MyKey::LShift => SdlKey::LShift,
        MyKey::LAlt => SdlKey::LAlt,
        MyKey::LGui => SdlKey::LGui,
        MyKey::RCtrl => SdlKey::RCtrl,
        MyKey::RShift => SdlKey::RShift,
        MyKey::RAlt => SdlKey::RAlt,
        MyKey::RGui => SdlKey::RGui,

        MyKey::Mode => SdlKey::Mode,
        MyKey::AudioNext => SdlKey::AudioNext,
        MyKey::AudioPrev => SdlKey::AudioPrev,
        MyKey::AudioStop => SdlKey::AudioStop,
        MyKey::AudioPlay => SdlKey::AudioPlay,
        MyKey::AudioMute => SdlKey::AudioMute,
        MyKey::MediaSelect => SdlKey::MediaSelect,
        _ => SdlKey::InvalidKey
    };

    let val:u32 = unsafe
    {
        std::mem::transmute::<SdlKey, u32>(key)
    };
    //if (val & 0x40000000) == 0x40000000
    //{
    //    val =  (val & 0xffffu32) + 128u32;
    //}
        */
//    return val as usize;
}

fn get_usize_from_keycode(keycode: Keycode) -> usize
{
    let val:SdlKey = unsafe
    {
        std::mem::transmute::<Keycode, SdlKey>(keycode).try_into().unwrap()
    };

    let val = match val
    {
        SdlKey::InvalidKey => MyKey::InvalidKey,
        SdlKey::Backspace => MyKey::Backspace,
        SdlKey::Tab => MyKey::Tab,
        SdlKey::Return => MyKey::Return,
        SdlKey::Escape => MyKey::Escape,
        SdlKey::Space => MyKey::Space,
        SdlKey::Exclaim => MyKey::Exclaim,
        SdlKey::Quotedbl => MyKey::Quotedbl,
        SdlKey::Hash => MyKey::Hash,
        SdlKey::Dollar => MyKey::Dollar,
        SdlKey::Percent => MyKey::Percent,
        SdlKey::Ampersand => MyKey::Ampersand,
        SdlKey::Quote => MyKey::Quote,
        SdlKey::LeftParen => MyKey::LeftParen,
        SdlKey::RightParen => MyKey::RightParen,
        SdlKey::Asterisk => MyKey::Asterisk,
        SdlKey::Plus => MyKey::Plus,
        SdlKey::Comma => MyKey::Comma,
        SdlKey::Minus => MyKey::Minus,
        SdlKey::Period => MyKey::Period,
        SdlKey::Slash => MyKey::Slash,
        SdlKey::Num0 => MyKey::Num0,
        SdlKey::Num1 => MyKey::Num1,
        SdlKey::Num2 => MyKey::Num2,
        SdlKey::Num3 => MyKey::Num3,
        SdlKey::Num4 => MyKey::Num4,
        SdlKey::Num5 => MyKey::Num5,
        SdlKey::Num6 => MyKey::Num6,
        SdlKey::Num7 => MyKey::Num7,
        SdlKey::Num8 => MyKey::Num8,
        SdlKey::Num9 => MyKey::Num9,
        SdlKey::Colon => MyKey::Colon,
        SdlKey::Semicolon => MyKey::Semicolon,
        SdlKey::Less => MyKey::Less,
        SdlKey::Equals => MyKey::Equals,
        SdlKey::Greater => MyKey::Greater,
        SdlKey::Question => MyKey::Question,
        SdlKey::At => MyKey::At,
        SdlKey::LeftBracket => MyKey::LeftBracket,
        SdlKey::Backslash => MyKey::Backslash,
        SdlKey::RightBracket => MyKey::RightBracket,
        SdlKey::Caret => MyKey::Caret,
        SdlKey::Underscore => MyKey::Underscore,
        SdlKey::Backquote => MyKey::Backquote,
        SdlKey::A => MyKey::A,
        SdlKey::B => MyKey::B,
        SdlKey::C => MyKey::C,
        SdlKey::D => MyKey::D,
        SdlKey::E => MyKey::E,
        SdlKey::F => MyKey::F,
        SdlKey::G => MyKey::G,
        SdlKey::H => MyKey::H,
        SdlKey::I => MyKey::I,
        SdlKey::J => MyKey::J,
        SdlKey::K => MyKey::K,
        SdlKey::L => MyKey::L,
        SdlKey::M => MyKey::M,
        SdlKey::N => MyKey::N,
        SdlKey::O => MyKey::O,
        SdlKey::P => MyKey::P,
        SdlKey::Q => MyKey::Q,
        SdlKey::R => MyKey::R,
        SdlKey::S => MyKey::S,
        SdlKey::T => MyKey::T,
        SdlKey::U => MyKey::U,
        SdlKey::V => MyKey::V,
        SdlKey::W => MyKey::W,
        SdlKey::X => MyKey::X,
        SdlKey::Y => MyKey::Y,
        SdlKey::Z => MyKey::Z,
        SdlKey::Delete => MyKey::Delete,
        SdlKey::CapsLock => MyKey::CapsLock,
        SdlKey::F1 => MyKey::F1,
        SdlKey::F2 => MyKey::F2,
        SdlKey::F3 => MyKey::F3,
        SdlKey::F4 => MyKey::F4,
        SdlKey::F5 => MyKey::F5,
        SdlKey::F6 => MyKey::F6,
        SdlKey::F7 => MyKey::F7,
        SdlKey::F8 => MyKey::F8,
        SdlKey::F9 => MyKey::F9,
        SdlKey::F10 => MyKey::F10,
        SdlKey::F11 => MyKey::F11,
        SdlKey::F12 => MyKey::F12,
        SdlKey::PrintScreen => MyKey::PrintScreen,
        SdlKey::ScrollLock => MyKey::ScrollLock,
        SdlKey::Pause => MyKey::Pause,
        SdlKey::Insert => MyKey::Insert,
        SdlKey::Home => MyKey::Home,
        SdlKey::PageUp => MyKey::PageUp,
        SdlKey::End => MyKey::End,
        SdlKey::PageDown => MyKey::PageDown,
        SdlKey::Right => MyKey::Right,
        SdlKey::Left => MyKey::Left,
        SdlKey::Down => MyKey::Down,
        SdlKey::Up => MyKey::Up,
        SdlKey::NumLockClear => MyKey::NumLockClear,
        SdlKey::KpDivide => MyKey::KpDivide,
        SdlKey::KpMultiply => MyKey::KpMultiply,
        SdlKey::KpMinus => MyKey::KpMinus,
        SdlKey::KpPlus => MyKey::KpPlus,
        SdlKey::KpEnter => MyKey::KpEnter,
        SdlKey::Kp1 => MyKey::Kp1,
        SdlKey::Kp2 => MyKey::Kp2,
        SdlKey::Kp3 => MyKey::Kp3,
        SdlKey::Kp4 => MyKey::Kp4,
        SdlKey::Kp5 => MyKey::Kp5,
        SdlKey::Kp6 => MyKey::Kp6,
        SdlKey::Kp7 => MyKey::Kp7,
        SdlKey::Kp8 => MyKey::Kp8,
        SdlKey::Kp9 => MyKey::Kp9,
        SdlKey::Kp0 => MyKey::Kp0,
        SdlKey::KpPeriod => MyKey::KpPeriod,
        //Application = 1073741925i32,
        //Power = 1073741926i32,
        SdlKey::KpEquals => MyKey::KpEquals,
        //F13 = 1073741928i32,
        //F14 = 1073741929i32,
        //F15 = 1073741930i32,
        //F16 = 1073741931i32,
        //F17 = 1073741932i32,
        //F18 = 1073741933i32,
        //F19 = 1073741934i32,
        //F20 = 1073741935i32,
        //F21 = 1073741936i32,
        //F22 = 1073741937i32,
        //F23 = 1073741938i32,
        //F24 = 1073741939i32,

        //Execute = 1073741940i32,
        //Help = 1073741941i32,
        //Menu = 1073741942i32,
        //Select = 1073741943i32,
        //Stop = 1073741944i32,
        //Again = 1073741945i32,

        SdlKey::Undo => MyKey::Undo,
        SdlKey::Cut => MyKey::Cut,
        SdlKey::Copy => MyKey::Copy,
        SdlKey::Paste => MyKey::Paste,

        //Find = Keycode::Find as i32,

        SdlKey::Mute => MyKey::Mute,
        SdlKey::VolumeUp => MyKey::VolumeUp,
        SdlKey::VolumeDown => MyKey::VolumeDown,

        SdlKey::KpComma => MyKey::KpComma,
        SdlKey::KpEqualsAS400 => MyKey::KpEqualsAS400,

        //AltErase = 1073741977i32,
        //Sysreq = 1073741978i32,
        //Cancel = 1073741979i32,
        //Clear = 1073741980i32,
        //Prior = 1073741981i32,
        //Return2 = 1073741982i32,

        //Separator =  i32,
        //Out = i32,
        //Oper = i32,
        //ClearAgain = i32,
        //CrSel =  i32,
        //ExSel =  i32,
        //Kp00 =  i32,
        //Kp000 =  i32,
        //ThousandsSeparator =  i32,
        //DecimalSeparator =  i32,
        //CurrencyUnit =  i32,
        //CurrencySubUnit =  i32,
        //KpLeftParen =  i32,
        //KpRightParen =  i32,
        //KpLeftBrace =  i32,
        //KpRightBrace =  i32,
        //KpTab =  i32,
        //KpBackspace =  i32,
        //KpA =  i32,
        //KpB =  i32,
        //KpC =  i32,
        //KpD =  i32,
        //KpE =  i32,
        //KpF =  i32,
        //KpXor =  i32,
        //KpPower =  i32,
        //KpPercent =  i32,
        //KpLess =  i32,
        //KpGreater =  i32,
        //KpAmpersand =  i32,
        //KpDblAmpersand =  i32,
        //KpVerticalBar =  i32,
        //KpDblVerticalBar =  i32,
        //KpColon =  i32,
        //KpHash =  i32,
        //KpSpace =  i32,
        //KpAt =  i32,
        //KpExclam =  i32,
        //KpMemStore =  i32,
        //KpMemRecall =  i32,
        //KpMemClear =  i32,
        //KpMemAdd =  i32,
        //KpMemSubtract =  i32,
        //KpMemMultiply =  i32,
        //KpMemDivide =  i32,
        //KpPlusMinus =  i32,
        //KpClear =  i32,
        //KpClearEntry =  i32,
        //KpBinary =  i32,
        //KpOctal =  i32,
        //KpDecimal =  i32,
        //KpHexadecimal =  i32,



        SdlKey::LCtrl => MyKey::LCtrl,
        SdlKey::LShift => MyKey::LShift,
        SdlKey::LAlt => MyKey::LAlt,
        SdlKey::LGui => MyKey::LGui,
        SdlKey::RCtrl => MyKey::RCtrl,
        SdlKey::RShift => MyKey::RShift,
        SdlKey::RAlt => MyKey::RAlt,
        SdlKey::RGui => MyKey::RGui,

        SdlKey::Mode => MyKey::Mode,
        SdlKey::AudioNext => MyKey::AudioNext,
        SdlKey::AudioPrev => MyKey::AudioPrev,
        SdlKey::AudioStop => MyKey::AudioStop,
        SdlKey::AudioPlay => MyKey::AudioPlay,
        SdlKey::AudioMute => MyKey::AudioMute,
        SdlKey::MediaSelect => MyKey::MediaSelect,

        //Www =  i32,
        //Mail =  i32,
        //Calculator =  i32,
        //Computer =  i32,
        //AcSearch =  i32,
        //AcHome =  i32,
        //AcBack =  i32,
        //AcForward =  i32,
        //AcStop =  i32,
        //AcRefresh =  i32,
        //AcBookmarks =  i32,
        //BrightnessDown = i32,
        //BrightnessUp =  i32,
        //DisplaySwitch =  i32,
        //KbdIllumToggle =  i32,
        //KbdIllumDown =  i32,
        //KbdIllumUp =  i32,
        //Eject =  i32,
        //Sleep =  i32,
        _ => MyKey::InvalidKey
    };
    return val as usize;
    /*
    let mut val:u32 = unsafe
    {
        std::mem::transmute::<Keycode, u32>(keycode)
    };
    if (val & 0x40000000) == 0x40000000
    {
        val =  (val & 0xffffu32) + 128u32;
    }

    return val as usize;
    */
}


pub struct App
{
    pub window_state: window_state::WindowState,

    _sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    sdl_timer: sdl2::TimerSubsystem,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,

    //gl: *const std::os::raw::c_void,
    _gl_context: sdl2::video::GLContext,

}


impl App
{
    pub fn load_fn(&self, proc: &'static str) -> *const std::os::raw::c_void
    {
        return self.video.gl_get_proc_address(proc) as _;
    }


    pub fn init(window_width: i32, window_height: i32, window_name: &str, vsync: bool) -> Result<Self, String>
    {

        /*
        if width == 1
        {
            return Err("failed to initialize".to_string());
        }
*/
        let sdl: sdl2::Sdl  = sdl2::init().unwrap();
        let video: sdl2::VideoSubsystem = sdl.video().unwrap();
        let sdl_timer: sdl2::TimerSubsystem = sdl.timer().unwrap();
        let window;
        match video.window(window_name, window_width as u32, window_height as u32)
        .resizable()
        .opengl()
        .build()
        {
            Ok(v) =>
            {
                window = v;
            }
            Err(e) =>
            {
                println!("Error: {}", e);
                return Err("Failed to build window!".to_string());
            }
        }

        let gl_attr = video.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 6);

        gl_attr.set_context_flags().debug().set();
        gl_attr.set_framebuffer_srgb_compatible(true);

        let _gl_context = window.gl_create_context()?;

        /*

        let _gl = gl::load_with(&|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);


        unsafe
        {
            //gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(gl_callback), std::ptr::null());
            gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0,
                                        std::ptr::null(), gl::TRUE);
            gl::Enable(gl::DEBUG_OUTPUT);
        }

        let version;
        match unsafe
        {
            let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
                .to_bytes()
                .to_vec();
            String::from_utf8(data)
        }
        {
            Ok(v) =>
            {
                version = v;
            }
            Err(e) =>
            {
                println!("Error: {}", e);
                return Err("Failed to read version data from gl!".to_string());
            }
        }

        println!("OpenGL version {}", version);


        unsafe
        {
            gl::Viewport(0, 0, window_width, window_height);
            gl::ClearColor(0.2, 0.3, 0.5, 1.0);
            gl::ClearDepth(1.0);
            // Swapping up and down just messes things up like in renderdoc....
            //gl::ClipControl(gl::UPPER_LEFT, gl::ZERO_TO_ONE);
            gl::ClipControl(gl::LOWER_LEFT, gl::ZERO_TO_ONE);
        }
*/
        let event_pump = sdl.event_pump()?;
        let mut t = Self{ window_state: window_state::WindowState {
            window_width, window_height, vsync,
            timer: window_state::MyTimer{ now_stamp: sdl_timer.performance_counter() as u128,
                 last_stamp: sdl_timer.performance_counter() as u128,
                 perf_freq: sdl_timer.performance_frequency() as f64,
                 dt: 0.0f64
            },
            key_downs: [0; 512], key_downs_previous: [0; 512], key_half_count: [0; 512],
            mouse_x: 0, mouse_y: 0, mouse_b: 0,
            quit: false, resized: false, },

            _sdl: sdl, video, sdl_timer, window, event_pump, _gl_context,
        };
        t.enable_vsync(vsync)?;

        return Ok(t);
    }

    pub fn enable_vsync(&mut self, enable_vsync: bool) -> Result<(), String>
    {
        if enable_vsync
        {
            self.video.gl_set_swap_interval(sdl2::video::SwapInterval::VSync)?;
        }
        else
        {
            self.video.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate)?;
        }

        self.window_state.vsync = enable_vsync;
        return Ok(());
    }

    pub fn was_pressed(&self, key_code: MyKey) -> bool
    {
        return self.window_state.was_pressed(key_code);
    }

    pub fn was_released(&self, key_code: MyKey) -> bool
    {
        return self.window_state.was_released(key_code);
    }

    pub fn is_down(&self, key_code: MyKey)  -> bool
    {
        return self.window_state.is_down(key_code);
    }

    pub fn swap_buffer(&mut self)
    {
        self.window.gl_swap_window();
    }

    pub fn set_window_title(&mut self, title:&String )
    {
        self.window.set_title(&title).unwrap();
    }

    pub fn update(&mut self)
    {
        let ref mut timer = self.window_state.timer;
        timer.last_stamp = timer.now_stamp;
        timer.now_stamp = self.sdl_timer.performance_counter() as u128;
        timer.dt = (timer.now_stamp - timer.last_stamp) as f64 * 1000.0f64 / timer.perf_freq;

        self.window_state.reset();

        for event in self.event_pump.poll_iter()
        {
            match event
            {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    self.window_state.quit = true;
                },
                Event::KeyDown { keycode, .. } =>
                {
                    match keycode
                    {
                        Some(x) =>
                        {
                            let index = get_usize_from_keycode(x);
                            if index < 512
                            {
                                self.window_state.key_half_count[ index ] += 1u8;
                                self.window_state.key_downs[ index ] = 1u8;
                            }
                            println!("index pressed: {}", index);
                        },
                        _ => {}
                    }
                },
                Event::KeyUp { keycode, .. } =>
                {
                    match keycode
                    {
                        Some(x) =>
                        {
                            let index = get_usize_from_keycode(x);
                            if index < 512
                            {
                                self.window_state.key_half_count[ index ] += 1u8;
                                self.window_state.key_downs[ index ] = 0u8;
                            }
                            //println!("index relased: {}", index);
                        },
                        _ => {}
                    }
                },

                Event::Window {win_event, ..  } =>
                {
                    match win_event
                    {
                        sdl2::event::WindowEvent::Resized( width, height ) =>
                        {
                            self.window_state.resized = self.window_state.window_width != width || self.window_state.window_height != height;
                            self.window_state.window_width = width;
                            self.window_state.window_height = height;
                        },

                        _ => {}
                    }
                },


                Event::MouseButtonDown { mouse_btn, x, y, .. } =>
                {
                    self.window_state.mouse_x = x;
                    self.window_state.mouse_y = self.window_state.window_height as i32 - y;
                    if mouse_btn == MouseButton::Left
                    {
                        self.window_state.mouse_b |= 1;
                    }
                    else if mouse_btn == MouseButton::Right
                    {
                        self.window_state.mouse_b |= 2;
                    }
                },
                Event::MouseButtonUp { mouse_btn, x, y, .. } =>
                {
                    self.window_state.mouse_x = x;
                    self.window_state.mouse_y = self.window_state.window_height as i32 - y;
                    if mouse_btn == MouseButton::Left
                    {
                        self.window_state.mouse_b &= !1;
                    }
                    else if mouse_btn == MouseButton::Right
                    {
                        self.window_state.mouse_b &= !2;
                    }
                },
                Event::MouseMotion { x, y, .. } =>
                {
                    self.window_state.mouse_x = x;
                    self.window_state.mouse_y = self.window_state.window_height - y;
                },

                _ => {}
            }
        }
    }
}
