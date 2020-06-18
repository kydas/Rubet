//use rusoto_core;
//use rusoto_sqs;

use rusoto_core::Region;
use rusoto_sqs::{ Sqs, SqsClient, ReceiveMessageRequest, ReceiveMessageResult, Message};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use crate::write_kv::write_to_kv;


pub async fn sqs_fetch(queue_url: String) {

    let region = Region::UsWest1;

    let sqs = SqsClient::new(region);

    // MESSAGE REQUIRED VALUES: 
    let max_messages = 50;
    let visibility_to = 10;
    let wait_time = 5;

    let (delete_send, delete_recv): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    let (write_send, write_recv): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    let kv_writer = std::thread::spawn(move || {
        write_to_kv(write_recv, delete_send);
    });

    loop {

        let message_attributes = vec![String::from("All")];

        let message_request = ReceiveMessageRequest {
            attribute_names: Some(message_attributes),
            max_number_of_messages: Some(max_messages),
            message_attribute_names: None,
            queue_url: queue_url.clone(),
            receive_request_attempt_id: None,
            visibility_timeout: Some(visibility_to),
            wait_time_seconds: Some(wait_time)

        };

        let received_messages: Vec<Message>; 
        

        match sqs.receive_message(message_request).await {
            Ok(output) => {
                match output.messages {
                    Some(message_list) => {
                        println!("Messages received");

                        for message in message_list {
                            write_send.send(message);
                        }
                    },
                    None => println!("No messages in queue!")
                }
            },
            Err(e) => println!("error retrieving messages: {}", e),
        };

        let to_delete_iter = delete_recv.iter();

        for message in to_delete_iter {
            println!("Deleted message: {}", message.body.unwrap());
            // !! TODO: Actually delete 
        }

    }

    
    let mut x = 0;

    while x < 10 {
        println!("{}: {}", queue_url, x);
        x += 1;
    }
}