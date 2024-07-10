
use actix::{Addr, Handler, StreamHandler};
use actix_web_actors::ws;
use actix::AsyncContext;

use crate::{message_types::{DataFrame, CONN_FRAME_TYPE, VIDEO_FRAME_TYPE}, stream_server::{AddStreamer, StreamMessage, StreamServer, StreamEnded}};

pub struct StreamSession {
    server: Addr<StreamServer>
}

impl StreamSession {
    pub fn new(server: Addr<StreamServer>) -> Self {
        Self { server }
    }
}

impl actix::Actor for StreamSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
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
        //log::debug!("sending data to client browser");
        let stream_id = msg.0;
        let image_data = msg.1;
        let frame = DataFrame::new(VIDEO_FRAME_TYPE, stream_id, image_data);
        let text = serde_json::to_string(&frame).unwrap();
        ctx.text(text) // this is the actual websocket object
    }
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
                        if stream_type == VIDEO_FRAME_TYPE {
                            let stream_id = frame.stream_id();
                            let data = frame.data;
                            log::debug!("received video frame stream_id: {}, datalen: {}", stream_id, data.len());
                            let stream_message = StreamMessage(stream_id, data);
                            if let Err(err) = self.server.try_send(stream_message) {
                                log::error!("[StreamSession] stream message text error {:?}", err);
                            }
                        } else if stream_type == CONN_FRAME_TYPE {
                            let stream_id = frame.stream_id();
                            log::info!("received connection frame, stream_id: {}", stream_id);
                            let add_streamer = AddStreamer(stream_id, recipient);
                            if let Err(err) = self.server.try_send(add_streamer) {
                                log::error!("[StreamSession] stream message text error {}", err);
                            }
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
            Ok(ws::Message::Continuation(_msg)) => {
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