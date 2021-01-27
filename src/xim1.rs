use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::error::Error;
use winapi::um::winuser::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum XIMStatus {
    Ok = 0,
    ParamInvalid = 100,
    CableNotFound = 200,
    CableConnectionFailed = 201,
    AlreadyConnected = 202,
    NotConnected = 203,
    TransferFailure = 204,
    Unknown,
    Uninitialized,
}

pub type XIMSTATUS = ::std::os::raw::c_int;
pub type MouseDelta = ::std::os::raw::c_short;
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct XIMMouseInput {
    pub Buttons: [bool; 24usize],
    pub DeltaX: MouseDelta,
    pub DeltaY: MouseDelta,
    pub Wheel: MouseDelta,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(usize)]
pub enum MouseButton {
    ButtonLeft = 0,
    ButtonRight = 1,
    ButtonMiddle = 2,
    ButtonBack = 3,
    ButtonForward = 4,
    ButtonAux1 = 5,
    ButtonAux2 = 6,
    ButtonAux3 = 7,
    ButtonAux4 = 8,
    ButtonAux5 = 9,
    ButtonAux6 = 10,
    ButtonAux7 = 11,
    ButtonAux8 = 12,
    ButtonAux9 = 13,
    ButtonAux10 = 14,
    ButtonAux11 = 15,
    ButtonAux12 = 16,
    ButtonAux13 = 17,
    ButtonAux14 = 18,
    ButtonAux15 = 19,
    ButtonAux16 = 20,
    ButtonAux17 = 21,
    ButtonPanLeft = 22,
    ButtonPanRight = 23,
    ButtonCount = 24,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMKeyboardInput {
    pub Buttons: [bool; 256usize],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum KeyboardButton {
    ButtonReserved0 = 0,
    ButtonErrorRolOver = 1,
    ButtonPOSTFail = 2,
    ButtonErrorUndef = 3,
    ButtonA = 4,
    ButtonB = 5,
    ButtonC = 6,
    ButtonD = 7,
    ButtonE = 8,
    ButtonF = 9,
    ButtonG = 10,
    ButtonH = 11,
    ButtonI = 12,
    ButtonJ = 13,
    ButtonK = 14,
    ButtonL = 15,
    ButtonM = 16,
    ButtonN = 17,
    ButtonO = 18,
    ButtonP = 19,
    ButtonQ = 20,
    ButtonR = 21,
    ButtonS = 22,
    ButtonT = 23,
    ButtonU = 24,
    ButtonV = 25,
    ButtonW = 26,
    ButtonX = 27,
    ButtonY = 28,
    ButtonZ = 29,
    Button1 = 30,
    Button2 = 31,
    Button3 = 32,
    Button4 = 33,
    Button5 = 34,
    Button6 = 35,
    Button7 = 36,
    Button8 = 37,
    Button9 = 38,
    Button0 = 39,
    ButtonEnter = 40,
    ButtonEscape = 41,
    ButtonBackspace = 42,
    ButtonTab = 43,
    ButtonSpace = 44,
    ButtonMinus = 45,
    ButtonEquals = 46,
    ButtonLeftBracket = 47,
    ButtonRightBracket = 48,
    ButtonBackslash = 49,
    ButtonTilde = 50,
    ButtonSemicolon = 51,
    ButtonQuote = 52,
    ButtonReverseQuote = 53,
    ButtonComma = 54,
    ButtonPeriod = 55,
    ButtonSlash = 56,
    ButtonCapsLock = 57,
    ButtonF1 = 58,
    ButtonF2 = 59,
    ButtonF3 = 60,
    ButtonF4 = 61,
    ButtonF5 = 62,
    ButtonF6 = 63,
    ButtonF7 = 64,
    ButtonF8 = 65,
    ButtonF9 = 66,
    ButtonF10 = 67,
    ButtonF11 = 68,
    ButtonF12 = 69,
    ButtonPrintScreen = 70,
    ButtonScrollLock = 71,
    ButtonPause = 72,
    ButtonInsert = 73,
    ButtonHome = 74,
    ButtonPageUp = 75,
    ButtonDelete = 76,
    ButtonEnd = 77,
    ButtonPageDown = 78,
    ButtonRightArrow = 79,
    ButtonLeftArrow = 80,
    ButtonDownArrow = 81,
    ButtonUpArrow = 82,
    ButtonKeypadNumLock = 83,
    ButtonKeypadSlash = 84,
    ButtonKeypadAsterisk = 85,
    ButtonKeypadMinus = 86,
    ButtonKeypadPlus = 87,
    ButtonKeypadEnter = 88,
    ButtonKeypad1 = 89,
    ButtonKeypad2 = 90,
    ButtonKeypad3 = 91,
    ButtonKeypad4 = 92,
    ButtonKeypad5 = 93,
    ButtonKeypad6 = 94,
    ButtonKeypad7 = 95,
    ButtonKeypad8 = 96,
    ButtonKeypad9 = 97,
    ButtonKeypad0 = 98,
    ButtonKeypadPeriod = 99,
    ButtonPipe = 100,
    ButtonApplication = 101,
    ButtonPower = 102,
    ButtonKeypadEquals = 103,
    ButtonF13 = 104,
    ButtonF14 = 105,
    ButtonF15 = 106,
    ButtonF16 = 107,
    ButtonF17 = 108,
    ButtonF18 = 109,
    ButtonF19 = 110,
    ButtonF20 = 111,
    ButtonF21 = 112,
    ButtonF22 = 113,
    ButtonF23 = 114,
    ButtonF24 = 115,
    ButtonExecute = 116,
    ButtonHelp = 117,
    ButtonMenu = 118,
    ButtonSelect = 119,
    ButtonStop = 120,
    ButtonAgain = 121,
    ButtonUndo = 122,
    ButtonCut = 123,
    ButtonCopy = 124,
    ButtonPaste = 125,
    ButtonFind = 126,
    ButtonMute = 127,
    ButtonVolumeUp = 128,
    ButtonVolumeDown = 129,
    ButtonLockingCapsLock = 130,
    ButtonLockingNumLock = 131,
    ButtonLockingScrollLock = 132,
    ButtonKeypadComma = 133,
    ButtonKeypadEqualSign = 134,
    ButtonKanji1 = 135,
    ButtonKanji2 = 136,
    ButtonKanji3 = 137,
    ButtonKanji4 = 138,
    ButtonKanji5 = 139,
    ButtonKanji6 = 140,
    ButtonKanji7 = 141,
    ButtonKanji8 = 142,
    ButtonKanji9 = 143,
    ButtonLANG1 = 144,
    ButtonLANG2 = 145,
    ButtonLANG3 = 146,
    ButtonLANG4 = 147,
    ButtonLANG5 = 148,
    ButtonLANG6 = 149,
    ButtonLANG7 = 150,
    ButtonLANG8 = 151,
    ButtonLANG9 = 152,
    ButtonAlternateErase = 153,
    ButtonSysReq = 154,
    ButtonCancel = 155,
    ButtonClear = 156,
    ButtonPrior = 157,
    ButtonReturn = 158,
    ButtonSeparator = 159,
    ButtonLeftControl = 224,
    ButtonLeftShift = 225,
    ButtonLeftAlt = 226,
    ButtonLeftGUI = 227,
    ButtonRightControl = 228,
    ButtonRightShift = 229,
    ButtonRightAlt = 230,
    ButtonRightGUI = 231,
    ButtonCount = 256,
    Noop = 257,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XIMJoystickInput {
    pub Buttons: [bool; 40usize],
    pub LeftStickX: ::std::os::raw::c_short,
    pub LeftStickY: ::std::os::raw::c_short,
    pub LeftStickZ: ::std::os::raw::c_short,
    pub RightStickX: ::std::os::raw::c_short,
    pub RightStickY: ::std::os::raw::c_short,
    pub RightStickZ: ::std::os::raw::c_short,
    pub LeftTrigger: ::std::os::raw::c_uchar,
    pub RightTrigger: ::std::os::raw::c_uchar,
    pub Slider: ::std::os::raw::c_uchar,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum JoystickButton {
    Button1 = 0,
    Button2 = 1,
    Button3 = 2,
    Button4 = 3,
    Button5 = 4,
    Button6 = 5,
    Button7 = 6,
    Button8 = 7,
    Button9 = 8,
    Button10 = 9,
    Button11 = 10,
    Button12 = 11,
    Button13 = 12,
    Button14 = 13,
    Button15 = 14,
    Button16 = 15,
    Button17 = 16,
    Button18 = 17,
    Button19 = 18,
    Button20 = 19,
    Button21 = 20,
    Button22 = 21,
    Button23 = 22,
    Button24 = 23,
    Button25 = 24,
    Button26 = 25,
    Button27 = 26,
    Button28 = 27,
    Button29 = 28,
    Button30 = 29,
    Button31 = 30,
    Button32 = 31,
    Button33 = 32,
    Button34 = 33,
    Button35 = 34,
    Button36 = 35,
    ButtonUp = 36,
    ButtonDown = 37,
    ButtonLeft = 38,
    ButtonRight = 39,
    ButtonCount = 40,
}
#[allow(dead_code)]
pub const STICK_REST: i16 = 0;
#[allow(dead_code)]
pub const STICK_MAXIMUM: i16 = 32767;
#[allow(dead_code)]
pub const STICK_MINIMUM: i16 = -32768;
#[allow(dead_code)]
pub const TRIGGER_REST: u8 = 0;
#[allow(dead_code)]
pub const TRIGGER_MAXIMUM: u8 = 255;
#[allow(dead_code)]
pub const SLIDER_MINIMUM: u8 = 0;
#[allow(dead_code)]
pub const SLIDER_MAXIMUM: u8 = 255;

#[allow(dead_code)]
#[link(name = "XIMCommander")]
extern "stdcall" {
    fn XIMConnect() -> XIMSTATUS;
    fn XIMDisconnect();
    fn XIMSendInput(
        mouse: *mut XIMMouseInput,
        keyboard: *mut XIMKeyboardInput,
        joystick: *mut XIMJoystickInput,
    ) -> XIMSTATUS;
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone)]
pub struct XIMManager {
   xim_keyboard: XIMKeyboardInput,
   xim_mouse: XIMMouseInput,

   pub status: XIMStatus
}
#[derive(Debug)]
pub struct XIMStatusError {status: XIMStatus}
impl XIMStatusError {
    pub fn new(status: XIMStatus) -> XIMStatusError {
        XIMStatusError{status: status}
    }
}
impl std::fmt::Display for XIMStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "XIMStatusError = {:?}", self.status)
    }
}

