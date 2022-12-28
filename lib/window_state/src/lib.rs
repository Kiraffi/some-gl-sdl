pub struct MyTimer
{
    pub now_stamp: u128,
    pub last_stamp: u128,
    pub perf_freq: f64,
    pub dt: f64,
}

#[repr(C)]
pub struct WindowState
{
    pub window_width: i32,
    pub window_height: i32,
    pub vsync: bool,

    pub timer: MyTimer,

    pub key_downs: [u8; 512],
    pub key_downs_previous: [u8; 512],
    pub key_half_count: [u8; 512],

    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_b: i32,

    pub quit: bool,
    pub resized: bool,
}
impl WindowState
{
    pub fn new() -> Self
    {
        unsafe{ std::mem::zeroed() }
    }

    pub fn reset(&mut self)
    {
        self.key_half_count = [0; 512];
        self.key_downs_previous = self.key_downs;
    }

    pub fn was_pressed(&self, key_code: MyKey) -> bool
    {
        let index = key_code as usize;
        return index < 512 && ((self.key_downs[index] == 1u8 && self.key_downs_previous[index] == 0u8 ) ||
            self.key_half_count[index] >= 2u8);
    }

    pub fn was_released(&self, key_code: MyKey) -> bool
    {
        let index = key_code as usize;
        return index < 512 && ((self.key_downs[index] == 0u8 && self.key_downs_previous[index] == 1u8 ) ||
            self.key_half_count[index] >= 2u8);
    }

    pub fn is_down(&self, key_code: MyKey)  -> bool
    {
        let index = key_code as usize;
        return index < 512 && self.key_downs[index] == 1u8;
    }

}


#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum MyKey
{
    InvalidKey = -1i32,
    Backspace = 8i32,
    Tab = 9i32,
    Return = 13i32,
    Escape = 27i32,
    Space = 32i32,
    Exclaim = 33i32,
    Quotedbl = 34i32,
    Hash = 35i32,
    Dollar = 36i32,
    Percent = 37i32,
    Ampersand = 38i32,
    Quote = 39i32,
    LeftParen = 40i32,
    RightParen = 41i32,
    Asterisk = 42i32,
    Plus = 43i32,
    Comma = 44i32,
    Minus = 45i32,
    Period = 46i32,
    Slash = 47i32,
    Num0 = 48i32,
    Num1 = 49i32,
    Num2 = 50i32,
    Num3 = 51i32,
    Num4 = 52i32,
    Num5 = 53i32,
    Num6 = 54i32,
    Num7 = 55i32,
    Num8 = 56i32,
    Num9 = 57i32,
    Colon = 58i32,
    Semicolon = 59i32,
    Less = 60i32,
    Equals = 61i32,
    Greater = 62i32,
    Question = 63i32,
    At = 64i32,
    LeftBracket = 91i32,
    Backslash = 92i32,
    RightBracket = 93i32,
    Caret = 94i32,
    Underscore = 95i32,
    Backquote = 96i32,
    A = 97i32,
    B = 98i32,
    C = 99i32,
    D = 100i32,
    E = 101i32,
    F = 102i32,
    G = 103i32,
    H = 104i32,
    I = 105i32,
    J = 106i32,
    K = 107i32,
    L = 108i32,
    M = 109i32,
    N = 110i32,
    O = 111i32,
    P = 112i32,
    Q = 113i32,
    R = 114i32,
    S = 115i32,
    T = 116i32,
    U = 117i32,
    V = 118i32,
    W = 119i32,
    X = 120i32,
    Y = 121i32,
    Z = 122i32,
    Delete = 127i32,
    CapsLock = 128i32,
    F1 = 129i32,
    F2 = 130i32,
    F3 = 131i32,
    F4 = 132i32,
    F5 = 133i32,
    F6 = 134i32,
    F7 = 135i32,
    F8 = 136i32,
    F9 = 137i32,
    F10 = 138i32,
    F11 = 139i32,
    F12 = 140i32,

    PrintScreen = 160i32,
    ScrollLock = 161i32,
    Pause = 162i32,
    Insert = 163i32,
    Home = 164i32,
    PageUp = 165i32,
    End = 166i32,
    PageDown = 167i32,


    Right = 192i32,
    Left = 193i32,
    Down = 194i32,
    Up = 195i32,
    NumLockClear = 200i32,
    KpDivide = 201i32,
    KpMultiply = 202i32,
    KpMinus = 203i32,
    KpPlus = 204i32,
    KpEnter = 205i32,
    KpPeriod = 206i32,
    KpEquals = 207i32,
    KpComma = 208i32,
    Kp1 = 210i32,
    Kp2 = 211i32,
    Kp3 = 212i32,
    Kp4 = 213i32,
    Kp5 = 214i32,
    Kp6 = 215i32,
    Kp7 = 216i32,
    Kp8 = 217i32,
    Kp9 = 218i32,
    Kp0 = 219i32,
    //Application = 1073741925i32,
    //Power = 1073741926i32,
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

    Undo = 256i32,
    Cut = 257i32,
    Copy = 258i32,
    Paste = 259i32,

    //Find = Keycode::Find as i32,

    Mute = 300i32,
    VolumeUp = 301i32,
    VolumeDown = 302i32,

    KpEqualsAS400 = 310i32,

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



    LCtrl = 400i32,
    LShift = 401i32,
    LAlt = 402i32,
    LGui = 403i32,
    RCtrl = 404i32,
    RShift = 405i32,
    RAlt = 406i32,
    RGui = 407i32,

    Mode = 320i32,
    AudioNext = 321i32,
    AudioPrev = 322i32,
    AudioStop = 323i32,
    AudioPlay = 324i32,
    AudioMute = 325i32,
    MediaSelect = 326i32,

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
}
