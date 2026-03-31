use crate::errors::{FrameError, SettingParseError};

const FRAME_LENGTH_SIZE: usize = 3;
const FRAME_TYPE_SIZE: usize = 1;
const FRAME_FLAGS_SIZE: usize = 1;
const FRAME_SID_SIZE: usize = 4;

#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    Data,
    Headers,
    Priority,
    RSTStream,
    Settings,
    PushPromise,
    Ping,
    Goaway,
    WindowUpdate,
    Continuation,
}

impl TryFrom<u8> for FrameType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(FrameType::Data),
            0x1 => Ok(FrameType::Headers),
            0x2 => Ok(FrameType::Priority),
            0x3 => Ok(FrameType::RSTStream),
            0x4 => Ok(FrameType::Settings),
            0x5 => Ok(FrameType::PushPromise),
            0x6 => Ok(FrameType::Ping),
            0x7 => Ok(FrameType::Goaway),
            0x8 => Ok(FrameType::WindowUpdate),
            0x9 => Ok(FrameType::Continuation),
            other => Err(other),
        }
    }
}

#[derive(Debug)]
pub enum FrameKind {
    Other(FrameOther),
    Settings(FrameSettings),
}

#[derive(Debug)]
pub struct FrameOther {
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct FrameSettings {
    pub settings: Vec<Setting>,
}

#[derive(Debug, Clone, Copy)]
pub enum Setting {
    HeaderTableSize(u32),
    EnablePush(bool),
    MaxConcurrentStreams(u32),
    InitialWindowSize(u32),
    MaxHeaderListSize(u32),
}

impl Setting {
    const SIZE: usize = 6;
}

impl TryFrom<(u16, u32)> for Setting {
    type Error = SettingParseError;

    fn try_from((id, value): (u16, u32)) -> Result<Self, Self::Error> {
        match id {
            0x1 => Ok(Setting::HeaderTableSize(value)),
            0x2 => match value {
                0 => Ok(Setting::EnablePush(false)),
                1 => Ok(Setting::EnablePush(true)),
                _ => Err(SettingParseError::InvalidValue(id, value)),
            },
            0x3 => Ok(Setting::MaxConcurrentStreams(value)),
            0x4 => {
                // RFC: values > 2^31-1 MUST be treated as connection error (FLOW_CONTROL_ERROR).
                // Here we validate that it fits in a signed 31-bit range (i.e., <= 2^31-1).
                if value > 0x7FFF_FFFF {
                    Err(SettingParseError::InvalidValue(id, value))
                } else {
                    Ok(Setting::InitialWindowSize(value))
                }
            }
            0x6 => Ok(Setting::MaxHeaderListSize(value)),
            other => Err(SettingParseError::UnknownId(other)),
        }
    }
}

#[derive(Debug)]
pub struct RawFrame<'a> {
    pub length: u32,
    pub _type: FrameType,
    pub flags: u8,
    pub sid: u32,
    pub data: &'a [u8],
}

impl RawFrame<'_> {
    fn into_settings(self) -> Result<Frame, FrameError> {
        assert!(matches!(self._type, FrameType::Settings));
        if !(self.length as usize).is_multiple_of(Setting::SIZE) {
            return Err(FrameError::InvalidSettingsLength(self.length));
        }
        let (settings_chunks, _) = self.data.as_chunks::<{ Setting::SIZE }>();
        let settings = settings_chunks
            .iter()
            .map(|chunk| {
                let id = u16::from_be_bytes([chunk[0], chunk[1]]);
                let val = u32::from_be_bytes([chunk[2], chunk[3], chunk[4], chunk[5]]);
                Setting::try_from((id, val))
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(FrameError::SettingParse)?;
        let kind = FrameKind::Settings(FrameSettings { settings });
        Ok(self.into_frame(kind))
    }

    fn into_other(self) -> Result<Frame, FrameError> {
        let kind = FrameKind::Other(FrameOther {
            data: self.data.to_vec(),
        });
        Ok(self.into_frame(kind))
    }

    fn into_frame(self, kind: FrameKind) -> Frame {
        if let FrameKind::Settings(_) = &kind {
            assert!(matches!(self._type, FrameType::Settings))
        }
        Frame {
            length: self.length,
            _type: self._type,
            flags: self.flags,
            sid: self.sid,
            kind,
        }
    }
}

impl<'a> TryFrom<RawFrame<'a>> for Frame {
    type Error = FrameError;

    fn try_from(raw_frame: RawFrame<'a>) -> Result<Self, Self::Error> {
        match raw_frame._type {
            FrameType::Settings => raw_frame.into_settings(),
            _ => raw_frame.into_other(),
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    length: u32,
    _type: FrameType,
    flags: u8,
    sid: u32,
    kind: FrameKind,
}

impl TryFrom<&[u8]> for Frame {
    type Error = FrameError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (length_bytes, buf) = value
            .split_first_chunk::<FRAME_LENGTH_SIZE>()
            .ok_or(FrameError::MissingLength)?;
        let length = u32::from_be_bytes([0, length_bytes[0], length_bytes[1], length_bytes[2]]);
        let (type_bytes, buf) = buf
            .split_first_chunk::<FRAME_TYPE_SIZE>()
            .ok_or(FrameError::MissingType)?;
        let _type = FrameType::try_from(type_bytes[0]).map_err(FrameError::UnknownType)?;
        let (flags_bytes, buf) = buf
            .split_first_chunk::<FRAME_FLAGS_SIZE>()
            .ok_or(FrameError::MissingFlags)?;
        let flags = flags_bytes[0];
        let (sid_bytes, buf) = buf
            .split_first_chunk::<FRAME_SID_SIZE>()
            .ok_or(FrameError::MissingSid)?;
        let sid = u32::from_be_bytes(*sid_bytes);
        let length_usize = length
            .try_into()
            .expect("frame length (u32) should always fit into usize on modern architectures");
        let (data, _) = buf
            .split_at_checked(length_usize)
            .ok_or(FrameError::MissingPayload)?;
        let raw_rame = RawFrame {
            length,
            _type,
            flags,
            sid,
            data,
        };
        raw_rame.try_into()
    }
}

impl Frame {
    pub fn get_length(&self) -> u32 {
        self.length
    }

    pub fn get_type(&self) -> FrameType {
        self._type
    }

    pub fn get_flags(&self) -> u8 {
        self.flags
    }

    pub fn get_sid(&self) -> u32 {
        self.sid
    }

    pub fn get_kind(&self) -> &FrameKind {
        &self.kind
    }

    pub fn get_size(&self) -> u32 {
        self.get_length()
            + (FRAME_LENGTH_SIZE + FRAME_TYPE_SIZE + FRAME_FLAGS_SIZE + FRAME_SID_SIZE) as u32
    }
}
