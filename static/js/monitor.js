
const streamerId = Math.floor(Math.random() * 1000000);
const videoStreamType = 128;
const connectionStreamType = 129;

const streamer = document.getElementById('video-pane');

let mediaStream;
let socket = new WebSocket('ws://localhost:8080/ws/');
socket.binaryType = 'arraybuffer';

socket.onmessage = (event) => {
    let data = JSON.parse(event.data);
    let streamId = data.stream_id;
    let imageFrame = data.data;
    console.log("image rx: " + imageFrame.length);
    streamer.src = imageFrame;
};

socket.onopen = () => {
    console.log('WebSocket connection opened');
    // TODO - send a proper object with auth
    const connect = {
        stream_id: streamerId,
        stream_type: connectionStreamType,
        data: "connect"
    };
    socket.send(JSON.stringify(connect));
};

socket.onclose = () => { 
    // TODO - blank out the final image
    console.log('WebSocket connection closed');
};

socket.onerror = (error) => {
    console.log("socket error: " + error);
};