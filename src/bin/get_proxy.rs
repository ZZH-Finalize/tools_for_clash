use chrono::Local;
use reqwest::{
    self,
    blocking::Client,
    header::{HeaderMap, HeaderValue},
    Method, Url,
};
use serde_json::Value;
use std::{cmp::Ordering, fs::File, io::Write, process::ExitCode, str::FromStr};

// "Accept: */*",
// "Accept-Encoding: gzip, deflate, br, zstd",
// "Accept-Language: zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7,zh-TW;q=0.6,en-GB;q=0.5",
// "Dnt: 1",
// "Priority: u=1, i",
// "Referer: https://checkerproxy.net/archive/2024-05-15",
// "Sec-Ch-Ua: "Chromium";v="124", "Microsoft Edge";v="124", "Not-A.Brand";v="99"",
// "Sec-Ch-Ua-Mobile: ?0",
// "Sec-Ch-Ua-Platform: "Windows"",
// "Sec-Fetch-Dest: empty",
// "Sec-Fetch-Mode: cors",
// "Sec-Fetch-Site: same-origin",
// "Sec-Gpc: 1",
// "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0",
// "Host: checkerproxy.net",
// const HEADERS: [&str; 2] = [
//     "Referer: https://checkerproxy.net/archive/",
//     "Host: checkerproxy.net",
// ];

pub fn cmp_by_timeout(x: &Value, y: &Value) -> Ordering {
    let x_tm = x.as_u64().unwrap();
    let y_tm = y.as_u64().unwrap();

    if x_tm > y_tm {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

pub fn main() -> ExitCode {
    let mut url = String::from_str("https://checkerproxy.net/api/archive/").unwrap();
    let now = Local::now().format("%Y-%m-%d");
    url += &now.to_string();

    let resp = reqwest::blocking::get(url);

    match resp {
        Ok(data) => {
            let proxies_data = data.json::<serde_json::Value>().unwrap();
            let proxies = proxies_data.as_array().unwrap();

            println!("{:#}", proxies[0]);
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    ExitCode::SUCCESS
}
