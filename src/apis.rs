use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::Json;
use serde::{Serialize, Deserialize};
use crate::thai_bath_text::bath_text;
use crate::thai_encoding::THString;

#[derive(Debug, Deserialize, Serialize)]
pub struct BahtTextRequest {
    pub baht_amount: Option<f64>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BathTextResponse {
    pub baht_text: String
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ThaiWordsRequest {
    pub texts : Option<Vec<String>>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ThaiWordsResponse {
    pub texts_sorted : Option<Vec<String>>
}

#[post("/v1/thai_nlp/baht_text")]
pub async fn do_baht_text_handler(req: Json<BahtTextRequest>) -> HttpResponse {
    if req.baht_amount.is_some() {
        let result = bath_text(req.baht_amount.unwrap());
        let response = BathTextResponse{
            baht_text: result
        };
        return HttpResponse::Ok().json(response);
    }
    HttpResponse::BadRequest().finish()
}
#[post("/v1/thai_nlp/text_sorting")]
pub async fn do_text_sorting_handler(req: Json<ThaiWordsRequest>) -> HttpResponse {
    if req.texts.is_some() {
        let mut thai_words : Vec<THString> = Vec::new();
        for text in req.texts.as_ref().unwrap() {
            thai_words.push(THString::from(text.as_str()));
        }
        thai_words.sort_by(|a,b|a.partial_cmp(b).unwrap());
        ////
        let mut response_thai_words : Vec<String> = Vec::new();
        for thai_word_item in thai_words {
            response_thai_words.push(thai_word_item.to_string());
        }
        ////
        return HttpResponse::Ok().json(
            ThaiWordsResponse{
                texts_sorted: Some(response_thai_words)
            }
        )
    }
    HttpResponse::BadRequest().finish()
}