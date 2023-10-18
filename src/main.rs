use serde_json::{json};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use surf;
use tokio;

async fn send_message(url: &str, title: &str, text: &str) {
    let client = surf::Client::new();
    client
        .post(url)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(json!({
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
        }))
        .send()
        .await
        .expect("Failed to send message");
}

#[tokio::main]
async fn main() {
    let url = std::env::var("FEISHU_WEBHOOK_URL").expect("FEISHU_WEBHOOK_URL is not set");
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 1 {
        return;
    }
    let child = Command::new(&args[1])
        .args(&args[2..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let command = args[1..].join(" ");
    let (title, message) = match child {
        Ok(mut res) => {
            let mut stdout = String::new();
            let mut stderr = String::new();

            let mut lines = BufReader::new(res.stdout.take().unwrap()).lines();
            while let Some(line) = lines.next() {
                stdout.push_str(&(line.unwrap() + "\n"));
            }
            let mut lines = BufReader::new(res.stderr.take().unwrap()).lines();
            while let Some(line) = lines.next() {
                println!("{}", &line.as_ref().unwrap());
                stderr.push_str(&(line.unwrap() + "\n"));
            }
            (
                format!("Command execution result for: `{}`", command),
                format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr),
            )
        }
        Err(e) => (
            format!("Command `{}` failed to execute: ", command),
            format!("{:?}", e),
        ),
    };
    send_message(&url, &title, &message).await;
}
