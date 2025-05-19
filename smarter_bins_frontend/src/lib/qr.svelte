<script lang="ts">
    import { BrowserQRCodeReader } from '@zxing/browser';
    import { onMount, onDestroy } from 'svelte';
    
    let videoElement: HTMLVideoElement;
    let currentCode: string | null = null;
    let scannerActive = true;
    let codeReader: BrowserQRCodeReader;
    let error: string | null = null;

    const constraints = {
        video: {
            facingMode: 'environment',
            width: { ideal: 1280 },
            height: { ideal: 720 }
        }
    };
  
    // onMount(async () => {
    //   try {
    //     codeReader = new BrowserQRCodeReader();
    //     const devices = await navigator.mediaDevices.enumerateDevices();
    //     const cameras = devices.filter(device => device.kind === 'videoinput');
        
    //     if (cameras.length === 0) {
    //       error = 'No cameras found';
    //       return;
    //     }

    //     const stream = await navigator.mediaDevices.getUserMedia(constraints);
    //     videoElement.srcObject = stream;
    //     await videoElement.play();
  
    //     scanLoop();
    //   } catch (err) {
    //     error = `Camera error: ${err.message}`;
    //     console.error('Error accessing camera:', err);
    //   }
    // });

    // onDestroy(() => {
    //   scannerActive = false;
    //   if (videoElement?.srcObject) {
    //     const tracks = videoElement.srcObject as MediaStream;
    //     tracks.getTracks().forEach(track => track.stop());
    //   }
    // });
  
    // async function scanLoop() {
    //   while (scannerActive) {
    //     try {
    //       const result = await codeReader.decodeOnceFromVideoElement(videoElement);
    //       const text = result.getText();
    //       currentCode = text;
    //     } catch (error) {
    //       console.log('No QR code detected, retrying...');
    //       await new Promise(resolve => setTimeout(resolve, 300));
    //     }
    //   }
    // }
</script>

<div class="scanner-container">
  {#if error}
    <div class="error">{error}</div>
  {:else}
    <video 
      bind:this={videoElement}
      autoplay 
      playsinline
      muted
    >
      <track kind="captions" src="" label="QR Scanner" default>
    </video>
  {/if}

  {#if currentCode}
    <div class="result">
      <h2>Scanned QR Code:</h2>
      <a href="https://{currentCode}" target="_blank" rel="noopener noreferrer">
        <div class="code-display">{currentCode}</div>
      </a>
    </div>
  {/if}
</div>

<style>
    .scanner-container {
        width: 100%;
        max-width: 600px;
        margin: 0 auto;
    }

    video {
        width: 100%;
        height: auto;
        border: 2px solid #000;
        border-radius: 8px;
        margin-bottom: 1rem;
        background: #000;
    }

    .error {
        color: red;
        text-align: center;
        padding: 1rem;
        background: #fee;
        border-radius: 8px;
        margin: 1rem 0;
    }

    .result {
        margin-top: 1rem;
        text-align: center;
    }
    .code-display {
        background: #f0f0f0;
        padding: 1rem;
        border-radius: 4px;
        margin: 0.5rem 0;
        transition: background-color 0.2s;
    }
    .code-display:hover {
        background: #e0e0e0;
    }
    h2 {
        color: #333;
        margin-bottom: 0.5rem;
    }
    a {
        text-decoration: none;
        color: inherit;
    }
</style>