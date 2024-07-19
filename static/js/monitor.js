
const streamerId = Math.floor(Math.random() * 1000000);
const connectionStreamType = 129;
const protocol = window.location.protocol === 'https:' ? 'wss://' : 'ws://';
const reconnectIntervalMs = 30 * 1000;
const senderIdRegex = /^[\w-]{1,50}$/;
const imgTagPrefix = "anzen-video";
const imgTagClass = "anzen-video-img";

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
    }
}

function socketOnMessage(event) {
    console.log('socketOnMessage');
    let data = JSON.parse(event.data);
    let senderId = data.sender_id;
    let imageFrame = data.data;
    if (!isValidSenderId(senderId)) {
        console.error("invalid sender id: " + senderId);
        return;
    }
    let img = getTargetImgTag(senderId);
    if (img) {
        img.src = imageFrame;
    }
}

function socketOnOpen() {
    console.log("socketOnOpen");
    clearInterval(connectSocketInterval);
    const connect = {
        sender_id: streamerId,
        stream_type: connectionStreamType,
        data: "connectStream"
    };
    socket.send(JSON.stringify(connect));
}

function isValidSenderId(senderId) {
    return senderIdRegex.test(senderId);
}

function getTargetImgTag(senderId) {
    let imgTags = document.querySelectorAll(`img.${imgTagClass}`);
    if (imgTags.length > 10) {
        console.error("reached limit of number of video streams");
        return null;
    }   
    let id = imgTagPrefix + "-" + senderId;
    let img = document.getElementById(id);
    if (!img) {
        br = document.createElement("br");
        img = document.createElement("img");
        img.id = id;
        img.classList.add(imgTagClass);
        img.style.width = "75%";
        img.style.height = "75%";
        img.style.objectFit = "contain";
        document.body.appendChild(br);
        document.body.appendChild(img);
    } 
    return img
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

window.onoffline = (event) => {
    console.log("network lost");
};

window.ononline = (event) => {
    console.log("network available");
};

window.onload = () => {
    console.log('window.onload');
    connectSocket();
};