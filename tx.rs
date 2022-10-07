//coordinates and sequentialize the messages from multiple peers
//and transmits those messages to the peers subscribed to the stream.

pub struct MessageStream{
    
}

pub struct MessageSubject{
    label: [char; 8],
    message_stream: MessageStream
}

pub struct MessageOrder{}



pub fn comparator(msg_a: &MessageOrder, msg_b: &MessageOrder) -> bool{
    false
}