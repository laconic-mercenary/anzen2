if ('ImageCapture' in window) { 
    console.log('ImageCapture is supported!');
} else {
    console.error('ImageCapture is not supported!');
}

const deviceId = Math.floor(Math.random() * 1000000);
const videoStreamType = 128;
const connectionDeviceType = 130;
const videoStream = document.getElementById('video-stream');
const deviceSelect = document.getElementById('device-select');
const captureCheckbox = document.getElementById('capture-checkbox');
const deviceIdLabel = document.getElementById('device-id');
const protocol = window.location.protocol === 'https:' ? 'wss://' : 'ws://';
const sendIntervalMs = 250;
const reconnectIntervalMs = 15 * 1000;
const txType = 'arraybuffer';
const videoDeviceType = "videoinput";
const jpegQuality = 0.8;
const minResolution = { width: 640, height: 480 };
const idealResolution = { width: 800, height: 600 };
const maxResolution = { width: 1920, height: 1080 };

let mediaStream;
let socket;
let socketReconnectHandle;
let animateFrameHandle;

function connectSocket() {
    console.log('connectSocket');
    socket = new WebSocket(`${protocol}${window.location.host}/ws/`);
    socket.binaryType = txType;
    socket.onopen = () => {
        console.log('socket.onopen');
        clearInterval(socketReconnectHandle);
        const connect = {
            sender_id: deviceId,
            stream_type: connectionDeviceType,
            data: "connectDevice"
        };
        socket.send(JSON.stringify(connect));
    };
    socket.onclose = () => { 
        console.log('socket.onclose');
        clearInterval(socketReconnectHandle);
        socketReconnectHandle = setTimeout(connectSocket, reconnectIntervalMs);
    };
    socket.onerror = (error) => {
        console.log('socket.onerror');
        console.error(error);
        closeSocket();
    };
}

function startCapture() {
    console.log('startCapture');
    const cameraId = deviceSelect.value;
    
    const constraints = { 
        video: { 
            deviceId: cameraId,
            width: { min: minResolution.width, ideal: idealResolution.width, max: maxResolution.width },
            height: { min: minResolution.height, ideal: idealResolution.height, max: maxResolution.height },
        }
    };

    navigator.mediaDevices
        .getUserMedia(constraints)
        .then(stream => {
            mediaStream = stream;
            videoStream.srcObject = stream;

            const track = stream.getVideoTracks()[0];
            const imageCapture = new ImageCapture(track);

            let lastSentTime = 0;

            const sendFrame = () => {
                const now = Date.now();
                if (now - lastSentTime >= sendIntervalMs) {
                    lastSentTime = now;
                    imageCapture
                        .grabFrame()
                        .then(imageBitmap => {
                            const canvas = document.createElement('canvas');
                            canvas.width = imageBitmap.width;
                            canvas.height = imageBitmap.height;
                            const ctx = canvas.getContext('2d');
                            ctx.drawImage(imageBitmap, 0, 0);
                            canvas.toBlob(blob => {
                                if (socket) {
                                    const reader = new FileReader();
                                    reader.readAsDataURL(blob);
                                    reader.onloadend = () => {
                                        const base64data = reader.result;
                                        socket.send(JSON.stringify({
                                            stream_type: videoStreamType,
                                            sender_id: deviceId,
                                            data: base64data
                                        }));
                                    }
                                } else {
                                    console.error("socket not initialized");
                                    connectSocket();
                                }
                            }, 'image/jpeg', jpegQuality);
                        })
                        .catch(error => console.error('Error grabbing frame:', error));
                }

                animateFrameHandle = requestAnimationFrame(sendFrame);
            };
            sendFrame();
        })
        .catch(error => console.error('Error accessing camera:', error));
}

function stopCapture() {
    console.log("stopCapture");
    if (mediaStream) {
        mediaStream.getTracks().forEach(track => track.stop());
        mediaStream = null;
        videoStream.srcObject = null;
    }
    if (animateFrameHandle) {
        cancelAnimationFrame(animateFrameHandle);
        animateFrameHandle = null;
    }
}

function closeSocket() {
    console.log("closeSocket");
    if (socket) {
        socket.close();
        socket.onclose = null;
        socket.onerror = null;
        socket.onopen = null;
        socket = null;
    }
}


// Get available camera devices
navigator.mediaDevices.enumerateDevices()
    .then(devices => {
        devices.forEach(device => {
            if (device.kind === videoDeviceType) {
                const option = document.createElement('option');
                option.value = device.deviceId;
                option.text = device.label || `Camera ${deviceSelect.options.length + 1}`;
                deviceSelect.add(option);
            }
        });
    })
    .catch(error => console.error('Error enumerating devices:', error));

// Start/stop video capture
captureCheckbox.addEventListener('change', () => {
    console.log('captureCheckbox.change');
    if (captureCheckbox.checked) {
        startCapture();
    } else {
        stopCapture();
    }
});
  
window.onoffline = (event) => {
    console.log("network lost");
};

window.ononline = (event) => {
    console.log("network available");
};

window.onload = () => {
    console.log('window.onload');
    deviceIdLabel.textContent = deviceId;
    connectSocket();
};