<script>
  import { onMount, onDestroy } from 'svelte';
  import init, {detect_qr_from_rgba} from '../../pkg/api';

  let video;
  let canvas;
  let scanning = false;
  let result = '';
  let wasmInitialized = false;
  
  onMount(async () => {
    try {
      // Initialize WASM first
      await init();
      wasmInitialized = true;
      console.log('WASM module loaded');

      // Get camera access
      const stream = await navigator.mediaDevices.getUserMedia({ 
        video: { 
          facingMode: "environment",
          width: { ideal: 1280 },
          height: { ideal: 720 }
        } 
      });
      
      // Use the bound video element
      video.srcObject = stream;
      await video.play();

      // Setup canvas with video dimensions
      canvas.width = video.videoWidth;
      canvas.height = video.videoHeight;
      
      // Start scanning
      scanning = true;
      scanQRCode();
    } catch (err) {
      console.error('Initialization error:', err);
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
      if (detected && detected.content !== "") {
        result = detected.content;
        console.log('QR Code detected:', result);
        
        // Draw bounding box
        context.strokeStyle = '#00FF00';
        context.lineWidth = 4;
        context.beginPath();
        const bounds = detected.bounds;
        
        if (bounds && bounds.length === 4) {
          context.moveTo(bounds[0][0], bounds[0][1]);
          bounds.slice(1).forEach(point => {
            context.lineTo(point[0], point[1]);
          });
          context.lineTo(bounds[0][0], bounds[0][1]); // Close the path
          context.stroke();
        }
      }
    } catch (err) {
      console.error('QR Detection error:', err);
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
    <canvas bind:this={canvas}></canvas> <!-- Removed hidden attribute -->
  </div>
  {#if result}
    <div class="result">
      <a href="https://{result}" target="_blank" rel="noopener noreferrer">
        <div class="code-display">{result}</div>
      </a>
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

  .result {
    margin-top: 1rem;
    padding: 0.5rem;
    background: #e6f3ff;
    border-radius: 4px;
    color: #333;
  }
</style>