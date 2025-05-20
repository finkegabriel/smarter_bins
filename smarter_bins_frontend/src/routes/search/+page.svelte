<script>
  import { onMount, onDestroy } from 'svelte';
  import init, {detect_qr_from_rgba} from '../../pkg/api';

  let video;
  let canvas;
  let scanning = false;
  let result = '';
  let wasmModule;
  
  onMount(async () => {
    try {
      // Initialize WASM
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
      console.error('Error:', err);
    }
  });

  onDestroy(() => {
    scanning = false;
    if (video && video.srcObject) {
      video.srcObject.getTracks().forEach(track => track.stop());
    }
  });
  async function scanQRCode() {
    if (!scanning) return;

    try {
      const context = canvas.getContext('2d');
      context.drawImage(video, 0, 0, canvas.width, canvas.height);
      const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
      
      // Debug log to check frame data
      console.log('Frame data:', {
        width: imageData.width,
        height: imageData.height,
        dataLength: imageData.data.length
      });
      await init();
      const detected = detect_qr_from_rgba(imageData.data, imageData.width, imageData.height);
      if (detected && detected !== "") {
        result = detected;
        console.log('QR Code detected:', result);
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
    <canvas bind:this={canvas} hidden></canvas>
    {#if result}
      <p class="result">Detected: {result}</p>
    {/if}
  </div>
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
  }

  video {
    width: 100%;
    max-width: 640px;
    border-radius: 4px;
  }

  .result {
    margin-top: 1rem;
    padding: 0.5rem;
    background: #e6f3ff;
    border-radius: 4px;
    color: #333;
  }
</style>