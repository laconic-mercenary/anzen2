
const streamerId = Math.floor(Math.random() * 1000000);
const videoStreamType = 128;
const connectionStreamType = 129;
const streamer = document.getElementById('video-pane');
const protocol = window.location.protocol === 'https:' ? 'wss://' : 'ws://';
const reconnectIntervalMs = 30 * 1000;

let mediaStream;
let socket;
let connectSocketInterval;

function closeSocket() {
    console.log('closeSocket');
    if (socket) {
        socket.close();
        socket.onmessage = null;
        socket.onopen = null;
        socket.onclose = null;
        socket.onerror = null;
        socket = null;
        streamer.src = null;
    }
}

function socketOnMessage(event) {
    console.log('socketOnMessage');
    let data = JSON.parse(event.data);
    let streamId = data.stream_id;
    let imageFrame = data.data;
    console.log("image rx: " + imageFrame.length);
    streamer.src = imageFrame;
}

function socketOnOpen() {
    console.log("socketOnOpen");
    clearInterval(connectSocketInterval);
    const connect = {
        stream_id: streamerId,
        stream_type: connectionStreamType,
        data: "connect"
    };
    socket.send(JSON.stringify(connect));
}

function socketOnClose() {
    console.log('socketOnClose');
    // auto reconnect
    connectSocketInterval = setTimeout(connectSocket, reconnectIntervalMs);
}

function socketOnError(err) {
    console.log('socketOnError');
    console.error(err);
    closeSoctket();
}

function connectSocket() {
    console.log('connectSocket');
    socket = new WebSocket(`${protocol}${window.location.host}/ws/`);
    socket.binaryType = 'arraybuffer';
    socket.onmessage = socketOnMessage;
    socket.onopen = socketOnOpen;
    socket.onclose = socketOnClose;
    socket.onerror = socketOnError;
}

window.onload = () => {
    console.log('window.onload');
    connectSocket();
};