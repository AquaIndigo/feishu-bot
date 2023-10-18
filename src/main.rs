use std::io::BufReader;
use surf;
use serde_json::{json, Value};
use tokio;

async fn send_message(url:&str, title: &str, text: &str) {
    let client = surf::Client::new();
    let request = client.post(url).header(
        "Content-Type",
        "application/json; charset=utf-8",
    ).body(json!({
        "msg_type": "post",
        "content": {
            "post": {
                "zh_cn" : {
                    "title": title,
                    "content": [
                        [
                            {
                                "tag": "text",
                                "text": text
                            }
                        ]
                    ]
                }
            }
        }
    })).send().await;
}

#[tokio::main]
async fn main() {
    let cfg = BufReader::new(std::fs::File::open("config.json").unwrap());
    let json: Value = serde_json::from_reader(cfg).unwrap();
    let url = json["feishu_url"].as_str().unwrap();


    // send_message(url, "Hello", "World").await;
}
