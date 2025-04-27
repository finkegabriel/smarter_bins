<script lang="ts">
    import { BrowserQRCodeReader } from '@zxing/browser';
    import { onMount } from 'svelte';
  
    let videoElement: HTMLVideoElement;
    let currentCode: string | null = null;
    let scannerActive = true;
    let codeReader: BrowserQRCodeReader;
  
    onMount(async () => {
      codeReader = new BrowserQRCodeReader();
  
      try {
        const stream = await navigator.mediaDevices.getUserMedia({ video: { facingMode: 'environment' } });
        videoElement.srcObject = stream;
        await videoElement.play();
  
        scanLoop();
      } catch (err) {
        console.error('Error accessing camera:', err);
      }
    });
  
    async function scanLoop() {
      while (scannerActive) {
        try {
          const result = await codeReader.decodeOnceFromVideoElement(videoElement);
          const text = result.getText();
          currentCode = text;
        } catch (error) {
          console.log('No QR code detected, retrying...');
          await new Promise(resolve => setTimeout(resolve, 300));
        }
      }
    }
</script>
  
<video bind:this={videoElement} autoplay playsinline style="width: 100%; height: auto;"></video>
  
{#if currentCode}
  <div class="result">
    <h2>Scanned QR Code:</h2>
    <a href="https://{currentCode}" target="_blank" rel="noopener noreferrer">
      <div class="code-display">{currentCode}</div>
    </a>
  </div>
{/if}

<style>
    video {
        border: 2px solid #000;
        border-radius: 8px;
        margin-bottom: 1rem;
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