use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use serde_json::value::RawValue;
use reqwest::Client as httpClient;
use reqwest::Url;
use jsonwebtoken::{encode, Header};

const BASE_URL : &str = "https://b1.run/api/v3/";

#[derive(Debug, Deserialize, Serialize)]
pub struct APIResponse<'a> {
  code: u32,
  message: Option<String>,
  #[serde(borrow)]
  data: &'a RawValue,
}

pub struct Client {
  key: String,
  secret: String,
  http_client: httpClient,
}

impl Client {
  pub fn request(&self, path: String) {
    let url = Url::parse(format!("{}{}", BASE_URL, path).as_str()).expect("request url is invalid");
    println!("url is {}", url);
    let mut request = reqwest::Request::new(reqwest::Method::GET, url);
    let token = sign(&self.key, &self.secret);
    println!("token is {}", token);
    let auth_header = format!("Bearer {}", token);
    let header_value = reqwest::header::HeaderValue::from_str(&auth_header).expect("auth header invalid");
    request.headers_mut().insert(reqwest::header::AUTHORIZATION, header_value);
    let body = self.http_client.execute(request).unwrap().text().unwrap();
    println!("body is {}", body);
    let resp: APIResponse = serde_json::from_str(&body).unwrap();
    println!("body = {:?}", resp);
  }
}

pub fn new(key: String, secret: String) -> Client {
  Client{
    key,
    secret,
    http_client: httpClient::new()
  }
}

#[derive(Debug, Serialize)]
struct Claims {
  r#type: String,
  sub: String,
  nonce: String,
  recv_window: String
}

fn sign(key: &str, secret: &str) -> String {
  let api_claims = Claims {
    r#type: String::from("OpenAPIV2"),
    sub: key.to_string(),
    nonce: get_nonce(),
    recv_window: String::from("30")
  };

  encode(&Header::default(), &api_claims, secret.as_bytes()).expect("invalid token")
}

fn get_nonce() -> String {
  let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!");
  duration.as_nanos().to_string()
}