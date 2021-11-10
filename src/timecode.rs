//! enables programmatic usage of SRT / WebVTT timecodes

use regex::Regex;

/// trait to implement for types that can be converted to
/// a `TimeCode`
pub trait TimeCodeTrait {
    /// Attempts to convert the type to `TimeCode`
    ///
    /// ```
    /// use title_parser::timecode::{TimeCodeTrait};
    ///
    /// let tc_string = "00:01:14.815";
    /// let tc = tc_string.to_timecode().unwrap();
    /// assert_eq!(tc.string, tc_string);
    ///
    /// // bad timecode
    /// let tc_string = "00:01:67.815";
    /// let tc = tc_string.to_timecode();
    /// assert_eq!(tc, Err("invalid timecode".to_string()));
    /// ```
    fn to_timecode(&self) -> Result<TimeCode, String>;
}

impl TimeCodeTrait for str {
    fn to_timecode(&self) -> Result<TimeCode, String> {
        let re = Regex::new(r"^((\d{2,4}):)?([0-5][0-9]):([0-5][0-9])[\.,](\d{3})$")
            .expect("failed to compile regex");
        let caps = re
            .captures(self)
            .ok_or_else(|| "invalid timecode".to_string())?;

        // it should be safe to unwrap() these values
        let hh: u32;
        if let Some(num) = caps.get(2) {
            hh = num.as_str().parse().unwrap();
        } else {
            hh = 0;
        }

        let tc = TimeCode {
            string: self.to_string(),
            hh,
            mm: caps.get(3).unwrap().as_str().parse().unwrap(),
            ss: caps.get(4).unwrap().as_str().parse().unwrap(),
            ttt: caps.get(5).unwrap().as_str().parse().unwrap(),
        };
        Ok(tc)
    }
}

/// A TimeCode represents any valid SRT or VTT timestamps used for cue timing
/// such as `00:01:14.815` or ``01:14.815``
///
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct TimeCode {
    /// copy of string used to create TimeCode
    pub string: String,
    /// hours field from timecode
    pub hh: u32,
    /// minutes field from timecode
    pub mm: u32,
    /// seconds field from timecode
    pub ss: u32,
    /// milliseconds field from timecode
    pub ttt: u32,
}

impl TimeCode {
    /// Converts TimeCode to seconds
    ///
    /// Converts hh and mm to seconds, sums them with ss field.
    /// milliseconds are skipped, rounding the number seconds down to the nearest
    /// second.
    ///
    /// ```
    /// use title_parser::timecode::{TimeCodeTrait};
    /// let tc = "00:01:14.815".to_timecode().unwrap();
    /// assert_eq!(tc.to_seconds(), 74);
    /// ```
    pub fn to_seconds(&self) -> u32 {
        (self.hh * 60 * 60) + (self.mm * 60) + (self.ss)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timecode_from_str() -> Result<(), String> {
        let tc_string = "01:02:03.004";
        let expected = TimeCode {
            string: tc_string.to_string(),
            hh: 1,
            mm: 2,
            ss: 3,
            ttt: 4,
        };
        assert_eq!(tc_string.to_timecode()?, expected);
        Ok(())
    }

    #[test]
    fn timecode_from_str_no_hours() -> Result<(), String> {
        let tc_string = "02:03.004";
        let expected = TimeCode {
            string: tc_string.to_string(),
            hh: 0,
            mm: 2,
            ss: 3,
            ttt: 4,
        };
        assert_eq!(tc_string.to_timecode()?, expected);
        Ok(())
    }

    #[test]
    fn timecode_from_invalid_str() -> Result<(), String> {
        let tc_string = "01:02:03";
        assert_eq!(tc_string.to_timecode(), Err("invalid timecode".to_string()));
        let tc_string = "0a:02:03.001";
        assert_eq!(tc_string.to_timecode(), Err("invalid timecode".to_string()));
        Ok(())
    }

    #[test]
    fn timecode_with_invalid_values() -> Result<(), String> {
        let tc_string = "01:02:60.004";
        assert_eq!(tc_string.to_timecode(), Err("invalid timecode".to_string()));
        let tc_string = "01:60:03.004";
        assert_eq!(tc_string.to_timecode(), Err("invalid timecode".to_string()));
        Ok(())
    }

    #[test]
    fn timecode_to_seconds() -> Result<(), String> {
        let tc_string = "01:02:03.004";
        assert_eq!(tc_string.to_timecode()?.to_seconds(), 3723);
        Ok(())
    }
}
