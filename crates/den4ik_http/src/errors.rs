use std::fmt;

use crate::frame::FRAME_HEADER_SIZE;

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
    InvalidHeaderSize(u8),
    InvalidPayloadSize(u8, u8),
    UnknownType(u8),
    InvalidLength,
    InvalidSettingsLength(u32),
    InvalidWindowUpdateLength(u32),
    SettingParse(SettingParseError),
}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameError::InvalidHeaderSize(got) => {
                write!(
                    f,
                    "invalid frame header size - expected: {FRAME_HEADER_SIZE}, got: {got}"
                )
            }
            FrameError::InvalidPayloadSize(expected, got) => {
                write!(
                    f,
                    "invalid frame data size - expected: {expected}, got: {got}"
                )
            }
            FrameError::UnknownType(t) => write!(f, "unknown frame type: {t:#x}"),
            FrameError::InvalidLength => write!(f, "invalid frame length"),
            FrameError::InvalidSettingsLength(l) => write!(f, "invalid settings frame length: {l}"),
            FrameError::InvalidWindowUpdateLength(l) => {
                write!(f, "invalid window update frame length: {l}")
            }
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
