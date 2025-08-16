class CameraMatrix {
    constructor() {
        this.cameras = new Map();
        this.cameraGrid = document.getElementById('cameraGrid');
        this.addCameraForm = document.getElementById('addCameraForm');
        this.overlay = document.getElementById('overlay');
        this.initializeEventListeners();
    }

    initializeEventListeners() {
        this.overlay.addEventListener('click', () => {
            this.hideAddCameraForm();
        });

        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                this.hideAddCameraForm();
            }
        });
    }

    showAddCameraForm() {
        this.addCameraForm.classList.add('show');
        this.overlay.classList.add('show');
        document.getElementById('cameraName').focus();
    }

    hideAddCameraForm() {
        this.addCameraForm.classList.remove('show');
        this.overlay.classList.remove('show');
        this.clearForm();
    }

    clearForm() {
        document.getElementById('cameraName').value = '';
        document.getElementById('streamUrl').value = '';
    }

    addCamera() {
        const name = document.getElementById('cameraName').value.trim();
        const url = document.getElementById('streamUrl').value.trim();

        if (!name || !url) {
            alert('Please fill in both camera name and stream URL');
            return;
        }

        if (this.cameras.has(name)) {
            alert('A camera with this name already exists');
            return;
        }

        this.createCameraFeed(name, url);
        this.hideAddCameraForm();
    }

    createCameraFeed(name, url) {
        const cameraId = `camera-${Date.now()}`;
        
        const cameraElement = document.createElement('div');
        cameraElement.className = 'camera-feed';
        cameraElement.id = cameraId;

        if (this.isHLSStream(url)) {
            cameraElement.innerHTML = `
                <video autoplay muted playsinline>
                    <source src="${url}" type="application/x-mpegURL">
                    Your browser does not support HLS video.
                </video>
                <div class="camera-overlay">
                    <span class="camera-label">${name}</span>
                    <button class="camera-close" onclick="cameraMatrix.removeCamera('${name}')" title="Remove camera">×</button>
                </div>
            `;
        } else {
            cameraElement.innerHTML = `
                <video autoplay muted playsinline>
                    <source src="${url}">
                    Your browser does not support this video format.
                </video>
                <div class="camera-overlay">
                    <span class="camera-label">${name}</span>
                    <button class="camera-close" onclick="cameraMatrix.removeCamera('${name}')" title="Remove camera">×</button>
                </div>
            `;
        }

        this.cameras.set(name, {
            id: cameraId,
            url: url,
            element: cameraElement
        });

        this.updateGrid();
        this.setupVideoElement(cameraElement, url);
    }

    setupVideoElement(cameraElement, url) {
        const video = cameraElement.querySelector('video');
        
        if (this.isHLSStream(url) && this.isHLSSupported()) {
            if (typeof Hls !== 'undefined') {
                const hls = new Hls();
                hls.loadSource(url);
                hls.attachMedia(video);
                hls.on(Hls.Events.MANIFEST_PARSED, () => {
                    video.play().catch(console.error);
                });
            } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
                video.src = url;
                video.addEventListener('loadedmetadata', () => {
                    video.play().catch(console.error);
                });
            }
        } else {
            video.addEventListener('loadedmetadata', () => {
                video.play().catch(console.error);
            });
        }

        video.addEventListener('error', (e) => {
            console.error('Video error:', e);
            alert(`Failed to load video stream: ${url}\nError: ${e.target.error?.message || 'Unknown error'}`);
            this.showVideoError(cameraElement);
        });
    }

    isHLSStream(url) {
        return url.includes('.m3u8') || url.includes('application/x-mpegURL');
    }

    isHLSSupported() {
        const video = document.createElement('video');
        return video.canPlayType('application/vnd.apple.mpegurl') !== '' || typeof Hls !== 'undefined';
    }

    showVideoError(cameraElement) {
        const video = cameraElement.querySelector('video');
        video.style.display = 'none';
        
        const errorDiv = document.createElement('div');
        errorDiv.style.cssText = `
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100%;
            background: #333;
            color: #999;
            font-size: 14px;
        `;
        errorDiv.textContent = 'Stream unavailable';
        
        cameraElement.insertBefore(errorDiv, cameraElement.querySelector('.camera-overlay'));
    }

    removeCamera(name) {
        const camera = this.cameras.get(name);
        if (camera) {
            camera.element.remove();
            this.cameras.delete(name);
            this.updateGrid();
        }
    }

    updateGrid() {
        const emptyState = this.cameraGrid.querySelector('.empty-state');
        
        if (this.cameras.size === 0) {
            if (!emptyState) {
                const emptyDiv = document.createElement('div');
                emptyDiv.className = 'empty-state';
                emptyDiv.innerHTML = '<p>No camera feeds added yet. Click "Add Camera" to get started.</p>';
                this.cameraGrid.appendChild(emptyDiv);
            }
        } else {
            if (emptyState) {
                emptyState.remove();
            }
            
            this.cameras.forEach(camera => {
                if (!this.cameraGrid.contains(camera.element)) {
                    this.cameraGrid.appendChild(camera.element);
                }
            });
        }
    }
}

const cameraMatrix = new CameraMatrix();

function showAddCameraForm() {
    cameraMatrix.showAddCameraForm();
}

function hideAddCameraForm() {
    cameraMatrix.hideAddCameraForm();
}

function addCamera() {
    cameraMatrix.addCamera();
}