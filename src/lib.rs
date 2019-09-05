

//#![feature(custom_attribute)]
//#![feature(plugin)]
//#![feature(custom_derive)]
//#![plugin(rocket_codegen)]
//
//extern crate base64;
//extern crate chrono;
//extern crate http_api_server;
//extern crate openssl;
//extern crate rand;
//extern crate rustc_serialize;
//extern crate rocket;
//extern crate rocket_contrib;
//extern crate serde;
//#[macro_use] extern crate serde_derive;
//extern crate serde_json;
//extern crate term_handler;
//extern crate uuid;
//
//use chrono::{DateTime, Utc};
//use http_api_server::{CustomResponse, Status, start_server_with_state,
//                      Client, MIME_JSON};
//use openssl::aes::*;
//use openssl::symm::*;
//use openssl::rand::rand_bytes;
//use rocket::State;
//use rocket_contrib::Json;
//use serde_json::{from_str, to_string};
//use std::thread::spawn;
//use term_handler::wait_for_term;
//
//#[derive(Serialize, Deserialize, Clone, Debug)]
//pub struct MerchantState {
//    recent_code_ids: Vec<String>,
//    purchases: Vec<Purchase>,
//    key: Vec<u8>,
//}
//
//#[derive(Serialize, Deserialize, Clone, Debug)]
//pub struct LatLonAlt {
//    lat: f64,
//    lon: f64,
//    alt: f64,
//}
//impl LatLonAlt {
//    pub fn new(lat: f64, lon: f64, alt: f64) -> LatLonAlt {
//        LatLonAlt { lat, lon, alt }
//    }
//}
//
//#[derive(Serialize, Deserialize, Clone, Debug)]
//pub struct QrData {
//    pub device_id: String,
//    pub enc_data: Vec<u8>,
//}
//
//#[derive(Serialize, Deserialize, Clone, Debug)]
//pub struct Purchase {
//    pub amount: f64,
//    pub consumer_id: String,
//    pub totp: String,
//}
//
//#[derive(Serialize, Deserialize, Clone, Debug)]
//pub struct QrEncData {
//    pub qr_code_id: String,
//    pub created_at: DateTime<Utc>,
//    pub location: LatLonAlt,
//    pub purchase: Vec<u8>,
//}
//
//#[derive(Serialize, Deserialize, Clone, Debug)]
//pub struct PurchaseResult {
//    pub accepted: bool,
//}
//
//#[post("/data", format="application/json", data="<data>")]
//fn http_data(data: Json<QrData>,
//             state: State<MerchantState>) -> CustomResponse {
//
//    let d: QrData = data.into_inner();
//    let enc_data = d.enc_data;
//    let dev_id = d.device_id;
//
//    let dec_bytes = decrypt(Cipher::aes_256_cbc(),
//                            state.key.as_ref(),
//                            Some(make_iv().as_ref()),
//                            enc_data.as_ref()).unwrap();
//    let dec_str = String::from_utf8(dec_bytes).unwrap();
//
//    let qr_info: QrEncData = from_str(&dec_str).unwrap();
//
//    let p_data = decrypt(Cipher::aes_256_cbc(),
//                         state.key.as_ref(),
//                         Some(make_iv().as_ref()),
//                         qr_info.purchase.as_ref()).unwrap();
//
//    let p_obj: Purchase = from_str(&String::from_utf8(p_data).unwrap()).unwrap();
//
//    println!("Received info: {:?}", qr_info);
//    println!("Decoded purchase info: {:?}", p_obj);
//    let res = PurchaseResult {
//        accepted: true
//    };
//
//    CustomResponse::new(res, MIME_JSON, Status::Ok, None)
//}
//
//pub fn make_key() -> Vec<u8> {
//    let mut buf = [0; 32];
//    rand_bytes(&mut buf).unwrap();
//    buf.to_vec()
//}
//
//pub fn make_iv() -> Vec<u8> {
//    vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf]
//}
//
//pub fn consumer_device(key: Vec<u8>) {
//    ::std::thread::sleep(::std::time::Duration::from_secs(2));
//
//    let url = "http://localhost:8080/data".to_string();
//    let client = Client::from_url(&url);
//
//    let p_obj = Purchase {
//        amount: 1000.0f64,
//        consumer_id: "test_consumer".into(),
//        totp: "1234567890".into(),
//    };
//
//    let purchase = encrypt(Cipher::aes_256_cbc(),
//                            key.as_ref(),
//                            Some(make_iv().as_ref()),
//                            to_string(&p_obj).unwrap().as_bytes()).unwrap();
//    let data = QrEncData {
//        created_at: Utc::now(),
//        location: LatLonAlt::new(157.0, 13.4, 23.0),
//        purchase,
//        qr_code_id: format!("{}", uuid::Uuid::new_v4().simple())
//    };
//
//    let data_str = to_string(&data).unwrap();
//    let enc_data = encrypt(Cipher::aes_256_cbc(),
//                           key.as_ref(),
//                           Some(make_iv().as_ref()),
//                           data_str.as_bytes()).unwrap();
//
//    let qr_code = QrData {
//        device_id: "test_device".into(),
//        enc_data
//    };
//
//    let res = client.post::<QrData, PurchaseResult>(url.as_ref(), &qr_code, MIME_JSON).unwrap()
//        .into_body().unwrap();
//
//}
//
//pub fn merchant_device(key: Vec<u8>) {
//
//    let state = MerchantState {
//        recent_code_ids: vec![],
//        purchases: vec![],
//        key,
//    };
//
//    start_server_with_state("127.0.0.1",
//                            8080,
//                            vec![("/".into(), routes![http_data])],
//                            Some(state)).unwrap();
//
//    loop {
//
//    }
//}
//
