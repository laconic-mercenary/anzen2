
use actix::{Addr, Handler, StreamHandler};
use actix_http::ws::Item;
use actix_web_actors::ws;
use actix::AsyncContext;

use crate::{images, message_types::{DataFrame, CONN_DEVICE_FRAME_TYPE, CONN_STREAM_FRAME_TYPE, VIDEO_FRAME_TYPE}, stream_server::{AddStreamer, DataMessage, StreamEnded, StreamMessage, StreamServer}};

pub struct StreamSession {
    server: Addr<StreamServer>,
    buffer: Vec<u8>,
    is_fragmented: bool,
}

impl StreamSession {
    pub fn new(server: Addr<StreamServer>) -> Self {
        Self { 
            server,
            buffer: Vec::<u8>::new(), 
            is_fragmented: false
        }
    }
}

impl actix::Actor for StreamSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // this will fire when a client connects to our server using 
        // websocket protocol. Note that this fires AFTER the 
        // start_websocket_monitor callback found in the http_server.rs
        log::info!("websocket session started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("websocket session stopped");
        self.server.do_send(StreamEnded());
    }
}

impl Handler<StreamMessage> for StreamSession {
    type Result = ();

    fn handle(&mut self, msg: StreamMessage, ctx: &mut Self::Context) -> Self::Result {
        // This is STEP 3 of the steps that happen when we receive an image from the device.
        // We require the session because it has access to the context (ctx) - which you can 
        // consider to be the actual push websocket. Previously the StreamMessage came from 
        // the stream_server.rs.
        log::debug!("sending data to client browser");
        let stream_id = msg.0;
        let image_data = msg.1;
        let frame = DataFrame::new(VIDEO_FRAME_TYPE, stream_id, image_data);
        let text = serde_json::to_string(&frame).unwrap();
        ctx.text(text) // this is the actual websocket object
    }
}

impl Handler<DataMessage> for StreamSession {
    type Result = ();

    fn handle(&mut self, msg: DataMessage, ctx: &mut Self::Context) -> Self::Result {
        // This is STEP 3 of the steps that happen when we receive an image from the device.
        // We require the session because it has access to the context (ctx) - which you can
        // consider to be the actual push websocket. Previously the StreamMessage came from
        // the stream_server.rs.
        log::debug!("sending data to client browser");
        let sender_id = msg.0;
        let image_data = msg.1;
        let base64_encoded = base64::encode(&image_data);
        let frame = DataFrame::new(VIDEO_FRAME_TYPE, sender_id, base64_encoded);
        let text = serde_json::to_string(&frame).unwrap();
        ctx.text(text) // this is the actual websocket object
    }
}

fn jpeg_to_data_url(jpeg_bytes: Vec<u8>) -> String {
    let base64_encoded = base64::encode(&jpeg_bytes);
    format!("data:image/jpeg;base64,{}", base64_encoded)
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for StreamSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Binary(_msg)) => {
                log::warn!("[StreamSession] stream message binary");
            },
            Ok(ws::Message::Text(msg)) => {
                log::debug!("received text message {}", msg);
                let recipient = ctx.address().recipient();
                let result: Result<DataFrame, serde_json::Error> = serde_json::from_str(&msg);
                match result {
                    Ok(frame) => {
                        let stream_type = frame.stream_type();
                        if stream_type == CONN_STREAM_FRAME_TYPE {
                            let stream_id = frame.sender_id();
                            log::info!("received connection from streamer, streamer id is {}", stream_id);
                            let add_streamer = AddStreamer(stream_id, recipient);
                            if let Err(err) = self.server.try_send(add_streamer) {
                                log::error!("[StreamSession] stream message text error {}", err);
                            }
                        } else if stream_type == CONN_DEVICE_FRAME_TYPE {
                            // At the moment there is no need to keep track of devices
                            // connected, but a helpful log is at least a good idea
                            let device_id = frame.sender_id();
                            log::info!("received connection from device, id is {}", device_id);
                            // no need for message sending (for now)
                        } else {
                            log::warn!("unknown stream type {}", stream_type);
                        }
                    },
                    Err(err) => {
                        log::error!("stream message text error {}", err);
                    }
                }
            },
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            },
            Ok(ws::Message::Close(_msg)) => {
                log::info!("stream message close");
            },
            Ok(ws::Message::Continuation(item)) => {
                match item {
                    Item::FirstBinary(bytes) | Item::FirstText(bytes) => {
                        self.buffer.extend_from_slice(&bytes);
                        self.is_fragmented = true;
                    }
                    Item::Continue(bytes) => {
                        if self.is_fragmented {
                            self.buffer.extend_from_slice(&bytes);
                        }
                    }
                    Item::Last(bytes) => {
                        if self.is_fragmented {
                            self.buffer.extend_from_slice(&bytes);
                            self.is_fragmented = false;
                            let mut image_data = self.buffer.split_off(0);
                            let sender_id_bytes = image_data.iter().rev().take(7).cloned().collect::<Vec<u8>>();
                            let sender_id = bytes_to_integer(&sender_id_bytes);
                            image_data.truncate(image_data.len() - 7);
                            log::debug!("Sender ID: {:?}, Image data length: {}", sender_id, image_data.len());
                            let data_msg = DataMessage(sender_id, image_data);
                            self.server.do_send(data_msg);
                        }
                    }
                }
                log::debug!("stream message continuation");
            },
            Err(err) => {
                log::error!("Protocol Error: {:?} - {}", err, err.to_string());
            }
            _ => {
                log::warn!("stream message unknown");
            }
        }
    }
}
fn bytes_to_integer(bytes: &[u8]) -> u64 {
    let s = bytes.iter().map(|&b| (b + b'0') as char).rev().collect::<String>();
    s.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_integer_single_digit() {
        let bytes = vec![5];
        assert_eq!(bytes_to_integer(&bytes), 5);
    }

    #[test]
    fn test_bytes_to_integer_multiple_digits() {
        let bytes = vec![1, 2, 3, 4, 5];
        assert_eq!(bytes_to_integer(&bytes), 12345);
    }

    #[test]
    fn test_bytes_to_integer_zero() {
        let bytes = vec![0];
        assert_eq!(bytes_to_integer(&bytes), 0);
    }

    #[test]
    fn test_bytes_to_integer_large_number() {
        let bytes = vec![9, 9, 9, 9, 9, 9, 9, 9, 9];
        assert_eq!(bytes_to_integer(&bytes), 999999999);
    }

    #[test]
    #[should_panic(expected = "ParseIntError")]
    fn test_bytes_to_integer_invalid_input() {
        let bytes = vec![255]; // This will result in an invalid character
        bytes_to_integer(&bytes);
    }

    #[test]
    fn test_bytes_to_integer_empty_input() {
        let bytes: Vec<u8> = vec![];
        assert_eq!(bytes_to_integer(&bytes), 0);
    }
}
