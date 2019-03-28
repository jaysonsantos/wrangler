use super::settings::Settings;
use reqwest::header::CONTENT_TYPE;
use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
struct AccountResponse {
    pub result: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub id: String,
    pub email: String,
    pub has_enterprise_zones: bool,
    pub has_pro_zones: bool,
    pub has_business_zones: bool,
    suspended: bool,
    pub enterprise_zone_quota: Quota,
}

#[derive(Debug, Deserialize)]
pub struct Quota {
    pub maximum: i32,
    current: i32,
    pub available: i32,
}

#[derive(Debug, Deserialize)]
struct ScriptsResponse {
    pub result: Scripts,
}

#[derive(Debug, Deserialize)]
struct Scripts {
    enabled: bool,
    pub multiscript: bool,
}

#[derive(Debug)]
pub struct Account {
    pub data: Data,
    pub multiscript: bool,
}

impl Account {
    pub fn new(settings: Settings) -> Result<Account, failure::Error> {
        let data = data(settings.clone())?;
        let multiscript = script_status(settings, &data)?;
        Ok(Account { data, multiscript })
    }
}

fn script_status(settings: Settings, data: &Data) -> Result<bool, failure::Error> {
    let addr = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/workers/settings",
        data.id
    );
    let client = reqwest::Client::new();

    let mut res = client
        .get(&addr)
        .header("X-Auth-Key", settings.api_key)
        .header("X-Auth-Email", settings.email)
        .header(CONTENT_TYPE, "application/json")
        .send()?;

    let status: ScriptsResponse = serde_json::from_str(&res.text()?)?;
    Ok(status.result.multiscript)
}

fn data(settings: Settings) -> Result<Data, failure::Error> {
    let user_addr = "https://api.cloudflare.com/client/v4/user";
    let client = reqwest::Client::new();

    let mut res = client
        .get(user_addr)
        .header("X-Auth-Key", settings.api_key)
        .header("X-Auth-Email", settings.email)
        .header(CONTENT_TYPE, "application/json")
        .send()?;

    let user: AccountResponse = serde_json::from_str(&res.text()?)?;
    println!("THE RESPONSEEE {:?}", user.result);
    Ok(user.result)
}
