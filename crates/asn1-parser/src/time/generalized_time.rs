use alloc::format;
use alloc::string::String;
use core::str::from_utf8;

#[cfg(not(feature = "std"))]
use num_traits::float::FloatCore;

use super::{read_number, Day, Hour, Minute, Month};
use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, Error, Tag, Taggable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Year(u16);

impl Year {
    pub fn new(year: u16) -> Self {
        Self(year)
    }

    fn from_reader(reader: &mut Reader) -> Asn1Result<Self> {
        Ok(Self(from_utf8(reader.read(4)?)?.parse::<u16>()?))
    }

    fn to_writer(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_slice(format!("{:04}", self.0).as_bytes())?;

        Ok(())
    }
}

impl AsRef<u16> for Year {
    fn as_ref(&self) -> &u16 {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Second(f32);

impl PartialEq for Second {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() <= f32::EPSILON
    }
}

impl Eq for Second {}

impl From<Second> for f32 {
    fn from(value: Second) -> Self {
        value.0
    }
}

impl AsRef<f32> for Second {
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}

impl TryFrom<u8> for Second {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 60 {
            Ok(Self(value.into()))
        } else {
            Err("invalid value".into())
        }
    }
}

impl TryFrom<f32> for Second {
    type Error = crate::Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value < 60.0 {
            Ok(Self(value))
        } else {
            Err("invalid value".into())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalTimeDirection {
    Plus,
    Minus,
}

impl From<LocalTimeDirection> for char {
    fn from(value: LocalTimeDirection) -> Self {
        match value {
            LocalTimeDirection::Minus => '-',
            LocalTimeDirection::Plus => '+',
        }
    }
}

impl From<LocalTimeDirection> for u8 {
    fn from(value: LocalTimeDirection) -> Self {
        match value {
            LocalTimeDirection::Minus => b'-',
            LocalTimeDirection::Plus => b'+',
        }
    }
}

impl TryFrom<u8> for LocalTimeDirection {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'-' => Ok(Self::Minus),
            b'+' => Ok(Self::Plus),
            _ => Err("invalid GeneralTime data".into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalTimeDiffFactor {
    pub time_direction: LocalTimeDirection,
    pub hour: Hour,
    pub minute: Minute,
}

impl LocalTimeDiffFactor {
    const ENCODED_LEN: usize = 1 /* sign */ + 2 /* hour */ + 2 /* minute */;

    fn from_reader(reader: &mut Reader) -> Asn1Result<Self> {
        Ok(Self {
            time_direction: reader.read_byte()?.try_into()?,
            hour: Hour::try_from(read_number(reader)?)?,
            minute: Minute::try_from(read_number(reader)?)?,
        })
    }

    fn to_writer(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(self.time_direction.into())?;
        writer.write_slice(format!("{:02}", self.hour.as_ref()).as_bytes())?;
        writer.write_slice(format!("{:02}", self.minute.as_ref()).as_bytes())?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneralizedTime {
    pub year: Year,
    pub month: Month,
    pub day: Day,
    pub hour: Hour,
    pub minute: Minute,
    pub second: Second,
    pub local_time: Option<LocalTimeDiffFactor>,
}

impl GeneralizedTime {
    pub const TAG: Tag = Tag(24);

    pub fn new(
        year: Year,
        month: Month,
        day: Day,
        hour: Hour,
        minute: Minute,
        second: Second,
        local_time: Option<LocalTimeDiffFactor>,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            local_time,
        }
    }

    fn calc_data_len(&self) -> usize {
        let second_len = if self.second.as_ref().fract() > f32::EPSILON {
            2 /* int part */ + 1 /* dot */ + 3 /* fract part */
        } else {
            2
        };

        2 /* year */ + 2 /* month */ + 2 /* day */ + 2 /* hour */ + 2 /* minute */ + second_len + LocalTimeDiffFactor::ENCODED_LEN
    }
}

impl Taggable for GeneralizedTime {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl<'data> Asn1ValueDecoder<'data> for GeneralizedTime {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let year = Year::from_reader(reader)?;
        let month = Month::try_from(read_number(reader)?)?;
        let day = Day::try_from(read_number(reader)?)?;
        let hour = Hour::try_from(read_number(reader)?)?;
        let minute = Minute::try_from(read_number(reader)?)?;

        let second_int_part = read_number(reader)?;

        if reader.empty() {
            return Ok(Self {
                year,
                month,
                day,
                hour,
                minute,
                second: second_int_part.try_into().unwrap(),
                local_time: None,
            });
        }

        match char::from(reader.peek_byte()?) {
            'Z' => Ok(Self {
                year,
                month,
                day,
                hour,
                minute,
                second: second_int_part.try_into().unwrap(),
                local_time: None,
            }),
            '+' | '-' => Ok(Self {
                year,
                month,
                day,
                hour,
                minute,
                second: second_int_part.try_into().unwrap(),
                local_time: Some(LocalTimeDiffFactor::from_reader(reader)?),
            }),
            '.' => {
                // sorry for this code
                let _ = reader.read_byte()?; // read the dot byte
                let mut frac = String::new();

                let mut c = char::from(reader.peek_byte()?);
                while c.is_ascii_digit() {
                    reader.read_byte()?;
                    frac.push(c);

                    if reader.empty() {
                        let seconds = format!("{}.{}", second_int_part, frac).parse::<f32>()?;
                        return Ok(Self {
                            year,
                            month,
                            day,
                            hour,
                            minute,
                            second: seconds.try_into().unwrap(),
                            local_time: None,
                        });
                    }

                    c = char::from(reader.peek_byte()?);
                }
                let seconds = format!("{}.{}", second_int_part, frac).parse::<f32>()?;

                match c {
                    'Z' => Ok(Self {
                        year,
                        month,
                        day,
                        hour,
                        minute,
                        second: seconds.try_into().unwrap(),
                        local_time: None,
                    }),
                    '+' | '-' => Ok(Self {
                        year,
                        month,
                        day,
                        hour,
                        minute,
                        second: seconds.try_into().unwrap(),
                        local_time: Some(LocalTimeDiffFactor::from_reader(reader)?),
                    }),
                    _ => Err("invalid GeneralTime data: invalid char after second frac part".into()),
                }
            }
            _ => Err("invalid GeneralTime data: invalid char after second int part".into()),
        }
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl Asn1Encoder for GeneralizedTime {
    fn needed_buf_size(&self) -> usize {
        let value_len = self.calc_data_len();

        1 /* tag */ + len_size(value_len) + value_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.calc_data_len(), writer)?;

        self.year.to_writer(writer)?;
        writer.write_slice(format!("{:02}", self.month.as_ref()).as_bytes())?;
        writer.write_slice(format!("{:02}", self.day.as_ref()).as_bytes())?;
        writer.write_slice(format!("{:02}", self.hour.as_ref()).as_bytes())?;
        writer.write_slice(format!("{:02}", self.minute.as_ref()).as_bytes())?;

        if self.second.as_ref().fract() > f32::EPSILON {
            writer.write_slice(format!("{:06.3}", self.second.as_ref()).as_bytes())?;
        } else {
            writer.write_slice(format!("{:02}", *self.second.as_ref() as u8).as_bytes())?;
        }

        if let Some(local_time) = self.local_time.as_ref() {
            local_time.to_writer(writer)
        } else {
            writer.write_byte(b'Z')
        }
    }
}
