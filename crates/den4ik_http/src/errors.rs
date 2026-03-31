use std::fmt;

#[derive(Debug)]
pub enum SettingParseError {
    UnknownId(u16),
    InvalidValue(u16, u32),
}

impl fmt::Display for SettingParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingParseError::UnknownId(id) => write!(f, "unknown setting id: {id}"),
            SettingParseError::InvalidValue(id, val) => {
                write!(f, "invalid value {val} for setting id {id}")
            }
        }
    }
}

impl std::error::Error for SettingParseError {}

#[derive(Debug)]
pub enum FrameError {
    MissingLength,
    MissingType,
    MissingFlags,
    MissingSid,
    MissingPayload,
    UnknownType(u8),
    InvalidLength,
    InvalidSettingsLength(u32),
    SettingParse(SettingParseError),
}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameError::MissingLength => write!(f, "incomplete frame data: missing length"),
            FrameError::MissingType => write!(f, "incomplete frame data: missing type"),
            FrameError::MissingFlags => write!(f, "incomplete frame data: missing flags"),
            FrameError::MissingSid => {
                write!(f, "incomplete frame data: missing stream identification")
            }
            FrameError::MissingPayload => write!(f, "incomplete frame data: missing payload"),
            FrameError::UnknownType(t) => write!(f, "unknown frame type: {t:#x}"),
            FrameError::InvalidLength => write!(f, "invalid frame length"),
            FrameError::InvalidSettingsLength(l) => write!(f, "invalid settings frame length: {l}"),
            FrameError::SettingParse(e) => write!(f, "setting parse error: {e}"),
        }
    }
}

impl std::error::Error for FrameError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FrameError::SettingParse(e) => Some(e),
            _ => None,
        }
    }
}
