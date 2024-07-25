if ('ImageCapture' in window) { 
    console.log('ImageCapture is supported!');
} else {
    console.error('ImageCapture is not supported!');
}

const deviceId = Math.floor(Math.random() * 9000000) + 1000000;
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
const idealResolution = { width: 640, height: 480 };
const maxResolution = { width: 1900, height: 1080 };

let mediaStream;
let socket;
let socketReconnectHandle;
let animateFrameHandle;

function isPNG(imageByteArray) {
    // Check if the array has at least 8 bytes (PNG signature length)
    if (imageByteArray.length < 8) {
        return false
    }
    
    // PNG files start with the following 8 bytes:
    // 137 80 78 71 13 10 26 10
    const pngSignature = [137, 80, 78, 71, 13, 10, 26, 10]
    
    for (let i = 0; i < 8; i++) {
        if (imageByteArray[i] !== pngSignature[i]) {
            return false
        }
    }
    
    return true
}


function isJPEG(imageByteArray) {
    // Check if the first two bytes are the JPEG Start of Image (SOI) marker
    if (imageByteArray.length < 2) {
        return false
    }
    
    // JPEG files start with the bytes 0xFF 0xD8
    return imageByteArray[0] === 0xFF && imageByteArray[1] === 0xD8
}


function findAppSection(imageByteArray) {
    const APP0Marker = 0xFFE0
    const APP15Marker = 0xFFEF
    let appSectionStart = -1
    let appSectionLength = 0

    for (let i = 0; i < imageByteArray.length - 1; i++) {
        if (imageByteArray[i] === 0xFF && 
            imageByteArray[i + 1] >= APP0Marker && 
            imageByteArray[i + 1] <= APP15Marker) {
            appSectionStart = i
            // The length of the APP segment is stored in the next two bytes
            appSectionLength = (imageByteArray[i + 2] << 8) | imageByteArray[i + 3]
            
            // Extract the APP section contents
            const appSectionContents = imageByteArray.slice(appSectionStart, appSectionStart + appSectionLength + 2)
            
            return {
                start: appSectionStart,
                length: appSectionLength,
                contents: appSectionContents
            }
        }
    }

    return null; // No APP section found
}

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
                            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
                            const buffer = imageData.data.buffer;
                            const byteArray = new Uint8Array(buffer);
                            const deviceIdArray = Array.from(String(deviceId), Number)
                            
                            const combinedArray = new Uint8Array(byteArray.length + deviceIdArray.length);
                            combinedArray.set(byteArray);
                            combinedArray.set(deviceIdArray, byteArray.length);

                            if (socket) {
                                console.log("sending frame " + combinedArray.length);
                                socket.send(combinedArray.buffer); // Send the modified ArrayBuffer through the WebSocket
                            } else {
                                console.error("socket not initialized");
                                connectSocket();
                            }
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