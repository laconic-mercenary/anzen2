
use actix::{Addr, Handler, Recipient, StreamHandler};
use actix_http::ws::Item;
use actix_web_actors::ws;
use actix::AsyncContext;
use bytes::Bytes;

use crate::{images, client_message::{ClientMessage, DEVICE_CONNECTION_TYPE, MONITOR_CONNECTION_TYPE, IMAGE_READY_TYPE}, stream_server::{AddMonitorClientEvent, ImageReadyEvent, VideoSessionEndedEvent, StreamServer}};

pub struct VideoSession {
    server: Addr<StreamServer>,
    buffer: Vec<u8>,
    is_message_fragmented: bool,
}

impl VideoSession {
    pub fn new(server: Addr<StreamServer>) -> Self {
        Self { 
            server,
            buffer: Vec::<u8>::new(), 
            is_message_fragmented: false
        }
    }
}

impl actix::Actor for VideoSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // this will fire when a client connects to our server using 
        // websocket protocol. Note that this fires AFTER the 
        // start_websocket_monitor callback found in the http_server.rs
        log::info!("websocket session started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("websocket session stopped");
        self.server.do_send(VideoSessionEndedEvent());
    }
}

impl Handler<ImageReadyEvent> for VideoSession {
    type Result = ();

    fn handle(&mut self, msg: ImageReadyEvent, ctx: &mut Self::Context) -> Self::Result {
        // This is STEP 3 of the steps that happen when we receive an image from the device.
        // We require the session because it has access to the context (ctx) - which you can
        // consider to be the actual push websocket. Previously the StreamMessage came from
        // the stream_server.rs.
        log::debug!("sending data to client browser");
        let sender_id = msg.0;
        let image_data = msg.1;
        let image_data_b64 = base64::encode(&image_data);
        let outbound_msg = ClientMessage::new(IMAGE_READY_TYPE, sender_id, image_data_b64);
        let outbound_msg_txt = serde_json::to_string(&outbound_msg).unwrap();
        ctx.text(outbound_msg_txt) // this is the actual websocket object
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for VideoSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Binary(_msg)) => {
                // it's possible that if the image is small enough that 
                // this will call instead of the Continuation block below
                log::warn!("received binary message - currently not supported");
            },
            Ok(ws::Message::Text(msg)) => {
                if log::log_enabled!(log::Level::Debug) {
                    log::debug!("received text message {}", msg);
                }
                match serde_json::from_str(&msg) {
                    Ok(client_msg) => {
                        self.handle_text_message(&client_msg, ctx);
                    },
                    Err(err) => {
                        log::warn!("not a valid client message: err is {} - msg is {}", err, msg);
                    }
                }
            },
            Ok(ws::Message::Ping(msg)) => {
                log::info!("received ping message");
                ctx.pong(&msg);
            },
            Ok(ws::Message::Close(_msg)) => {
                log::info!("client closed the session");
            },
            Ok(ws::Message::Continuation(item)) => {
                // we must manually handle websocket continuations if the data is larger than
                // the max frame size - which seems be around 64KB
                log::trace!("message continuation");
                match item {
                    Item::FirstBinary(bytes) | Item::FirstText(bytes) => {
                        self.begin_continuation(&bytes);
                    }
                    Item::Continue(bytes) => {
                        self.continue_continuation(&bytes);
                    }
                    Item::Last(bytes) => {
                        self.end_continuation(&bytes);                    
                    }
                }
            },
            Err(err) => {
                log::error!("protocol error: {:?} - {}", err, err.to_string());
            }
            _ => {
                log::warn!("stream message unknown");
            }
        }
    }
}

impl VideoSession {
    fn add_monitor_client(&self, monitor_client: Recipient<ImageReadyEvent>, monitor_client_id: u64) {
        log::info!("adding new monitor client, id = {}", monitor_client_id);
        let add_mtr_evt = AddMonitorClientEvent(monitor_client_id, monitor_client);
        if let Err(err) = self.server.try_send(add_mtr_evt) {
            log::error!("error in sending message for adding new monitor client {}", err);
        }
    }

    fn handle_text_message(&self, msg: &ClientMessage, ctx: &mut <VideoSession as actix::Actor>::Context) {
        // recipient is a 'handle' to the websocket session for monitor clients
        // NOTE this is not a handle for the device clients. 
        let recipient = ctx.address().recipient();
        let sender_id = msg.sender_id();
        let connection_type = msg.connection_type();
        if connection_type == MONITOR_CONNECTION_TYPE {
            // we want to maintain a list of monitor clients so we can 
            // broadcast the data from the devices to them
            self.add_monitor_client(recipient, sender_id);
        } else if connection_type == DEVICE_CONNECTION_TYPE {
            // At the moment there is no need to keep track of devices
            // connected, but a helpful log is at least a good idea
            log::info!("received connection from device, id is {}", sender_id);
        } else {
            log::warn!("unknown stream type {}", connection_type);
        }
    }

    fn begin_continuation(&mut self, data: &Bytes) {
        self.buffer.extend_from_slice(data);
        self.is_message_fragmented = true;
    }

    fn continue_continuation(&mut self, data: &Bytes) {
        if self.is_message_fragmented {
            self.buffer.extend_from_slice(data);
        }
    }

    fn end_continuation(&mut self, data: &Bytes) {
        if self.is_message_fragmented {
            self.is_message_fragmented = false;
            self.buffer.extend_from_slice(data);
        
            let mut image_data = self.buffer.split_off(0);
            
            if let Some(sender_id) = images::app::extract_sender_id(&image_data) {
                images::app::remove_sender_id(&mut image_data);
        
                log::debug!(
                    "sender_id: {}, image data length: {}",
                    sender_id,
                    image_data.len()
                );
            
                let img_rdy_evt = ImageReadyEvent(sender_id, image_data);
                if let Err(err) = self.server.try_send(img_rdy_evt) {
                    log::error!("error in sending image ready event: {}", err.to_string());
                }
            }
        }
    }
}