impl Error for XIMStatusError {} // Defaults are okay here.

#[derive(Debug, PartialEq, Eq)]
pub enum ButtonEvent {
    ButtonPressed,
    ButtonReleased,
    NOOP,
}

pub enum XIMEvent {
    MouseMoveEvent{x: MouseDelta, y: MouseDelta, wheel: MouseDelta},
    MouseButtonEvent(u16),
    KeyboardButton(u16, ButtonEvent)
}

type KB = KeyboardButton;
impl XIMManager {
    pub fn new() -> XIMManager {
        XIMManager {
            xim_keyboard: XIMKeyboardInput{Buttons: [false;256]},
            xim_mouse:  XIMMouseInput{Buttons:[false; 24],DeltaX: 0, DeltaY:0, Wheel: 0},
            status: XIMStatus::Uninitialized
        }
    }


    pub unsafe fn connect(&self) -> Result<XIMStatus, Box<dyn Error>> {
            if self.status != XIMStatus::Uninitialized {
                return Err(XIMStatusError::new(XIMStatus::Uninitialized).into());
            }
            let status = XIMConnect();
            
            match XIMStatus::try_from(status) {
                Ok(XIMStatus::Ok) => Ok(XIMStatus::Ok),
                Ok(status) => Err(XIMStatusError::new(status).into()),
                Err(e) => Err(e.into()),
            }
    }
    
