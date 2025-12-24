use std::io::{BufReader, Read};

use super::{
    event::{CrocEvent, CrocHashOutput, CrocTransferOutput},
    read_until_any,
};

pub struct CrocParser<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> CrocParser<R> {
    pub fn new(inner: R) -> Self {
        Self {
            reader: BufReader::new(inner),
        }
    }

    pub fn parse_next_event(&mut self) -> Option<CrocEvent> {
        let mut buf = Vec::<u8>::new();
        let bytes_read = read_until_any(&mut self.reader, b"\r\n", &mut buf).ok()?;
        let line_cow = String::from_utf8_lossy(&buf);
        let line = line_cow.trim();

        match bytes_read {
            0 => Some(CrocEvent::Done),
            _ => {
                if let Some(code) = try_parse_code(line) {
                    return Some(CrocEvent::CodeGenerated(code));
                }

                if let Some(data) = try_parse_hashing(line) {
                    return Some(CrocEvent::HashOutput(data));
                }

                if let Some(data) = try_parse_tranferring(line) {
                    return Some(CrocEvent::TransferOutput(data));
                }

                Some(CrocEvent::Unknown(line.to_string()))
            }
        }
    }
}

fn try_parse_code(line: &str) -> Option<String> {
    let code = line.strip_prefix("Code is: ")?;
    Some(code.to_string())
}

fn try_parse_hashing(raw: &str) -> Option<CrocHashOutput> {
    if !raw.starts_with("Hashing") {
        return None;
    }

    let raw = raw.strip_prefix("Hashing")?;
    let percent_pos = raw.rfind('%')?;
    let space_before_percent = raw[..percent_pos].rfind(' ')?;
    let after_percent = &raw[percent_pos + 1..];

    let progress = get_progress(raw, space_before_percent, percent_pos)?;
    let filename = get_filename(raw, space_before_percent);
    let meta = get_metadata(after_percent)?;

    let speed = parse_speed(&meta)?;
    let (time_spent, time_remaining) = parse_time(after_percent)?;

    Some(CrocHashOutput {
        raw_message: raw.to_string(),
        progress,
        speed,
        time_spent,
        time_remaining,
        filename,
    })
}

fn try_parse_tranferring(raw: &str) -> Option<CrocTransferOutput> {
    if raw.is_empty() {
        return None;
    }

    let percent_pos = raw.rfind('%')?;
    let space_before_percent = raw[..percent_pos].rfind(' ')?;
    let after_percent = &raw[percent_pos + 1..];

    let progress = get_progress(raw, space_before_percent, percent_pos)?;
    let filename = get_filename(raw, space_before_percent);
    let meta = get_metadata(after_percent)?;

    let mut meta_split = meta.split(',');
    let (total_sent, total_size) = parse_size(meta_split.next()?.trim())?;
    let speed = parse_speed(meta_split.next()?.trim())?;
    let time = parse_time(after_percent);

    Some(CrocTransferOutput {
        raw_message: raw.to_string(),
        progress,
        total_size,
        total_sent,
        speed,
        time_spent: time.map(|(s, _)| s),
        time_remaining: time.map(|(_, r)| r),
        filename,
    })
}

fn get_metadata(line: &str) -> Option<String> {
    // Find the '(' and ')' that enclose "(sent/total UNIT, speed UNIT/s)".
    let start_paren = line.find('(')?;
    let end_paren = line.find(')')?;
    let inner = line[start_paren + 1..end_paren].trim().to_string(); // e.g. "45/52 MB, 12 MB/s"
    Some(inner)
}

fn get_filename(line: &str, end_index: usize) -> String {
    line[..end_index].trim().to_string()
}

fn get_progress(line: &str, start_index: usize, percent_index: usize) -> Option<u8> {
    let progress_str = line[start_index + 1..percent_index].trim();
    progress_str.parse().ok()
}

fn get_size_number(value: &str, unit: &str) -> Option<usize> {
    let unit = unit.to_ascii_uppercase();

    let num = value.parse().ok()?;
    unit_to_bytes(num, &unit)
}

fn unit_to_bytes(value: f64, unit: &str) -> Option<usize> {
    let multiplier = match unit {
        "B" => 1_f64,
        "KB" => 1_000_f64,
        "MB" => 1_000_000_f64,
        "GB" => 1_000_000_000_f64,
        "TB" => 1_000_000_000_000_f64,
        _ => return None,
    };
    Some((value * multiplier) as usize)
}

fn parse_size(s: &str) -> Option<(usize, usize)> {
    // Format: "{sent} {unit}/{total} {unit}"
    let mut size_iter = s.split('/');
    let sent = size_iter.next()?.trim();
    let total = size_iter.next()?.trim();

    // Separate numeric value from unit on the total side.
    let mut total_iter = total.split_whitespace();
    let total = total_iter.next()?;
    let total_unit = total_iter.next()?;

    let mut sent_iter = sent.split_whitespace();
    let sent = sent_iter.next()?;
    // its possible that the sent unit is omitted,
    // if it is, its the same as the total unit.
    let sent_unit = match sent_iter.next() {
        None => total_unit,
        Some(unit) => unit,
    };

    // Both numbers share the same unit.
    let sent = get_size_number(sent, sent_unit)?;
    let total = get_size_number(total, total_unit)?;

    Some((sent, total))
}

fn parse_speed(s: &str) -> Option<usize> {
    // Format: "value UNIT/s"
    let mut speed_iter = s.split_whitespace();
    let speed_val = speed_iter.next()?; // "12"
    let speed_unit = speed_iter.next()?.trim_end_matches("/s"); // "MB"
    get_size_number(speed_val, speed_unit)
}

fn parse_time(s: &str) -> Option<(usize, usize)> {
    // The timing block follows the ')' – look for '[' and ']'.
    // Format: "[{spent} s:{remaining} s]"
    let start_bracket = s.rfind('[')?;
    let end_bracket = s.find(']')?;
    let timing = &s[start_bracket + 1..end_bracket];

    let mut time_parts = timing.split(':');
    let spent = time_parts.next()?.trim_end_matches('s');
    let remaining = time_parts.next()?.trim_end_matches('s');

    let spent: usize = spent.parse().ok()?;
    let remaining: usize = remaining.parse().ok()?;

    Some((spent, remaining))
}
