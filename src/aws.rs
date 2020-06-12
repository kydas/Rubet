//use rusoto_core;
//use rusoto_sqs;

use rusoto_core::Region;
use rusoto_sqs::SqsClient;
use rusoto_sqs::ReceiveMessageRequest;
use rusoto_sqs::Sqs;
use core::pin::Pin;



pub async fn sqs_fetch(queue_url: String) {

    let region = Region::UsWest1;

    let sqs = SqsClient::new(region);

    // MESSAGE ATTRIBUTES: 
    let mut message_attributes = Vec::new();
    message_attributes.push(String::from("All"));
    let max_messages = 50;
    let visibility_to = 10;
    let wait_time = 5;

    let message_request = ReceiveMessageRequest {
        attribute_names: Some(message_attributes),
        max_number_of_messages: Some(max_messages),
        message_attribute_names: None,
        queue_url: queue_url.clone(),
        receive_request_attempt_id: None,
        visibility_timeout: Some(visibility_to),
        wait_time_seconds: Some(wait_time)

    };

    // let message_future_pin = Sqs::receive_message(&sqs, message_request);

    //let future = sqs.receive_message(message_request).sync();


    match sqs.receive_message(message_request).await {
        Ok(output) => {
            match output.messages {
                Some(message_list) => {
                    println!("Messages in list:");

                    for message in message_list {
                        println!("{}", message.body.unwrap());
                    }
                },
                None => println!("No messages in queue!")
            }
        },
        Err(e) => println!("error retrieving messages: {}", e),
    }; 


    // let message_future = Pin::into_inner_unchecked(message_future_pin);


    
    let mut x = 0;

    while x < 10 {
        println!("{}: {}", queue_url, x);
        x += 1;
    }
}