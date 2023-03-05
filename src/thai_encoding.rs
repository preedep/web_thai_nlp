use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::{THChar,KO_KAI,KHOMUT,ZERO,NINE,THAI_FRONT_VOWELS,THAI_TONE_VOWELS};

pub struct THString {
    words: String
}
struct THCharVowelIdxPair {
    idx : u32,
    th_vowel_char : THChar
}

pub trait THCharCode {
    fn is_thai_char(&self) -> bool;
    fn is_thai_number(&self) -> bool;
}
pub trait THStringOp {
    fn chars_count(&self) -> usize;
    fn bytes_len(&self) -> usize;
}
/////
//  Impl
/////
impl THCharCode for THChar {
    fn is_thai_char(&self) -> bool {
        let ch = self;
        if *ch >= KO_KAI && *ch <= KHOMUT {
            return true;
        }
        return false;
    }
    fn is_thai_number(&self) -> bool {
        if *self >= ZERO && *self <= NINE {
            return true;
        }
        return false;
    }
}
impl From<&str> for THString {
    fn from(thai_str: &str) -> Self {
        THString {
            words: thai_str.to_string(),
        }
    }
}
impl fmt::Display for THString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.words)
    }
}

impl Debug for THString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.words)
    }
}

impl THStringOp for THString {
    fn chars_count(&self) -> usize {
        self.words.chars().count()
    }
    fn bytes_len(&self) -> usize {
        self.words.len()
    }
}

impl PartialEq for THString {
    fn eq(&self, other: &Self) -> bool {
        let thai_string = self.transform_text_for_sort();
        let other_thai_string = other.transform_text_for_sort();
        thai_string.words.eq(&other_thai_string.words)
    }
}

impl PartialOrd for THString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let thai_string = self.transform_text_for_sort();
        let other_thai_string = other.transform_text_for_sort();
        let ordering = thai_string.words.partial_cmp(&other_thai_string.words);
        match ordering {
            Some(Ordering::Equal) => {
                self.words.partial_cmp(&other.words)
            }
            _ => {
                return ordering;
            }
        }
    }
}

impl THString {
    fn find_front_vowel_indexes(&self) -> Vec<THCharVowelIdxPair> {
        self.find_vowel_indexes(THAI_FRONT_VOWELS)
    }
    fn find_tone_vowel_indexes(&self) -> Vec<THCharVowelIdxPair> {
        self.find_vowel_indexes(THAI_TONE_VOWELS)
    }
    fn find_vowel_indexes(&self, vowels_set: &'static [THChar]) -> Vec<THCharVowelIdxPair> {
        let mut indexes:Vec<THCharVowelIdxPair> = Vec::new();
        for (i,word) in self.words.chars().enumerate(){
            for (vowel_item) in vowels_set {
                if word == *vowel_item {
                    let pair = THCharVowelIdxPair{ idx: i as u32,
                        th_vowel_char: *vowel_item
                    };
                    indexes.push(pair);
                    break;
                }
            }
        }
        indexes
    }
    fn transform_text_for_sort(&self) -> THString {
        let mut chars: Vec<_> = self.words.as_str().chars().collect();
        let idx_tone_vowel_set = self.find_tone_vowel_indexes();
        //let mut vowel_chars: Vec<THChar> = Vec::new();
        for (i,idx_tone_vowel) in idx_tone_vowel_set.iter().enumerate() {
            let idx = idx_tone_vowel.idx;
            let counts = chars.len();
            if idx  < counts as u32 {
                //vowel_chars.push(idx_tone_vowel.th_vowel_char);
                chars.remove(idx_tone_vowel.idx as usize);
            }
        }
        let idx_front_vowel_set = self.find_front_vowel_indexes();
        for (i,front_vowel_pair) in idx_front_vowel_set.iter().enumerate() {
            let idx = front_vowel_pair.idx;
            let counts = chars.len();
            if idx + 1 < counts as u32 {
                chars.swap(idx as usize, (idx + 1) as usize);
            }
        }
        let new_words : String = chars.into_iter().collect();
        //let new_vowels : String = vowel_chars.into_iter().collect();
        //let new_words = format!("{}{}",new_words,new_vowels);
        THString {
            words : new_words
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_thai_words_sorting1() {
        //{"ไก่", "เกิด", "กาล", "เป็ด", "หมู", "วัว", "วันที่", "cat"}
        let mut thai_words : Vec<THString> = vec![
            THString::from("ไก่"),
            THString::from("เกิด"),
            THString::from("กาล"),
            THString::from("เป็ด"),
            THString::from("หมู"),
            THString::from("วัว"),
            THString::from("วันที่"),
            THString::from("cat"),
        ];
        thai_words.sort_by(|a,b|a.partial_cmp(b).unwrap());
        //{"cat", "กาล", "เกิด", "ไก่", "เป็ด", "วันที่", "วัว", "หมู"}
        assert_eq!(thai_words,
                   vec![
                       THString::from("cat"),
                       THString::from("กาล"),
                       THString::from("เกิด"),
                       THString::from("ไก่"),
                       THString::from("เป็ด"),
                       THString::from("วันที่"),
                       THString::from("วัว"),
                       THString::from("หมู"),
                   ]
        );
    }
    #[test]
    fn test_thai_words_sorting2() {
        let mut thai_words : Vec<THString> = vec![
            THString::from("แห่ง"),
            THString::from("แหง่")
        ];
        thai_words.sort_by(|a,b|a.partial_cmp(b).unwrap());
        assert_eq!(thai_words,
                   vec![
                       THString::from("แหง่"),
                       THString::from("แห่ง")
                   ]
        );
    }
    #[test]
    fn test_thai_words_sorting3() {
        let mut thai_words : Vec<THString> = vec![
            THString::from("เก"),
            THString::from("กา")
        ];
        thai_words.sort_by(|a,b|a.partial_cmp(b).unwrap());
        assert_eq!(thai_words,
                   vec![
                       THString::from("กา"),
                       THString::from("เก")
                   ]
        );
    }
    #[test]
    fn test_thai_words_sorting4(){
        let mut thai_words : Vec<THString> = vec![
            THString::from("ไต่ไม้"),
            THString::from("ไต้ก๋ง")
        ];
        thai_words.sort_by(|a,b|a.partial_cmp(b).unwrap());
        assert_eq!(thai_words,
                   vec![
                       THString::from("ไต้ก๋ง"),
                       THString::from("ไต่ไม้")
                   ]
        );
    }
}
