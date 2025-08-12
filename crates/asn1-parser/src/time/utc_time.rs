use alloc::format;

use super::{Day, Hour, Minute, Month, Second, Year, read_number};
use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, Tag, Taggable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UtcTime {
    pub year: Year,
    pub month: Month,
    pub day: Day,
    pub hour: Hour,
    pub minute: Minute,
    pub second: Option<Second>,
}

impl UtcTime {
    pub const TAG: Tag = Tag(23);

    pub fn new(year: Year, month: Month, day: Day, hour: Hour, minute: Minute, second: Option<Second>) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    fn calc_data_len(&self) -> usize {
        2 /* year */ + 2 /* month */ + 2 /* day */ + 2 /* hour */ + 2 /* minute */ + self.second.as_ref().map(|_| 2).unwrap_or_default() + 1
        /* 'Z' */
    }
}

impl<'data> Asn1ValueDecoder<'data> for UtcTime {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let utc_time = UtcTime {
            year: Year::try_from(read_number(reader)?)?,
            month: Month::try_from(read_number(reader)?)?,
            day: Day::try_from(read_number(reader)?)?,
            hour: Hour::try_from(read_number(reader)?)?,
            minute: Minute::try_from(read_number(reader)?)?,
            second: if reader.peek_byte()? != b'Z' {
                Some(Second::try_from(read_number(reader)?)?)
            } else {
                None
            },
        };

        if reader.read_byte()? != b'Z' {
            return Err("utctime value should end with 'Z'".into());
        }

        Ok(utc_time)
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl Taggable for UtcTime {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl Asn1Encoder for UtcTime {
    fn needed_buf_size(&self) -> usize {
        let value_len = self.calc_data_len();

        1 /* tag */ + len_size(value_len) + value_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.calc_data_len(), writer)?;

        writer.write_slice(format!("{:02}", self.year.as_ref()).as_bytes())?;
        writer.write_slice(format!("{:02}", self.month.as_ref()).as_bytes())?;
        writer.write_slice(format!("{:02}", self.day.as_ref()).as_bytes())?;
        writer.write_slice(format!("{:02}", self.hour.as_ref()).as_bytes())?;
        writer.write_slice(format!("{:02}", self.minute.as_ref()).as_bytes())?;

        if let Some(second) = self.second.as_ref() {
            writer.write_slice(format!("{:02}", second.as_ref()).as_bytes())?;
        }

        writer.write_byte(b'Z')
    }
}
