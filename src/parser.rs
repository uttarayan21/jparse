use crate::types::*;

use nom::{
    bytes::streaming::{tag, take, take_until1},
    combinator::{cond, peek},
    error::{Error, ErrorKind},
    number::streaming::be_u16,
    IResult,
};

impl Marker {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, marker) = be_u16(input)?;
        Ok((
            input,
            Marker::from_bytes(marker)
                .map_err(|_| nom::Err::Failure(Error::new(input, ErrorKind::Tag)))?,
        ))
    }
}

impl Segment {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, marker) = Marker::parse(input)?;

        if marker.has_length() {
            let (input, length) = be_u16(input)?;
            let (input, data) = take(length)(input)?;

            Ok((
                input,
                Self {
                    marker,
                    data: data.to_vec(),
                },
            ))
        } else {
            Ok((
                input,
                Self {
                    marker,
                    data: vec![],
                },
            ))
        }
    }

    fn skip(input: &[u8]) -> IResult<&[u8], usize> {
        let _input = input;
        let (input, marker) = Marker::parse(input)?;
        let (input, length) = cond(marker.has_length(), be_u16)(input)?;
        let (input, _) = cond(length.is_some(), take(length.unwrap_or(2) - 2))(input)?;
        Ok((input, _input.len() - input.len()))
    }
}

impl Jpeg {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, _) = tag(Marker::SOI.to_bytes())(input)?;
        let mut segments = Vec::new();
        let mut input = input;
        loop {
            let (i, segment) = Segment::parse(input)?;
            input = i;
            let marker = segment.marker;
            segments.push(segment);
            if marker == Marker::EOI {
                break;
            }
        }
        Ok((input, Self { segments }))
    }

    pub fn skip(mut input: &[u8]) -> IResult<&[u8], usize> {
        let mut total = 0;
        let (_, mut marker) = peek(Marker::parse)(input)?;
        while marker != Marker::EOI {
            let (_, _marker) = peek(Marker::parse)(input)?;
            if _marker == Marker::SOS {
                let (input, data) = take_until1(Marker::EOI.to_bytes().as_slice())(input)?;
                let (input, _) = tag(Marker::EOI.to_bytes().as_slice())(input)?;
                return Ok((input, data.len() + total + 2));
            }
            let (_input, size) = Segment::skip(input)?;
            total += size;
            marker = _marker;
            input = _input;
        }
        Ok((input, total))
    }

    #[cfg(feature = "bufreader")]
    pub(crate) fn __skip_owned_error(input: &[u8]) -> IResult<&[u8], usize, ()> {
        Self::skip(input).map_err(|e| e.map(|_| ()))
    }
}
