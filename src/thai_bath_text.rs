use crate::thai_text_number_utility::text_number_to_thai_words;

static BAHT_UNITS: &'static [&str] = &["บาท", "สตางค์"];
const ONLY_BAHT: &str = "ถ้วน";

pub fn bath_text(baht: f64) -> String {
    //truncate to 2 digits
    let mut str_bath = format!("{:.2}", f64::trunc(baht * 100.0) / 100.0);
    //
    let mut numbers: Vec<&str> = str_bath.as_str().split(".").collect();
    let mut str_bath_text = String::new();
    for (index, number) in numbers.iter().enumerate() {
        if index == 0 {
            //baht
            let thai_word = text_number_to_thai_words(number);
            str_bath_text.push_str(thai_word.as_str());
            str_bath_text.push_str(BAHT_UNITS[index]);
        } else {
            //stang
            let number_int: u8 = number.parse().unwrap();
            if number_int == 0 {
                str_bath_text.push_str(ONLY_BAHT);
            } else {
                let thai_word = text_number_to_thai_words(number);
                str_bath_text.push_str(thai_word.as_str());
                str_bath_text.push_str(BAHT_UNITS[index]);
            }
        }
    }
    str_bath_text
}

pub fn parse_text_bath() -> f64 {
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bath_text() {
        assert_eq!(bath_text(0.), "ศูนย์บาทถ้วน");
        assert_eq!(bath_text(0.0), "ศูนย์บาทถ้วน");
        assert_eq!(bath_text(0.00), "ศูนย์บาทถ้วน");
        assert_eq!(bath_text(1.00), "หนึ่งบาทถ้วน");
        assert_eq!(bath_text(1.235), "หนึ่งบาทยี่สิบสามสตางค์");
        assert_eq!(
            bath_text(2123456123451.00),
            "สองล้านหนึ่งแสนสองหมื่นสามพันสี่ร้อยห้าสิบหกล้านหนึ่งแสนสองหมื่นสามพันสี่ร้อยห้าสิบเอ็ดบาทถ้วน"
        );
        assert_eq!(
            bath_text(100011030510.0),
            "หนึ่งแสนสิบเอ็ดล้านสามหมื่นห้าร้อยสิบบาทถ้วน"
        );
        assert_eq!(bath_text(10.50), "สิบบาทห้าสิบสตางค์");
    }
}
