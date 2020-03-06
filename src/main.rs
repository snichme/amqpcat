use quicli::prelude::*;
use std::io;
use structopt::StructOpt;

use lapin::{options::*, BasicProperties, Channel, Connection, ConnectionProperties};

mod consumer;

/// amqpcat is a AMQP cli producer and consumer, like netcat but for AMQP.
#[derive(Debug, StructOpt)]
pub struct Cli {
    /// amqp(s)://127.0.0.1:5672/
    url: String,

    /// Mode: consume or produce
    #[structopt(long = "mode", short = "m", default_value = "consume")]
    mode: String,

    /// Consume or produce to this queue
    #[structopt(long = "queue", short = "q", default_value = "")]
    queue: String,

    /// In consume mode, will create a temp queue and bind to amq.topic with the given routing key,
    /// In produce mode, will produce messages into amq.topic with the given routing key
    #[structopt(long = "exchange", short = "e", default_value = "amq.direct")]
    exchange: String,

    /// In consume mode, will create a temp queue and bind to amq.topic with the given routing key,
    /// In produce mode, will produce messages into amq.topic with the given routing key,
    /// when set will set the --exchange to amq.topic
    #[structopt(long = "routing-key", short = "r", default_value = "")]
    routing_key: String,

    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn produce(ch: Channel, args: Cli) {
    let mut guess = String::new();
    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        info!("sending {}", guess);
        ch.basic_publish(
            "amq.topic",
            &args.routing_key,
            BasicPublishOptions::default(),
            guess.as_bytes().to_vec(),
            BasicProperties::default(),
        )
        .wait()
        .expect("basic_publish");
        guess = "".to_string()
    }
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("amqpcat")?;

    info!("Connecting to {}", args.url);
    let conn = Connection::connect(&args.url, ConnectionProperties::default())
        .wait()
        .expect("connection error");

    info!("Connected to {}", args.url);
    let channel = conn.create_channel().wait().expect("create_channel");
    info!("state: {:?}", conn.status().state());

    if args.mode == "consume" {
        consumer::consume(channel, args)
    } else if args.mode == "produce" {
        produce(channel, args)
    }

    Ok(())
}
