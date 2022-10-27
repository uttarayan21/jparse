use crate::errors;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u16)]
pub enum Marker {
    APP0 = 0xffe0,
    APP1 = 0xffe1,
    APP2 = 0xffe2,
    APP3 = 0xffe3,
    APP4 = 0xffe4,
    APP5 = 0xffe5,
    APP6 = 0xffe6,
    APP7 = 0xffe7,
    APP8 = 0xffe8,
    APP9 = 0xffe9,
    APP10 = 0xffea,
    APP11 = 0xffeb,
    APP12 = 0xffec,
    APP13 = 0xffed,
    APP14 = 0xffee,
    APP15 = 0xffef,

    COM = 0xfffe,
    DHP = 0xffde,
    DHT = 0xffc4,
    DNL = 0xffdc,
    DQT = 0xffdb,
    DRI = 0xffdd,
    EOI = 0xffd9,
    RST0 = 0xffd0,
    RST1 = 0xffd1,
    RST2 = 0xffd2,
    RST3 = 0xffd3,
    RST4 = 0xffd4,
    RST5 = 0xffd5,
    RST6 = 0xffd6,
    RST7 = 0xffd7,
    SOF0 = 0xffc0,
    SOF1 = 0xffc1,
    SOF10 = 0xffca,
    SOF11 = 0xffcb,
    SOF13 = 0xffcd,
    SOF14 = 0xffce,
    SOF15 = 0xffcf,
    SOF2 = 0xffc2,
    SOF3 = 0xffc3,
    SOF5 = 0xffc5,
    SOF6 = 0xffc6,
    SOF7 = 0xffc7,
    SOF9 = 0xffc9,
    SOI = 0xffd8,
    SOS = 0xffda,
}

impl Marker {
    pub fn to_bytes(&self) -> [u8; 2] {
        let marker = *self as u16;
        [(marker >> 8) as u8, marker as u8]
    }

    pub fn from_bytes(input: u16) -> Result<Self, errors::Error> {
        match input {
            0xffe0 => Ok(Marker::APP0),
            0xffe1 => Ok(Marker::APP1),
            0xffe2 => Ok(Marker::APP2),
            0xffe3 => Ok(Marker::APP3),
            0xffe4 => Ok(Marker::APP4),
            0xffe5 => Ok(Marker::APP5),
            0xffe6 => Ok(Marker::APP6),
            0xffe7 => Ok(Marker::APP7),
            0xffe8 => Ok(Marker::APP8),
            0xffe9 => Ok(Marker::APP9),
            0xffea => Ok(Marker::APP10),
            0xffeb => Ok(Marker::APP11),
            0xffec => Ok(Marker::APP12),
            0xffed => Ok(Marker::APP13),
            0xffee => Ok(Marker::APP14),
            0xffef => Ok(Marker::APP15),
            0xfffe => Ok(Marker::COM),
            0xffde => Ok(Marker::DHP),
            0xffc4 => Ok(Marker::DHT),
            0xffdc => Ok(Marker::DNL),
            0xffdb => Ok(Marker::DQT),
            0xffdd => Ok(Marker::DRI),
            0xffd9 => Ok(Marker::EOI),
            0xffd0 => Ok(Marker::RST0),
            0xffd1 => Ok(Marker::RST1),
            0xffd2 => Ok(Marker::RST2),
            0xffd3 => Ok(Marker::RST3),
            0xffd4 => Ok(Marker::RST4),
            0xffd5 => Ok(Marker::RST5),
            0xffd6 => Ok(Marker::RST6),
            0xffd7 => Ok(Marker::RST7),
            0xffc0 => Ok(Marker::SOF0),
            0xffc1 => Ok(Marker::SOF1),
            0xffca => Ok(Marker::SOF10),
            0xffcb => Ok(Marker::SOF11),
            0xffcd => Ok(Marker::SOF13),
            0xffce => Ok(Marker::SOF14),
            0xffcf => Ok(Marker::SOF15),
            0xffc2 => Ok(Marker::SOF2),
            0xffc3 => Ok(Marker::SOF3),
            0xffc5 => Ok(Marker::SOF5),
            0xffc6 => Ok(Marker::SOF6),
            0xffc7 => Ok(Marker::SOF7),
            0xffc9 => Ok(Marker::SOF9),
            0xffd8 => Ok(Marker::SOI),
            0xffda => Ok(Marker::SOS),
            m => Err(errors::Error::InvalidMarker(m)),
        }
    }

    pub fn has_length(&self) -> bool {
        match self {
            Marker::SOI
            | Marker::SOS
            | Marker::EOI
            | Marker::RST0
            | Marker::RST1
            | Marker::RST2
            | Marker::RST3
            | Marker::RST4
            | Marker::RST5
            | Marker::RST6
            | Marker::RST7 => false,
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Segment {
    pub marker: Marker,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Jpeg {
    pub segments: Vec<Segment>,
}
