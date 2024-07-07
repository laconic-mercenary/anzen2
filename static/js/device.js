if ('ImageCapture' in window) { 
    console.log('ImageCapture is supported!');
} else {
    console.log('ImageCapture is not supported!');
}

const streamerId = Math.floor(Math.random() * 1000000);
const videoStreamType = 128;
const connectionStreamType = 129;

const videoStream = document.getElementById('video-stream');
const deviceSelect = document.getElementById('device-select');
const captureCheckbox = document.getElementById('capture-checkbox');
const streamer = document.getElementById('video-pane');

let mediaStream;
let socket = new WebSocket(`ws://${window.location.host}/ws/`);
socket.binaryType = 'arraybuffer';
socket.onopen = () => {
    console.log('WebSocket connection opened');
};
socket.onclose = () => { 
    console.log('WebSocket connection closed');
}

// Get available camera devices
navigator.mediaDevices.enumerateDevices()
    .then(devices => {
        devices.forEach(device => {
            if (device.kind === 'videoinput') {
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
    if (captureCheckbox.checked) {
        startCapture();
    } else {
        stopCapture();
    }
});

function startCapture() {
    const deviceId = deviceSelect.value;
    const constraints = { video: { deviceId: deviceId } };

    navigator.mediaDevices
        .getUserMedia(constraints)
        .then(stream => {
            mediaStream = stream;
            videoStream.srcObject = stream;

            const track = stream.getVideoTracks()[0];
            const imageCapture = new ImageCapture(track);

            // throttle to 250ms
            let lastSentTime = 0;
            const sendDelay = 250;

            const sendFrame = () => {
                const now = Date.now();
                if (now - lastSentTime >= sendDelay) {
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
                                            stream_id: streamerId,
                                            data: base64data
                                        }));
                                    }
                                } else {
                                    console.log("socket not initialized")
                                }
                            }, 'image/jpeg', 0.8);
                        })
                        .catch(error => console.error('Error grabbing frame:', error));
                }

                requestAnimationFrame(sendFrame);
            };
            sendFrame();
        })
        .catch(error => console.error('Error accessing camera:', error));
}

function stopCapture() {
    if (mediaStream) {
        mediaStream.getTracks().forEach(track => track.stop());
        mediaStream = null;
        videoStream.srcObject = null;
    }
}
