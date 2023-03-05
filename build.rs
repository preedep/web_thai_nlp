use std::env;
use std::fs;
use std::path::Path;
use std::char;

const THAI_CHAR_SET_NAMES: &'static [&'static str] = &[
    "KO_KAI",
    "KHO_KHAI",
    "KHO_KHUAT",
    "KHO_KHWAI",
    "KHO_KHON",
    "KHO_RAKHANG",
    "NGO_NGU",
    "CHO_CHAN",
    "CHO_CHING",
    "CHO_CHANG",
    "SO_SO",
    "CHO_CHOE",
    "YO_YING",
    "DO_CHADA",
    "TO_PATAK",
    "THO_THAN",
    "THO_NANGMONTHO",
    "THO_PHUTHAO",
    "NO_NEN",
    "DO_DEK",
    "TO_TAO",
    "THO_THUNG",
    "THO_THAHAN",
    "THO_THONG",
    "NO_NU",
    "BO_BAIMAI",
    "PO_PLA",
    "PHO_PHUNG",
    "FO_FA",
    "PHO_PHAN",
    "FO_FAN",
    "PHO_SAMPHAO",
    "MO_MA",
    "YO_YAK",
    "RO_RUA",
    "RU",
    "LO_LING",
    "LU",
    "WO_WAEN",
    "SO_SALA",
    "SO_RUSI",
    "SO_SUA",
    "HO_HIP",
    "LO_CHULA",
    "O_ANG",
    "HO_NOKHUK",
    "PAIYANNOI",
    "SARA_A",
    "MAI_HAN_AKAT",
    "SARA_AA",
    "SARA_AM",
    "SARA_I",
    "SARA_II",
    "SARA_UE",
    "SARA_UEE",
    "SARA_U",
    "SARA_UU",
    "PHINTHU",
    "SYMBOL_BAHT",
    "SARA_E",
    "SARA_AE",
    "SARA_O",
    "SARA_AI_MAIMUAN",
    "SARA_AI_MAIMALAI",
    "LAKKHANGYAO",
    "MAIYAMOK",
    "MAITAIKHU",
    "MAI_EK",
    "MAI_THO",
    "MAI_TRI",
    "MAI_CHATTAWA",
    "THANTHAKHAT",
    "NIKHAHIT",
    "YAMAKKAN",
    "FONGMAN",
    "ZERO",
    "ONE",
    "TWO",
    "THREE",
    "FOUR",
    "FIVE",
    "SIX",
    "SEVEN",
    "EIGHT",
    "NINE",
    "ANGKHANKHU",
    "KHOMUT"
];

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("thai_charset.rs");
    let mut thai_charset_sr = String::new();
    thai_charset_sr.push_str("pub type THChar = char;\r\n");
    //const KO_KAI: THChar = 'ก';
    for (i,name) in THAI_CHAR_SET_NAMES.iter().enumerate() {

        let mut char_code : u32= (0xe01 as u32) + (i as u32);

        if char_code > 0x0E3A {
           char_code = char_code + (0x0E3F-0x0E3A) - 1;
        }

        println!("{}",char_code);
        let thai_char = char::from_u32(char_code);
        if thai_char.is_some() {
            let source = format!("const {}: THChar = \'{}\'; //{}\r\n", name, thai_char.unwrap(),format!("{:#02x}",char_code));
            thai_charset_sr.push_str(source.as_str());
        }
    }
    //เ,แ,โ,ใ,ไ
    //thai_charset_sr.push_str("const THAI_FRONT_VOWELS: &'static [&'static THChar] = &[&SARA_E, &SARA_AE, &SARA_O, &SARA_AI_MAIMUAN, &SARA_AI_MAIMALAI];\r\n");
    thai_charset_sr.push_str("const THAI_FRONT_VOWELS: &'static [THChar] = &[SARA_E, SARA_AE, SARA_O, SARA_AI_MAIMUAN, SARA_AI_MAIMALAI];\r\n");

    //103	0x0E47	็ (THAI CHARACTER MAITAIKHU)
    //104	0x0E48	่ (THAI CHARACTER MAI_EK)
    //105	0x0E49	้ (THAI CHARACTER MAI_THO)
    //106	0x0E4A	๊ (THAI CHARACTER MAI_TRI)
    //107	0x0E4B	๋ (THAI CHARACTER MAI_CHATTAWA)
    //108	0x0E4C	์ (THAI CHARACTER THANTHAKHAT)
    //109	0x0E4D	ํ (THAI CHARACTER NIKHAHIT)
    //110	0x0E4E	๎ (THAI CHARACTER YAMAKKAN)
    //thai_charset_sr.push_str("const THAI_TONE_VOWELS: &'static [&'static THChar] = &[&MAITAIKHU, &MAI_EK, &MAI_THO, &MAI_TRI, &MAI_CHATTAWA, &THANTHAKHAT, &NIKHAHIT, &YAMAKKAN];\r\n");
    thai_charset_sr.push_str("const THAI_TONE_VOWELS: &'static [THChar] = &[MAITAIKHU, MAI_EK, MAI_THO, MAI_TRI, MAI_CHATTAWA, THANTHAKHAT, NIKHAHIT, YAMAKKAN];\r\n");

    fs::write(&dest_path, thai_charset_sr.as_str()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
