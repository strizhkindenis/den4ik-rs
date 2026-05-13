use crate::ffi;
use crate::math::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub(crate) inner: ffi::Camera3D,
}

impl Camera {
    pub const fn new(
        position: Vector3,
        target: Vector3,
        up: Vector3,
        fovy: f32,
        projection: i32,
    ) -> Self {
        Self {
            inner: ffi::Camera3D {
                position: position.inner,
                target: target.inner,
                up: up.inner,
                fovy,
                projection,
            },
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vector3::default(),
            Vector3::default(),
            Vector3::new(0.0, 1.0, 0.0),
            45.0,
            0,
        )
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraMode {
    Custom = crate::ffi::CameraMode_CAMERA_CUSTOM as i32,
    Free = crate::ffi::CameraMode_CAMERA_FREE as i32,
    Orbital = crate::ffi::CameraMode_CAMERA_ORBITAL as i32,
    FirstPerson = crate::ffi::CameraMode_CAMERA_FIRST_PERSON as i32,
    ThirdPerson = crate::ffi::CameraMode_CAMERA_THIRD_PERSON as i32,
}

impl From<CameraMode> for i32 {
    fn from(val: CameraMode) -> Self {
        val as i32
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    Null = crate::ffi::KeyboardKey_KEY_NULL as i32,
    Apostrophe = crate::ffi::KeyboardKey_KEY_APOSTROPHE as i32,
    Comma = crate::ffi::KeyboardKey_KEY_COMMA as i32,
    Minus = crate::ffi::KeyboardKey_KEY_MINUS as i32,
    Period = crate::ffi::KeyboardKey_KEY_PERIOD as i32,
    Slash = crate::ffi::KeyboardKey_KEY_SLASH as i32,
    Zero = crate::ffi::KeyboardKey_KEY_ZERO as i32,
    One = crate::ffi::KeyboardKey_KEY_ONE as i32,
    Two = crate::ffi::KeyboardKey_KEY_TWO as i32,
    Three = crate::ffi::KeyboardKey_KEY_THREE as i32,
    Four = crate::ffi::KeyboardKey_KEY_FOUR as i32,
    Five = crate::ffi::KeyboardKey_KEY_FIVE as i32,
    Six = crate::ffi::KeyboardKey_KEY_SIX as i32,
    Seven = crate::ffi::KeyboardKey_KEY_SEVEN as i32,
    Eight = crate::ffi::KeyboardKey_KEY_EIGHT as i32,
    Nine = crate::ffi::KeyboardKey_KEY_NINE as i32,
    Semicolon = crate::ffi::KeyboardKey_KEY_SEMICOLON as i32,
    Equal = crate::ffi::KeyboardKey_KEY_EQUAL as i32,
    A = crate::ffi::KeyboardKey_KEY_A as i32,
    B = crate::ffi::KeyboardKey_KEY_B as i32,
    C = crate::ffi::KeyboardKey_KEY_C as i32,
    D = crate::ffi::KeyboardKey_KEY_D as i32,
    E = crate::ffi::KeyboardKey_KEY_E as i32,
    F = crate::ffi::KeyboardKey_KEY_F as i32,
    G = crate::ffi::KeyboardKey_KEY_G as i32,
    H = crate::ffi::KeyboardKey_KEY_H as i32,
    I = crate::ffi::KeyboardKey_KEY_I as i32,
    J = crate::ffi::KeyboardKey_KEY_J as i32,
    K = crate::ffi::KeyboardKey_KEY_K as i32,
    L = crate::ffi::KeyboardKey_KEY_L as i32,
    M = crate::ffi::KeyboardKey_KEY_M as i32,
    N = crate::ffi::KeyboardKey_KEY_N as i32,
    O = crate::ffi::KeyboardKey_KEY_O as i32,
    P = crate::ffi::KeyboardKey_KEY_P as i32,
    Q = crate::ffi::KeyboardKey_KEY_Q as i32,
    R = crate::ffi::KeyboardKey_KEY_R as i32,
    S = crate::ffi::KeyboardKey_KEY_S as i32,
    T = crate::ffi::KeyboardKey_KEY_T as i32,
    U = crate::ffi::KeyboardKey_KEY_U as i32,
    V = crate::ffi::KeyboardKey_KEY_V as i32,
    W = crate::ffi::KeyboardKey_KEY_W as i32,
    X = crate::ffi::KeyboardKey_KEY_X as i32,
    Y = crate::ffi::KeyboardKey_KEY_Y as i32,
    Z = crate::ffi::KeyboardKey_KEY_Z as i32,
    LeftBracket = crate::ffi::KeyboardKey_KEY_LEFT_BRACKET as i32,
    Backslash = crate::ffi::KeyboardKey_KEY_BACKSLASH as i32,
    RightBracket = crate::ffi::KeyboardKey_KEY_RIGHT_BRACKET as i32,
    Grave = crate::ffi::KeyboardKey_KEY_GRAVE as i32,
    Space = crate::ffi::KeyboardKey_KEY_SPACE as i32,
    Escape = crate::ffi::KeyboardKey_KEY_ESCAPE as i32,
    Enter = crate::ffi::KeyboardKey_KEY_ENTER as i32,
    Tab = crate::ffi::KeyboardKey_KEY_TAB as i32,
    Backspace = crate::ffi::KeyboardKey_KEY_BACKSPACE as i32,
    Insert = crate::ffi::KeyboardKey_KEY_INSERT as i32,
    Delete = crate::ffi::KeyboardKey_KEY_DELETE as i32,
    Right = crate::ffi::KeyboardKey_KEY_RIGHT as i32,
    Left = crate::ffi::KeyboardKey_KEY_LEFT as i32,
    Down = crate::ffi::KeyboardKey_KEY_DOWN as i32,
    Up = crate::ffi::KeyboardKey_KEY_UP as i32,
    PageUp = crate::ffi::KeyboardKey_KEY_PAGE_UP as i32,
    PageDown = crate::ffi::KeyboardKey_KEY_PAGE_DOWN as i32,
    Home = crate::ffi::KeyboardKey_KEY_HOME as i32,
    End = crate::ffi::KeyboardKey_KEY_END as i32,
    CapsLock = crate::ffi::KeyboardKey_KEY_CAPS_LOCK as i32,
    ScrollLock = crate::ffi::KeyboardKey_KEY_SCROLL_LOCK as i32,
    NumLock = crate::ffi::KeyboardKey_KEY_NUM_LOCK as i32,
    PrintScreen = crate::ffi::KeyboardKey_KEY_PRINT_SCREEN as i32,
    Pause = crate::ffi::KeyboardKey_KEY_PAUSE as i32,
    F1 = crate::ffi::KeyboardKey_KEY_F1 as i32,
    F2 = crate::ffi::KeyboardKey_KEY_F2 as i32,
    F3 = crate::ffi::KeyboardKey_KEY_F3 as i32,
    F4 = crate::ffi::KeyboardKey_KEY_F4 as i32,
    F5 = crate::ffi::KeyboardKey_KEY_F5 as i32,
    F6 = crate::ffi::KeyboardKey_KEY_F6 as i32,
    F7 = crate::ffi::KeyboardKey_KEY_F7 as i32,
    F8 = crate::ffi::KeyboardKey_KEY_F8 as i32,
    F9 = crate::ffi::KeyboardKey_KEY_F9 as i32,
    F10 = crate::ffi::KeyboardKey_KEY_F10 as i32,
    F11 = crate::ffi::KeyboardKey_KEY_F11 as i32,
    F12 = crate::ffi::KeyboardKey_KEY_F12 as i32,
    LeftShift = crate::ffi::KeyboardKey_KEY_LEFT_SHIFT as i32,
    LeftControl = crate::ffi::KeyboardKey_KEY_LEFT_CONTROL as i32,
    LeftAlt = crate::ffi::KeyboardKey_KEY_LEFT_ALT as i32,
    LeftSuper = crate::ffi::KeyboardKey_KEY_LEFT_SUPER as i32,
    RightShift = crate::ffi::KeyboardKey_KEY_RIGHT_SHIFT as i32,
    RightControl = crate::ffi::KeyboardKey_KEY_RIGHT_CONTROL as i32,
    RightAlt = crate::ffi::KeyboardKey_KEY_RIGHT_ALT as i32,
    RightSuper = crate::ffi::KeyboardKey_KEY_RIGHT_SUPER as i32,
    KbMenu = crate::ffi::KeyboardKey_KEY_KB_MENU as i32,
    Kp0 = crate::ffi::KeyboardKey_KEY_KP_0 as i32,
    Kp1 = crate::ffi::KeyboardKey_KEY_KP_1 as i32,
    Kp2 = crate::ffi::KeyboardKey_KEY_KP_2 as i32,
    Kp3 = crate::ffi::KeyboardKey_KEY_KP_3 as i32,
    Kp4 = crate::ffi::KeyboardKey_KEY_KP_4 as i32,
    Kp5 = crate::ffi::KeyboardKey_KEY_KP_5 as i32,
    Kp6 = crate::ffi::KeyboardKey_KEY_KP_6 as i32,
    Kp7 = crate::ffi::KeyboardKey_KEY_KP_7 as i32,
    Kp8 = crate::ffi::KeyboardKey_KEY_KP_8 as i32,
    Kp9 = crate::ffi::KeyboardKey_KEY_KP_9 as i32,
    KpDecimal = crate::ffi::KeyboardKey_KEY_KP_DECIMAL as i32,
    KpDivide = crate::ffi::KeyboardKey_KEY_KP_DIVIDE as i32,
    KpMultiply = crate::ffi::KeyboardKey_KEY_KP_MULTIPLY as i32,
    KpSubtract = crate::ffi::KeyboardKey_KEY_KP_SUBTRACT as i32,
    KpAdd = crate::ffi::KeyboardKey_KEY_KP_ADD as i32,
    KpEnter = crate::ffi::KeyboardKey_KEY_KP_ENTER as i32,
    KpEqual = crate::ffi::KeyboardKey_KEY_KP_EQUAL as i32,
    Back = crate::ffi::KeyboardKey_KEY_BACK as i32,
    Menu = crate::ffi::KeyboardKey_KEY_MENU as i32,
    VolumeUp = crate::ffi::KeyboardKey_KEY_VOLUME_UP as i32,
    VolumeDown = crate::ffi::KeyboardKey_KEY_VOLUME_DOWN as i32,
}

impl From<KeyboardKey> for i32 {
    fn from(val: KeyboardKey) -> Self {
        val as i32
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left = crate::ffi::MouseButton_MOUSE_BUTTON_LEFT as i32,
    Right = crate::ffi::MouseButton_MOUSE_BUTTON_RIGHT as i32,
    Middle = crate::ffi::MouseButton_MOUSE_BUTTON_MIDDLE as i32,
    Side = crate::ffi::MouseButton_MOUSE_BUTTON_SIDE as i32,
    Extra = crate::ffi::MouseButton_MOUSE_BUTTON_EXTRA as i32,
    Forward = crate::ffi::MouseButton_MOUSE_BUTTON_FORWARD as i32,
    Back = crate::ffi::MouseButton_MOUSE_BUTTON_BACK as i32,
}

impl From<MouseButton> for i32 {
    fn from(val: MouseButton) -> Self {
        val as i32
    }
}
