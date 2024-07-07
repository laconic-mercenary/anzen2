use std::collections::HashMap;

use actix::{Addr, AsyncContext, Message, Recipient};
use bytes::Bytes;

use crate::time;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct StreamMessage(pub u64, pub String);

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct AddStreamer(pub u64, pub Recipient<StreamMessage>);

#[derive(Debug, Clone)]
pub struct StreamServer {
    streamers: HashMap<u64, Recipient<StreamMessage>>,
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

impl actix::Handler<AddStreamer> for StreamServer {
    type Result = ();

    fn handle(&mut self, msg: AddStreamer, _ctx: &mut Self::Context) -> Self::Result {
        // ADD-STREAMER-2
        // Received a new streamer event from the session, add them to the streamers list
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

impl actix::Handler<StreamMessage> for StreamServer {
    type Result = ();

    fn handle(&mut self, msg: StreamMessage, _ctx: &mut Self::Context) -> Self::Result {
        // STREAM-MESSAGE-2
        // Received a new stream message from the session, send it to all streamers
        log::debug!("sending data to stream sessions, total sessions: {}", self.streamers.len());
        let now_ts = time::current_ts_millis();
        let mut to_remove = Vec::<u64>::new();
        for (key, streamer) in self.streamers.iter_mut() {
            if streamer.connected() {
                let recipient_hash = {
                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::Hash;
                    use std::hash::Hasher;
        
                    let mut hasher = DefaultHasher::new();
                    streamer.hash(&mut hasher);
                    hasher.finish()
                };
                log::debug!("sending data to stream session, browser id {} hash {}", key, recipient_hash);
                match streamer.try_send(msg.clone()) {
                    Ok(_) => {
                        // log::debug!("sent data to stream session, browser id {} hash {}", key, recipient_hash);
                    },
                    Err(e) => {
                        log::error!("error sending data to stream session, browser id {} hash {}, error: {}", key, recipient_hash, e);
                    }
                }
            } else {
                log::warn!("streamer not connected: {} - will remove", key);
                to_remove.push(key.clone());
            }
        }
        for id in to_remove {
            self.streamers.remove(&id);
        }
        log::debug!("send complete in {} ms", time::current_ts_millis() - now_ts);
    }
}