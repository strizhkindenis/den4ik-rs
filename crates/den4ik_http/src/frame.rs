use std::fmt;

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
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct FrameSettings {
    settings: Vec<Setting>,
}

#[derive(Debug, Clone, Copy)]
pub enum Setting {
    HeaderTableSize(u32),
    EnablePush(bool),
    MaxConcurrentStreams(u32),
    InitialWindowSize(u32),
    MaxHeaderListSize(u32),
}

#[derive(Debug)]
pub enum SettingParseError {
    UnknownId(u16),
    InvalidValue(u16, u32),
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
pub struct Frame {
    length: u32,
    _type: FrameType,
    flags: u8,
    sid: u32,
    kind: FrameKind,
}

impl TryFrom<&[u8]> for Frame {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (length, buf) = value.split_at_checked(FRAME_LENGTH_SIZE).unwrap();
        let length = u32::from_be_bytes([0, length[0], length[1], length[2]]);
        let (_type, buf) = buf.split_at_checked(FRAME_TYPE_SIZE).unwrap();
        let _type = FrameType::try_from(_type[0]).unwrap();
        let (flags, buf) = buf.split_at_checked(FRAME_FLAGS_SIZE).unwrap();
        let flags = u8::from_be_bytes([flags[0]]);
        let (sid, buf) = buf.split_at_checked(FRAME_SID_SIZE).unwrap();
        let sid = u32::from_be_bytes([sid[0], sid[1], sid[2], sid[3]]);
        let (data, _) = buf.split_at_checked(length.try_into().unwrap()).unwrap();
        let kind = match _type {
            FrameType::Settings => {
                assert_eq!(length % 6, 0);
                let (settings, _) = data.as_chunks::<6>();
                let settings = settings
                    .iter()
                    .map(|chunk| {
                        let id = u16::from_be_bytes([chunk[0], chunk[1]]);
                        let val = u32::from_be_bytes([chunk[2], chunk[3], chunk[4], chunk[5]]);
                        Setting::try_from((id, val))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                FrameKind::Settings(FrameSettings { settings })
            }
            _ => FrameKind::Other(FrameOther {
                data: data.to_vec(),
            }),
        };
        Ok(Frame {
            length,
            _type,
            flags,
            sid,
            kind,
        })
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
