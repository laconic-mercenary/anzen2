use std::collections::HashMap;

use actix::{Message, Recipient};

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ImageReadyEvent(pub u64, pub Vec<u8>);

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct AddMonitorClientEvent(pub u64, pub Recipient<ImageReadyEvent>);

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct VideoSessionEndedEvent();

#[derive(Debug, Clone)]
pub struct StreamServer {
    connected_monitor_clients: HashMap<u64, Recipient<ImageReadyEvent>>,
}

impl StreamServer {
    pub fn new() -> Self {
        Self {
            connected_monitor_clients: HashMap::new(),
        }
    }
}

impl actix::Actor for StreamServer {
    type Context = actix::Context<Self>;
}

impl actix::Handler<ImageReadyEvent> for StreamServer {
    type Result = ();
    
    fn handle(&mut self, msg: ImageReadyEvent, _ctx: &mut Self::Context) -> Self::Result {
        let device_sender_id = msg.0;
        let image_data = msg.1;
        for (_monitor_client_id, monitor_client) in &self.connected_monitor_clients {
            if let Err(err) = monitor_client.try_send(ImageReadyEvent(device_sender_id, image_data.clone())) {
                log::error!("failed to send image to monitor client: {:?}", err);
            }
        }
    }
}

impl actix::Handler<AddMonitorClientEvent> for StreamServer {
    type Result = ();

    fn handle(&mut self, msg: AddMonitorClientEvent, _ctx: &mut Self::Context) -> Self::Result {
        let monitor_client_id = msg.0;
        let monitor_client = msg.1;
        let monitor_hash = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let mut hasher = DefaultHasher::new();
            monitor_client.hash(&mut hasher);
            hasher.finish()
        };
        log::info!("adding monitor client (id: {}, hash: {:?})", monitor_client_id, monitor_hash);
        self.connected_monitor_clients.insert(monitor_client_id, monitor_client);
    }
}

impl actix::Handler<VideoSessionEndedEvent> for StreamServer {
    type Result = ();

    fn handle(&mut self, _msg: VideoSessionEndedEvent, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("session ended, removing disconnected monitor clients...");
        self.connected_monitor_clients.retain(|_, mtr_client| mtr_client.connected());
    }
}
