use std::ops::{Bound, RangeBounds};

static NUMBER_WORDS: &'static [&str] = &[
    "",
    "หนึ่ง",
    "สอง",
    "สาม",
    "สี่",
    "ห้า",
    "หก",
    "เจ็ด",
    "แปด",
    "เก้า",
];
static NUMBER_BASE_WORDS: &'static [&str] = &["", "สิบ", "ร้อย", "พัน", "หมื่น", "แสน", "ล้าน"];
static NUMBER_BASE_ALTERNATE_WORDS: &'static [&str] = &["ยี่", "เอ็ด", "ศูนย์"];
static MULTIPLICATIONS: &'static [i32] = &[
    1, //หลักหน่วย
    10, 100, 1000, 10000, 100000, 1000000,
];

const MILLION_LEN_SEGMENT: usize = 6;

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

fn text_number_to_thai_words_segment(text_number: &str, is_million: bool) -> String {
    let mut j = 0;
    let mut res_text_numbers: Vec<String> = vec![];
    let reversed_text_number: String = text_number.chars().rev().collect();
    for c in reversed_text_number.as_str().chars() {
        if c.is_digit(10) {
            let number_int = c.to_digit(10).unwrap();
            let base_idx = j % 6;
            let mut base = "";
            if number_int > 0 {
                base = NUMBER_BASE_WORDS[base_idx];
            }
            let mut number_word = NUMBER_WORDS[number_int as usize];
            if base_idx == 0 {
                if number_int == 0 && text_number.len() == 1 {
                    number_word = NUMBER_BASE_ALTERNATE_WORDS[2];
                } else if number_int == 1 && text_number.len() > 1 {
                    number_word = NUMBER_BASE_ALTERNATE_WORDS[1];
                }
            } else if base_idx == 1 {
                if number_int == 1 {
                    number_word = "";
                } else if number_int == 2 {
                    number_word = NUMBER_BASE_ALTERNATE_WORDS[0];
                }
            }
            let thai_word = format!("{}{}", number_word, base);
            if !thai_word.is_empty() {
                res_text_numbers.push(thai_word);
            }
            //res_text_numbers.push(base.to_string());
            //res_text_numbers.push(number_word.to_string());
            j = j + 1;
        }
    }
    if is_million {
        res_text_numbers.push(NUMBER_BASE_WORDS[6].to_string());
    }
    res_text_numbers.reverse();
    return res_text_numbers.join("");
}
pub fn text_number_to_thai_words(text_number: &str) -> String {
    let mut res_text_numbers: Vec<String> = vec![];
    let tmp_text_number = text_number.clone();

    let mut has_million = false;
    let mut new_start = 0;
    if tmp_text_number.len() >= MILLION_LEN_SEGMENT {
        new_start = tmp_text_number.len() - MILLION_LEN_SEGMENT;
    }

    let mut cur_len = tmp_text_number.len();
    let mut count_million = 0;

    loop {
        if cur_len > MILLION_LEN_SEGMENT {
            let tmp_text = tmp_text_number.substring(new_start, MILLION_LEN_SEGMENT); //tmp_text_number.get(new_start..new_end).unwrap();
            res_text_numbers.push(text_number_to_thai_words_segment(tmp_text, true));
            if new_start >= MILLION_LEN_SEGMENT {
                new_start -= MILLION_LEN_SEGMENT;
            }
            if cur_len >= MILLION_LEN_SEGMENT {
                cur_len -= MILLION_LEN_SEGMENT;
            }
            count_million += MILLION_LEN_SEGMENT;
            has_million = true;
        } else {
            let mut end = 0;
            if has_million {
                end = tmp_text_number.len() - count_million;
            } else {
                end = tmp_text_number.len();
            }
            let tmp_text = tmp_text_number.substring(0, end); //tmp_text_number.get(start..end).unwrap();
            res_text_numbers.push(text_number_to_thai_words_segment(tmp_text, false));
            break;
        }
    }
    res_text_numbers.reverse();
    return res_text_numbers.join("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_number_to_thai_words() {
        // text_number_to_thai_words("1234");
    }
}
