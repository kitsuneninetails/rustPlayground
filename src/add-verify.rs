extern crate riker;
extern crate chrono;
extern crate http_api_server;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use riker::actors::*;
use chrono::{Utc, DateTime, Duration as CDuration};
use http_api_server::{Client, MIME_JSON, Headers, NullObject, HttpMethod,
                      Authorization, Basic, Bearer,
                      start_server_with_state, CustomResponse, Status};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

// API Structs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddrVerification {
    pub req_id: String,
    pub address: String,
    pub cb_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddrToVerify {
    pub address: String,
    pub callback_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestId {
    pub id: String,
}

fn main() {
    let login_url: String = "https://address.paidyx.io/login".into();
    let base_url: String = "https://address.paidyx.io/addresses".into();
    let cl = Client::from_url(&base_url);

    let mut hdrs = Headers::new();
    hdrs.set(Authorization(Basic {
        username: "admin".into(),
        password: Some("Zebras are horses with multiple personality disorder".into())
    }));

    let admin_token = cl.with_headers::<NullObject, Token>(
        login_url.as_ref(),
        HttpMethod::PostEmpty,
        hdrs).unwrap().into_body().unwrap().token;
    let svc_token = "pk_cGFpZHlmaW5hbmNlOg==".to_string();

    let mut post_headers = Headers::new();
    post_headers.set(Authorization(Bearer { token: svc_token }));

    let mut get_headers = Headers::new();
    get_headers.set(Authorization(Bearer { token: admin_token }));

    // Test Get Request
    let request = AddrToVerify {
        address: "東京都港区六本木１ー１ー3".into(),
        callback_url: Some("http://test-callback.paidyx.io/callback".into()),
    };

    let ret = cl.with_headers::<AddrToVerify, RequestId>(
        base_url.as_ref(),
        HttpMethod::Post(request, MIME_JSON),
        post_headers.clone()).unwrap().into_body().unwrap();

    let deadline = Utc::now() + CDuration::seconds(10);
    let test_doc: AddrVerification = loop {
        let res = cl.with_headers::<NullObject, AddrVerification>(
            format!("{}/{}", base_url, ret.id).as_ref(),
            HttpMethod::Get, get_headers.clone());
        if let Ok(d) = res {
            break d.into_body().unwrap();
        }
        assert!(Utc::now() < deadline);
    };

    println!("Added record = {:?}", test_doc);
}
