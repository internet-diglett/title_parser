#![warn(missing_docs)]

//! Parser for SRT and WebVTT
//!
//! Provides a parser that will extract a sequence of Cues
//! from text that conforms to SRT or WebVTT standards

pub mod timecode;
use regex::Regex;
use timecode::{TimeCode, TimeCodeTrait};
// use std::{error, fs};

/// A Cue represents a single SRT / WebVTT cue extracted from
/// a subtitle file:
///
/// ```vtt
/// 14
/// 00:01:14.815 --> 00:01:18.114
/// - This line belongs to a subtitle cue.
/// - This line is also a member of the same cue.
/// ```
///
pub struct Cue {
    /// timestamp for cue to appear
    pub start: TimeCode,
    /// timestamp for cue to disappear
    pub end: TimeCode,
    /// text for cue to display
    pub text: String,
}

/// trait to implement for types that can be converted to
/// a `Cue`
pub trait CueTrait {
    /// Attempts to create a cue from a string
    ///
    /// ```vtt
    /// 00:01:14.815 --> 00:01:18.114
    /// - I'm text for a cue
    /// - Me too!
    /// ```
    ///
    /// ```
    /// use title_parser::{CueTrait};
    ///
    /// let text = "00:01:14.815 --> 00:01:18.114\n- I'm text for a cue\n- Me too!";
    ///
    /// let cue = text.to_cue().unwrap();
    /// assert_eq!(cue.text, "I'm text for a cue\nMe too!");
    /// ```
    fn to_cue(&self) -> Result<Cue, String>;
}

impl CueTrait for str {
    fn to_cue(&self) -> Result<Cue, String> {
        let lines: Vec<&str> = self.trim().split('\n').into_iter().collect();
        let (start, end) =
            generate_timecodes(lines[0]).ok_or_else(|| "not a valid cue".to_string())?;
        let cue_lines: Vec<&str> = lines[1..].to_vec();
        let clean_lines: Vec<String> = cue_lines.iter().map(|i| sanitize_text(i)).collect();
        Ok(Cue {
            start,
            end,
            text: clean_lines.join("\n"),
        })
    }
}

// Attempts to extract TimeCodes from input, ignores css formatting text
fn generate_timecodes(input: &str) -> Option<(TimeCode, TimeCode)> {
    let re = Regex::new(r"^([0-9:\.,]{9,}) --> ([0-9:\.,]{9,})( .*)?$")
        .expect("failed to compile regex");
    let caps = re.captures(input)?;
    let start = caps.get(1)?.as_str().to_timecode().ok()?;
    let end = caps.get(2)?.as_str().to_timecode().ok()?;
    Some((start, end))
}

static REGEX_TO_PRUNE: [&str; 3] = [r"<[0-9a-zA-Z\.,:_\-]+>", r"</[0-9a-zA-Z\.,:_\-]+>", r"^\- "];

static ES_TO_PRUNE: [&str; 6] = ["&amp;", "&lt;", "&gt;", "&lrm;", "&rlm;", "&nbsp;"];

// Removes leading hyphens, HTML tags, CSS tags, etc. from input
fn sanitize_text(input: &str) -> String {
    let mut text: String = input.to_string();
    for regex in REGEX_TO_PRUNE.iter() {
        let re = Regex::new(regex).expect("unable to compile regex");
        text = re.replace_all(&text, "").to_string();
    }
    for es in ES_TO_PRUNE.iter() {
        text = text.replace(es, "");
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_generate_timecodes() -> Result<(), String> {
        let input = "00:00:13.916 --> 00:00:16.500 position:50.00%,middle align:middle";
        let expected = ("00:00:13.916".to_timecode()?, "00:00:16.500".to_timecode()?);
        assert_eq!(generate_timecodes(input), Some(expected));
        Ok(())
    }

    #[test]
    fn private_sanitize_text() -> Result<(), String> {
        let input = "<c.japanese><c.bg_some>&lrm;（聖弥）フフッ</c.bg_some></c.japanese>";
        assert_eq!(sanitize_text(input), "（聖弥）フフッ".to_string());
        Ok(())
    }
}
