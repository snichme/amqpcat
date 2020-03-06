use crate::Cli;
use quicli::prelude::*;
use std::str;

use lapin::{options::*, types::FieldTable, Channel};

pub fn consume(ch: Channel, args: Cli) {
    let queue_opts = QueueDeclareOptions {
        auto_delete: true,
        durable: false,
        exclusive: false,
        nowait: false,
        passive: false,
    };
    let queue = ch
        .queue_declare(&args.queue, queue_opts, FieldTable::default())
        .wait()
        .expect("queue_declare");
    info!("declared queue {}", queue.name());
    if args.routing_key != "" {
        ch.queue_bind(
            queue.name().as_str(),
            "amq.topic",
            &args.routing_key,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .wait()
        .expect("queue_bind");
        info!("bind queue to {}", args.routing_key);
    }
    info!("starting consumer on queue {}", queue.name());
    let consumer = ch
        .basic_consume(
            queue.name().as_str(),
            "amqpcat",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .wait()
        .expect("basic_consume");
    for delivery in consumer {
        if let Ok(delivery) = delivery {
            println!(
                "[{}] {}: {}",
                delivery.delivery_tag,
                delivery.routing_key,
                str::from_utf8(&delivery.data).unwrap()
            );
            ch.basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .wait()
                .expect("basic_ack");
        }
    }
}