    pub unsafe fn send_input(&mut self, input: XIMEvent) -> XIMStatus {
            self.xim_mouse.DeltaX = 0;
            self.xim_mouse.DeltaY = 0;
            self.translate(input);
            let status = XIMSendInput( &mut self.xim_mouse, &mut self.xim_keyboard, std::ptr::null_mut());
            XIMStatus::try_from(status).unwrap()
    }
    
    pub fn translate(&mut self, input: XIMEvent) {
        match &input {
            XIMEvent::MouseMoveEvent { x, y, wheel } => self.translate_mouse_move(x,y,wheel),
            XIMEvent::KeyboardButton(button, event) => self.translate_keyboard_button_event(*button as i32, event),
            XIMEvent::MouseButtonEvent(us_button_flags) => self.translate_mouse_button_event(*us_button_flags),
        }
    }

    fn translate_mouse_button_event(&mut self, us_button_flags: u16) {
        match us_button_flags{
            x if x & RI_MOUSE_LEFT_BUTTON_DOWN == RI_MOUSE_LEFT_BUTTON_DOWN => self.xim_mouse.Buttons[0] = true,
            x if x & RI_MOUSE_LEFT_BUTTON_UP == RI_MOUSE_LEFT_BUTTON_UP =>self.xim_mouse.Buttons[0] = false,
            x if x & RI_MOUSE_RIGHT_BUTTON_DOWN == RI_MOUSE_RIGHT_BUTTON_DOWN => self.xim_mouse.Buttons[1] = true,
            x if x & RI_MOUSE_RIGHT_BUTTON_UP == RI_MOUSE_RIGHT_BUTTON_UP =>self.xim_mouse.Buttons[1] = false,
            x if x & RI_MOUSE_MIDDLE_BUTTON_DOWN == RI_MOUSE_MIDDLE_BUTTON_DOWN => self.xim_mouse.Buttons[2] = true,
            x if x & RI_MOUSE_MIDDLE_BUTTON_UP == RI_MOUSE_MIDDLE_BUTTON_UP =>self.xim_mouse.Buttons[2] = false,
            _ => {}
        }
    }
    pub fn translate_keyboard_button_event(&mut self, button: i32, event: &ButtonEvent) {
        let xim_button = match button {
            VK_BACK => KB::ButtonBackspace,
            VK_TAB => KB::ButtonTab,
            VK_CLEAR => KB::ButtonClear,
            VK_RETURN => KB::ButtonReturn,
            VK_SHIFT => KB::ButtonLeftShift,
            VK_CONTROL => KB::ButtonLeftControl,
            VK_MENU => KB::ButtonLeftAlt,
            VK_CAPITAL => KB::ButtonCapsLock,
            VK_ESCAPE => KB::ButtonEscape,
            VK_SPACE => KB::ButtonSpace,
            VK_PRIOR => KB::ButtonPageUp,
            VK_NEXT => KB::ButtonPageDown,
            VK_END => KB::ButtonEnd,
            VK_HOME => KB::ButtonHome,
            VK_LEFT => KB::ButtonLeftArrow,
            VK_UP => KB::ButtonUpArrow,
            VK_RIGHT => KB::ButtonRightArrow,
            VK_DOWN => KB::ButtonDownArrow,
            VK_SELECT => KB::ButtonSelect,
            VK_PRINT => KB::ButtonPrintScreen,
            VK_EXECUTE => KB::ButtonExecute,
            VK_INSERT => KB::ButtonInsert,
            VK_DELETE => KB::ButtonDelete, 
            48 => KB::Button0,
            /* 49-57 => Buttons 1-9 handled below */
            /* 65-90 => a-z handled below*/
            VK_LWIN => KB::ButtonLeftGUI,
            VK_RWIN => KB::ButtonRightGUI,
            VK_HELP => KB::ButtonHelp,
            VK_NUMPAD0 => KB::ButtonKeypad0,
            /* 97 - 105 => numpage 1-9 handled below*/
            VK_MULTIPLY => KB::ButtonKeypadAsterisk,
            VK_ADD => KB::ButtonKeypadPlus,
            VK_SEPARATOR => KB::ButtonKeypadEnter,
            VK_SUBTRACT => KB::ButtonKeypadMinus,
            VK_DECIMAL => KB::ButtonKeypadPeriod,
            VK_DIVIDE => KB::ButtonKeypadSlash,
            VK_NUMLOCK => KB::ButtonKeypadNumLock,
            VK_SCROLL => KB::ButtonScrollLock,
            VK_LSHIFT => KB::ButtonLeftShift,
            VK_RSHIFT => KB::ButtonRightShift,
            VK_LCONTROL => KB::ButtonLeftControl,
            VK_RCONTROL => KB::ButtonRightControl,
            VK_LMENU => KB::ButtonLeftAlt,
            VK_RMENU => KB::ButtonRightAlt,
            VK_BROWSER_BACK => KB::Noop,
            VK_OEM_1 => KB::ButtonSemicolon,
            VK_OEM_PLUS => KB::ButtonEquals,
            VK_OEM_COMMA => KB::ButtonComma,
            VK_OEM_MINUS => KB::ButtonMinus,
            VK_OEM_PERIOD => KB::ButtonPeriod,
            VK_OEM_2 => KB::ButtonSlash,
            VK_OEM_3 => KB::ButtonReverseQuote,
            VK_OEM_4 => KB::ButtonLeftBracket,
            VK_OEM_5 => KB::ButtonBackslash,
            VK_OEM_6 => KB::ButtonRightBracket,
            VK_OEM_7 => KB::ButtonQuote,
          
            
            _ => KeyboardButton::Noop,
        };

        let mut xim_code: i32 = xim_button.into();

        // 0-9
        if button >= 49 && button <=57 {
            xim_code = button - 19;
        }

        // a-z
        if button >= 65 && button <=90 {
            xim_code = button - 61;
        }
        
        if button >= VK_NUMPAD1 && button <= VK_NUMPAD9 {
            xim_code = button - 8;
        }

        if button >= VK_F1 && button <= VK_F12 {
            xim_code = button - 53;
        }
        
        if button >= VK_F13 && button <= VK_F24 {
            xim_code = button + 20;
        }
        let index = xim_code as usize;

        if index < 256 {
           self.xim_keyboard.Buttons[index] = match event {
                ButtonEvent::ButtonPressed => true,
                ButtonEvent::ButtonReleased => false,
                ButtonEvent::NOOP => self.xim_keyboard.Buttons[index]
            };
        }
        
    }
    pub fn translate_mouse_move(&mut self, x: &MouseDelta, y: &MouseDelta, wheel: &MouseDelta) {
        self.xim_mouse.DeltaX = *x;
        self.xim_mouse.DeltaY = *y;
        self.xim_mouse.Wheel = *wheel;
    }
    pub fn disconnect(&self) {
        unsafe {
          
            XIMDisconnect();
           
        }
    }
}


