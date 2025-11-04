<script>
  import { onMount, onDestroy } from 'svelte';
  import init, {detect_qr_from_rgba} from '../../pkg/api.js';

  let video;
  let canvas;
  let scanning = false;
  let results = []; // Array to store all detected QR codes
  let wasmInitialized = false;
  
  onMount(async () => {
    try {
      // Initialize WASM first
      await init();
      wasmInitialized = true;
      console.log('WASM module loaded');

      // Get camera access - try with flexible constraints first (for external webcams)
      let stream;
      try {
        // First try: prefer rear camera on mobile, but flexible for external webcams
        stream = await navigator.mediaDevices.getUserMedia({ 
          video: { 
            facingMode: { ideal: "environment" }, // ideal instead of required
            width: { ideal: 1280 },
            height: { ideal: 720 }
          } 
        });
      } catch (err) {
        // Fallback 1: try without facingMode constraint (for external webcams)
        console.log('Fallback: trying without facingMode constraint');
        try {
          stream = await navigator.mediaDevices.getUserMedia({ 
            video: { 
              width: { ideal: 1280 },
              height: { ideal: 720 }
            } 
          });
        } catch (err2) {
          // Fallback 2: try with minimal constraints (for problematic external webcams)
          console.log('Fallback 2: trying with minimal constraints');
          stream = await navigator.mediaDevices.getUserMedia({ 
            video: true 
          });
        }
        console.log('Camera access successful with fallback constraints');
      }
      
      // Use the bound video element
      video.srcObject = stream;
      
      // Wait for video metadata to load before playing
      await new Promise((resolve, reject) => {
        let timeoutId;
        let resolved = false;
        
        const cleanup = () => {
          if (timeoutId) clearTimeout(timeoutId);
          video.onloadedmetadata = null;
          video.onerror = null;
        };
        
        const success = () => {
          if (resolved) return;
          resolved = true;
          cleanup();
          // Setup canvas with video dimensions after metadata is loaded
          canvas.width = video.videoWidth;
          canvas.height = video.videoHeight;
          resolve();
        };
        
        const fail = (error) => {
          if (resolved) return;
          resolved = true;
          cleanup();
          reject(error);
        };
        
        // Check if metadata is already loaded
        if (video.readyState >= 1) { // HAVE_METADATA
          video.play()
            .then(() => {
              // Wait a bit for dimensions to be available
              if (video.videoWidth > 0 && video.videoHeight > 0) {
                success();
              } else {
                // Wait for loadedmetadata event
                video.onloadedmetadata = () => {
                  video.play()
                    .then(success)
                    .catch(fail);
                };
              }
            })
            .catch(fail);
        } else {
          video.onloadedmetadata = () => {
            video.play()
              .then(success)
              .catch(fail);
          };
        }
        
        video.onerror = () => fail(new Error('Video element error'));
        
        // Fallback timeout - increased for external webcams
        timeoutId = setTimeout(() => {
          if (video.videoWidth === 0 || video.videoHeight === 0) {
            fail(new Error('Video metadata timeout - camera may need more time to initialize'));
          }
        }, 10000);
      });
      
      // Start scanning
      scanning = true;
      scanQRCode();
    } catch (err) {
      console.error('Initialization error:', err);
      results = [{ content: `Error: ${err.message || 'Failed to initialize camera'}`, bounds: [] }];
    }
  });

  onDestroy(() => {
    scanning = false;
    if (video && video.srcObject) {
      video.srcObject.getTracks().forEach(track => track.stop());
    }
  });
  async function scanQRCode() {
    if (!scanning || !wasmInitialized) return;  // Changed from !init to !wasmInitialized

    try {
      const context = canvas.getContext('2d');
      context.clearRect(0, 0, canvas.width, canvas.height); // Clear previous frame
      context.drawImage(video, 0, 0, canvas.width, canvas.height);
      const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
      
      const detected = detect_qr_from_rgba(imageData.data, imageData.width, imageData.height);
      
      // Handle array of QR codes
      if (detected && Array.isArray(detected) && detected.length > 0) {
        results = detected;
        console.log('QR Codes detected:', detected.length);
        
        // Draw bounding boxes for all detected QR codes
        detected.forEach((qr, index) => {
          if (qr && qr.content && qr.content !== "" && qr.bounds && qr.bounds.length === 4) {
            // Use different colors for different QR codes
            const colors = ['#00FF00', '#FF0000', '#0000FF', '#FFFF00', '#FF00FF', '#00FFFF'];
            const color = colors[index % colors.length];
            
            context.strokeStyle = color;
            context.lineWidth = 4;
            context.beginPath();
            
            const bounds = qr.bounds;
            context.moveTo(bounds[0][0], bounds[0][1]);
            bounds.slice(1).forEach(point => {
              context.lineTo(point[0], point[1]);
            });
            context.lineTo(bounds[0][0], bounds[0][1]); // Close the path
            context.stroke();
            
            // Draw QR code content near the bounding box
            context.fillStyle = color;
            context.font = '16px Arial';
            context.fillText(qr.content, bounds[0][0], bounds[0][1] - 10);
          }
        });
      } else {
        // No QR codes detected
        results = [];
      }
    } catch (err) {
      console.error('QR Detection error:', err);
      results = [];
    }

    requestAnimationFrame(scanQRCode);
  }
</script>

<div class="container">
  <div class="scanner-wrapper">
    <video 
      bind:this={video} 
      autoplay 
      playsinline 
      muted
    ></video>
    <canvas bind:this={canvas}></canvas>
  </div>
  {#if results && results.length > 0}
    <div class="results">
      <h3>Detected QR Codes ({results.length}):</h3>
      {#each results as qr}
        {#if qr && qr.content && qr.content !== ""}
          <div class="result">
            <a href="https://{qr.content}" target="_blank" rel="noopener noreferrer">
              <div class="code-display">{qr.content}</div>
            </a>
          </div>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  .container {
    padding: 1rem;
    max-width: 800px;
    margin: 0 auto;
    text-align: center;
  }

  .scanner-wrapper {
    margin: 2rem auto;
    padding: 1rem;
    background: #333;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    position: relative;
    width: 100%;
    max-width: 640px;
  }

  video {
    width: 100%;
    border-radius: 4px;
  }

  canvas {
    position: absolute;
    top: 1rem;  /* Match padding of scanner-wrapper */
    left: 1rem;
    width: calc(100% - 2rem);  /* Account for padding */
    height: calc(100% - 2rem);
    pointer-events: none;  /* Allow clicking through to video */
  }

  .results {
    margin-top: 1rem;
    padding: 1rem;
    background: #f5f5f5;
    border-radius: 8px;
  }

  .results h3 {
    margin: 0 0 0.5rem 0;
    color: #333;
    font-size: 1.1rem;
  }

  .result {
    margin-top: 0.5rem;
    padding: 0.5rem;
    background: #e6f3ff;
    border-radius: 4px;
    color: #333;
  }

  .result:first-child {
    margin-top: 0;
  }

  .code-display {
    font-family: monospace;
    word-break: break-all;
  }
</style>