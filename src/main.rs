use std::thread;

extern crate consul;

pub mod aws;
pub mod write_kv;

fn main() {

    // Get any config 

    // spin off thread to fetch from aws sqs
    let queue = String::from("test-queue");

    println!("About to kick off thread");

    let aws_thread = thread::spawn(move || {
        aws::sqs_fetch(queue);
    });

    println!("Thread started");

    aws_thread.join().unwrap(); 
    
    println!("Thread finished");
}
