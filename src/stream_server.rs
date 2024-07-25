use std::collections::HashMap;

use actix::{Message, Recipient};

use crate::time;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct DataMessage(pub u64, pub Vec<u8>);

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct StreamMessage(pub u64, pub String);

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct AddStreamer(pub u64, pub Recipient<DataMessage>);

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct StreamEnded();

#[derive(Debug, Clone)]
pub struct StreamServer {
    streamers: HashMap<u64, Recipient<DataMessage>>,
}

impl StreamServer {
    pub fn new() -> Self {
        Self {
            streamers: HashMap::new(),
        }
    }
}

impl actix::Actor for StreamServer {
    type Context = actix::Context<Self>;
}

impl actix::Handler<DataMessage> for StreamServer {
    type Result = ();
    
    fn handle(&mut self, msg: DataMessage, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let data = msg.1;
        for (_, recipient) in &self.streamers {
            recipient.do_send(DataMessage(id, data.clone()));
        }
    }
}

impl actix::Handler<AddStreamer> for StreamServer {
    type Result = ();

    fn handle(&mut self, msg: AddStreamer, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let recipient = msg.1;
        let recipient_hash = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let mut hasher = DefaultHasher::new();
            recipient.hash(&mut hasher);
            hasher.finish()
        };
        log::info!("adding streamer (id: {}, recipient hash: {:?})", id, recipient_hash);
        self.streamers.insert(id, recipient);
    }
}

impl actix::Handler<StreamEnded> for StreamServer {
    type Result = ();

    fn handle(&mut self, _msg: StreamEnded, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("stream ended, removing unconnected streamers...");
        self.streamers.retain(|_, streamer| streamer.connected());
    }
}

// impl actix::Handler<StreamMessage> for StreamServer {
//     type Result = ();

//     fn handle(&mut self, msg: StreamMessage, _ctx: &mut Self::Context) -> Self::Result {
//         // This is step 2 of the 3 steps we perform when we receive an image from 
//         // a device client. Here we are iterating through our monitor clients and sending
//         // the image to all of them. Remember - do not confuse device clients and monitor clients.
//         log::debug!("sending data to stream sessions, total sessions: {}", self.streamers.len());
//         let now_ts = time::current_ts_millis();
//         for (key, streamer) in self.streamers.iter_mut() {
//             if streamer.connected() {
//                 let recipient_hash = {
//                     use std::collections::hash_map::DefaultHasher;
//                     use std::hash::Hash;
//                     use std::hash::Hasher;
        
//                     let mut hasher = DefaultHasher::new();
//                     streamer.hash(&mut hasher);
//                     hasher.finish()
//                 };
//                 log::debug!("sending data to stream session, browser id {} hash {}", key, recipient_hash);
//                 match streamer.try_send(msg.clone()) {
//                     Ok(_) => {
//                         // log::debug!("sent data to stream session, browser id {} hash {}", key, recipient_hash);
//                     },
//                     Err(e) => {
//                         log::error!("error sending data to stream session, browser id {} hash {}, error: {}", key, recipient_hash, e);
//                     }
//                 }
//             } else {
//                 log::warn!("streamer not connected: {}", key);
//             }
//         }
//         if log::log_enabled!(log::Level::Debug) {
//             log::debug!("send complete in {} ms", time::current_ts_millis() - now_ts);
//         }
//     }
// }