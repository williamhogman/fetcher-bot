extern crate rusoto_core;
extern crate rusoto_sqs;

use std::env;

use rusoto_core::RusotoError;
use rusoto_core::Region;
use rusoto_sqs::{SqsClient, ReceiveMessageError, ReceiveMessageRequest, Sqs};

fn main() {
    let queue_url = match env::var("SQS_QUEUE_URL") {
        Ok(x) => x,
        Err(e) => {
            println!("Env-var SQS_QUEUE_URL is required {}", e);
            return;
        }
    };
    let inp = ReceiveMessageRequest {
        queue_url: queue_url,
        ..Default::default()
    };
    let client = SqsClient::new(Region::EuNorth1);

    loop {
        let res = client.receive_message(inp.clone()).sync();
        if let Err(e) = res {
            if let RusotoError::Validation(v) = e {

                println!("Terminating due to validation error {}", v);
                return;
            }
            println!("Error fetching messages {:?}", e);
        }
    }
}
