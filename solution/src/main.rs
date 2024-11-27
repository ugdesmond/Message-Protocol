use std::num::NonZeroU64;
//import message program crate  
use message_program::{Message, send_instruction}; 
use bytemuck::bytes_of;

fn main() {
    
    let sender_id: [u8; 32] = [
        239, 186, 37, 214, 155, 187, 203, 179, 
        156, 132, 249, 18, 141, 27, 182, 146,
        89, 81, 128, 210, 213, 34, 89, 148,
        12, 167, 128, 175, 170, 207, 148, 105
    ];

    let message = Message {
        sender_id,
        size: 1024,
        priority_fee: NonZeroU64::new(100),
        data: [1u8; 1024],
    };

  
    println!("About to initialize message");

    let mut initialize_data = vec![3];
    initialize_data.extend_from_slice(bytes_of(&message));
    send_instruction(&initialize_data).unwrap();

   
    let mut update_data = vec![6];
    update_data.extend_from_slice(bytes_of(&message));
    send_instruction(&update_data).unwrap();

 
    let mut close_data = vec![9];
    close_data.extend_from_slice(bytes_of(&message));
    send_instruction(&close_data).unwrap();

    //NOTE
    // I firstly ran the code after implement the various instructions for Initialize,Update and close.
    // I got the error attached on the screenshot on the solution directory.
    //The issue is that the size_of function is not in scope
    // I fixed it by adding use std::mem::size_of at lib.rs file.
}
