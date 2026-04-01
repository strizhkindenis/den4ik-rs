use std::array;

use crate::errors::{FrameError, SettingParseError};

const FRAME_LENGTH_SIZE: usize = 3;
const FRAME_TYPE_SIZE: usize = 1;
const FRAME_FLAGS_SIZE: usize = 1;
const FRAME_SID_SIZE: usize = 4;
pub const FRAME_HEADER_SIZE: usize =
    FRAME_LENGTH_SIZE + FRAME_TYPE_SIZE + FRAME_FLAGS_SIZE + FRAME_SID_SIZE;

const FRAME_RBIT_MASK: u32 = 0x7FFF_FFFF;

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
    Headers(FrameHeaders),
    Settings(FrameSettings),
    WindowUpdate(u32),
}

#[derive(Debug)]
pub struct FrameHeaders {
    is_end_stream: bool,
    is_end_headers: bool,
    is_padded: bool,
    is_priority: bool,
    is_exclusive: Option<bool>,
    sid_dep: Option<u32>,
    weight: Option<u8>,
    pad: Option<u8>,
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
pub struct FrameHeader {
    pub length: u32,
    pub r#type: FrameType,
    pub flags: u8,
    pub sid: u32,
}

impl TryFrom<&[u8; FRAME_HEADER_SIZE]> for FrameHeader {
    type Error = FrameError;

    fn try_from(buf: &[u8; FRAME_HEADER_SIZE]) -> Result<Self, Self::Error> {
        let (length_bytes, buf) = buf.split_at(FRAME_LENGTH_SIZE);
        let length = u32::from_be_bytes([0, length_bytes[0], length_bytes[1], length_bytes[2]]);
        let (type_bytes, buf) = buf.split_at(FRAME_TYPE_SIZE);
        let r#type = FrameType::try_from(type_bytes[0]).map_err(FrameError::UnknownType)?;
        let (flags_bytes, sid_bytes) = buf.split_at(FRAME_FLAGS_SIZE);
        let flags = flags_bytes[0];
        let sid = u32::from_be_bytes(array::from_fn(|i| sid_bytes[i])) & FRAME_RBIT_MASK;
        Ok(FrameHeader {
            length,
            r#type,
            flags,
            sid,
        })
    }
}

#[derive(Debug)]
pub struct RawFrame<'a> {
    header: FrameHeader,
    data: &'a [u8],
}

impl<'a> RawFrame<'a> {
    pub fn new(header: FrameHeader, data: &'a [u8]) -> Self {
        Self { header, data }
    }

    pub fn r#type(&self) -> FrameType {
        self.header.r#type
    }

    fn into_headers(self) -> Result<Frame, FrameError> {
        let Self { header, data } = self;
        assert!(matches!(header.r#type, FrameType::Headers));
        let is_end_stream = (header.flags & 0x01) != 0; //  0x01 | 0000 0001
        let is_end_headers = (header.flags & 0x04) != 0; // 0x04 | 0000 0100
        let is_padded = (header.flags & 0x08) != 0; //      0x08 | 0000 1000
        let is_priority = (header.flags & 0x20) != 0; //    0x20 | 0010 0000
        let (pad, data) = if is_padded {
            let (pad_byte, data) = data.split_at_checked(std::mem::size_of::<u8>()).unwrap();
            (Some(u8::from_be_bytes([pad_byte[0]])), data)
        } else {
            (None, data)
        };
        let (is_exclusive, sid_dep, data) = if is_priority {
            let (sid_dep_bytes, data) = data.split_at_checked(std::mem::size_of::<u32>()).unwrap();
            let sid_dep = u32::from_be_bytes(sid_dep_bytes.try_into().unwrap());
            let is_exclusive = Some((sid_dep & 0x8000_000) != 0);
            let sid_dep = Some(sid_dep & FRAME_RBIT_MASK);
            (is_exclusive, sid_dep, data)
        } else {
            (None, None, data)
        };
        todo!()
        // Ok(Frame::new(header, kind))
    }

    fn into_settings(self) -> Result<Frame, FrameError> {
        let Self { header, data } = self;
        assert!(matches!(header.r#type, FrameType::Settings));
        if !(header.length as usize).is_multiple_of(Setting::SIZE) {
            return Err(FrameError::InvalidSettingsLength(header.length));
        }
        let (settings_chunks, _) = data.as_chunks::<{ Setting::SIZE }>();
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
        Ok(Frame::new(header, kind))
    }

    fn into_window_update(self) -> Result<Frame, FrameError> {
        // TODO: make proper error checks
        let Self { header, data } = self;
        assert!(matches!(header.r#type, FrameType::WindowUpdate));
        if (header.length as usize) != std::mem::size_of::<u32>() {
            return Err(FrameError::InvalidSettingsLength(header.length));
        }
        let window_size = u32::from_be_bytes(array::from_fn(|i| data[i])) & FRAME_RBIT_MASK;
        let kind = FrameKind::WindowUpdate(window_size);
        Ok(Frame::new(header, kind))
    }

    fn into_other(self) -> Result<Frame, FrameError> {
        let Self { header, data } = self;
        let kind = FrameKind::Other(FrameOther {
            data: data.to_vec(),
        });
        Ok(Frame::new(header, kind))
    }
}

impl<'a> TryFrom<RawFrame<'a>> for Frame {
    type Error = FrameError;

    fn try_from(raw_frame: RawFrame<'a>) -> Result<Self, Self::Error> {
        match raw_frame.r#type() {
            FrameType::Settings => raw_frame.into_settings(),
            FrameType::WindowUpdate => raw_frame.into_window_update(),
            _ => raw_frame.into_other(),
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    header: FrameHeader,
    kind: FrameKind,
}

impl Frame {
    pub fn new(header: FrameHeader, kind: FrameKind) -> Self {
        match &kind {
            FrameKind::Settings(_) => assert!(matches!(header.r#type, FrameType::Settings)),
            FrameKind::WindowUpdate(_) => assert!(matches!(header.r#type, FrameType::WindowUpdate)),
            _ => (),
        }
        Self { header, kind }
    }

    pub fn length(&self) -> u32 {
        self.header.length
    }

    pub fn r#type(&self) -> FrameType {
        self.header.r#type
    }

    pub fn flags(&self) -> u8 {
        self.header.flags
    }

    pub fn sid(&self) -> u32 {
        self.header.sid
    }

    pub fn kind(&self) -> &FrameKind {
        &self.kind
    }

    pub fn size(&self) -> u32 {
        self.length() + FRAME_HEADER_SIZE as u32
    }
}
