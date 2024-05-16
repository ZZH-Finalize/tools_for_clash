use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;
use reqwest;
use std::{fs::File, io::Write, process::ExitCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display, strum_macros::EnumString)]
enum ProxyType {
    #[strum(ascii_case_insensitive)]
    ALL = 0,

    #[strum(ascii_case_insensitive)]
    HTTP = 1,

    #[strum(ascii_case_insensitive)]
    HTTPS = 2,

    #[strum(ascii_case_insensitive)]
    Socks4 = 3,

    #[strum(ascii_case_insensitive)]
    Socks5 = 4,
}

const ID_TO_PTYPE: [&str; 5] = ["ALL", "HTTP", "HTTPS", "Socks4", "Socks5"];

#[derive(Debug, Parser)]
#[command(version = "0.0.1")]
#[command(about = "Get proxy infos from website and write to file in plain text.")]
struct Cli {
    #[arg(long)]
    date: Option<String>,

    #[arg(short = 't', long = "type", default_value = "All")]
    p_type: ProxyType,
}

const DATE_FMT: &str = "%Y-%m-%d";

pub fn main() -> ExitCode {
    let mut url = String::from("https://checkerproxy.net/api/archive/");
    let options = Cli::parse();
    let date_use: String;

    match options.date {
        Some(date_string) => {
            let mut year_string = Local::now().year().to_string();
            year_string += "-";
            year_string += &date_string;

            if let Ok(date) = NaiveDate::parse_from_str(&year_string, DATE_FMT) {
                println!("input date is {}", date);
                date_use = date.to_string();
                url += &date_use;
            } else {
                panic!("Input date format error");
            }
        }
        None => {
            let now = Local::now().format(DATE_FMT);
            date_use = now.to_string();
            url += &date_use;
        }
    };

    println!("Sending out request...");

    let resp = reqwest::blocking::get(url);

    println!("Get response");

    match resp {
        Ok(data) => {
            let proxies_data = data.json::<serde_json::Value>().unwrap();
            let proxies = proxies_data.as_array().unwrap();

            println!("Recived {} proxies", proxies.len());

            let mut file_name = String::from("proxies/");
            file_name += &date_use;

            match File::create(&file_name) {
                Ok(mut file) => {
                    let mut write_count = 0;
                    println!("write data to {}", &file_name);

                    for p in proxies {
                        let p_obj = p.as_object().unwrap();
                        let addr = p_obj.get("addr").unwrap().as_str().unwrap();
                        let p_type = p_obj.get("type").unwrap().as_u64().unwrap();

                        if options.p_type != ProxyType::ALL {
                            if p_type != options.p_type as u64 {
                                continue;
                            }
                        }

                        if let Err(e) = writeln!(file, "{}:{}", addr, ID_TO_PTYPE[p_type as usize])
                        {
                            println!("write {} to file {} fail, error: {}", addr, &file_name, e);
                        } else {
                            write_count += 1;
                        }
                    }

                    println!("Write {} proxies to {}", write_count, &file_name);
                }
                Err(e) => {
                    println!("error occurs when create file: {}", &file_name);
                    println!("{}", e);
                }
            }
        }
        Err(e) => {
            println!("error occurs after send request");
            println!("{}", e);
        }
    }

    ExitCode::SUCCESS
}
