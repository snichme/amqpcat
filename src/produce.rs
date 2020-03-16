use crate::Cli;
use lapin::{options::*, BasicProperties, Channel};
use quicli::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub fn produce<T: Read>(ch: Channel, args: Cli, source: T) {
    let mut reader = BufReader::new(source);
    let mut buf = String::new();
    loop {
        match reader.read_line(&mut buf) {
            Ok(0) => return,
            Ok(_n) => {
                let d = buf.trim();
                if !d.is_empty() {
                    info!("sending {} to {}", d, args.routing_key);
                    ch.basic_publish(
                        "amq.topic",
                        &args.routing_key,
                        BasicPublishOptions::default(),
                        d.as_bytes().to_vec(),
                        BasicProperties::default(),
                    )
                    .wait()
                    .expect("basic_publish");
                    buf.clear();
                }
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }
}
