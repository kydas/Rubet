use rusoto_sqs::Message;
use std::sync::mpsc;

pub fn write_to_kv(write_channel: mpsc::Receiver<Message>, delete_channel: mpsc::Sender<Message>){
    // stub
}