use catenary_tdx_data::TRATrainLiveBoardList;
use core::result::Result;
use reqwest::{header::AUTHORIZATION, header::CONTENT_TYPE, *};
use serde_json::*;
use std::env;
use std::env::consts::ARCH;
use std::error::Error;
use std::{collections::HashMap, fs::File, path::Path};
use iso8601::*;

static AUTH_URL: &str =
    "https://tdx.transportdata.tw/auth/realms/TDXConnect/protocol/openid-connect/token";
static TEST_URL: &str = "https://tdx.transportdata.tw/api/basic/v3/Rail/TRA/TrainLiveBoard";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let raw_path = match ARCH {
        "x86_64" => format!(
            "C:\\Users\\{}\\Downloads\\tdx-secret.json",
            env::var("USERNAME")?
        ),

        &_ => todo!(),
    };
    let file_path = Path::new(&raw_path);
    let file = File::open(file_path).expect("file not found");
    let secret: HashMap<String, String> =
        serde_json::from_reader(file).expect("error while reading");

    let auth_header = json!({
        "grant_type": "client_credentials",
        "client_id": secret.get("client_id").unwrap(),
        "client_secret": secret.get("client_secret").unwrap()
    });

    let client = Client::new();
    let auth_response = client
        .post(AUTH_URL)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&auth_header)
        .send()
        .await?
        .text()
        .await?;

    let data_header = auth_response.split_once("\":\"").unwrap().1;
    let access_token = format!("Bearer {}", data_header.split_once("\",").unwrap().0);

    #[allow(unused)] //
    let data_response = client
        .get(TEST_URL)
        .header(AUTHORIZATION, access_token.clone())
        .send()
        .await?
        .text()
        .await?;

    let data = client
        .get(TEST_URL)
        .header(AUTHORIZATION, access_token)
        .send()
        .await?
        .json::<TRATrainLiveBoardList>()
        .await?;

    print!("{}", datetime(&data.UpdateTime)?);
    Ok(())
}
