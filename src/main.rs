use chrono::prelude::{DateTime, NaiveDateTime, Utc};
use futures::StreamExt;
use markov::Chain;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Lines, Read, Write};
use std::path::Path;
use telegram_bot::{Api, CanReplySendMessage, Error, MessageKind, UpdateKind};

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let mut chain = Chain::new();
    let file = File::open("chat.txt");
    match &file {
        Ok(_) => {}
        Err(_) => {
            File::create("chat.txt").unwrap();
        }
    };
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open("chat.txt")
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if let Ok(lines) = read_lines("./chat.txt") {
        for line in lines {
            if let Ok(content) = line {
                chain.feed_str(&content);
            }
        }
    }
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        match update {
            Err(x) => println!("error: {}", x),
            Ok(updated) => {
                if let UpdateKind::Message(message) = updated.kind {
                    let first_name = &message.from.first_name;
                    let message_instance = message.clone();
                    let id = message.chat.id();
                    let date = message.date;
                    let naive = NaiveDateTime::from_timestamp(date, 0);
                    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
                    let newdate = datetime.format("%Y-%m-%d %H:%M");
                    let username = match message.from.username {
                        None => "None".to_string(),
                        Some(x) => x,
                    };
                    if let MessageKind::Text { ref data, .. } = message.kind {
                        chain.feed_str(data);
                        let phrase = chain.generate_str().to_string();
                        println!(
                            "<{}@{}> of {}: {} at {}\n\tand replied: \"{}\"",
                            first_name, username, id, data, newdate, phrase
                        );
                        file.write_all(format!("{}\n", data).as_bytes()).unwrap();
                        match api.send(message_instance.text_reply(phrase)).await {
                            Err(_) => {}
                            Ok(_) => {}
                        };
                    }
                }
            }
        }
    }
    Ok(())
}
