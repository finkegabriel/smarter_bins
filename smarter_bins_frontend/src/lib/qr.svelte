<script lang="ts">
    import { BrowserQRCodeReader } from '@zxing/browser';
    import { onMount } from 'svelte';
  
    let videoElement: HTMLVideoElement;
    let codes: string[] = [];
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
          if (!codes.includes(text)) {
            codes = [...codes, text];
          }
        } catch (error) {
          console.log('No QR code detected, retrying...');
          // Wait a tiny bit before retrying
          await new Promise(resolve => setTimeout(resolve, 300));
        }
      }
    }
  </script>
  
  <!-- svelte-ignore a11y_media_has_caption -->
  <video bind:this={videoElement} autoplay playsinline style="width: 100%; height: auto;"></video>
  
  <h2>Detected QR Codes:</h2>
  <ul>
    {#each codes as code}
        {console.log(code)}
      <li>{code}</li>
    {/each}
  </ul>
    <button on:click={() => scannerActive = !scannerActive}>
        {scannerActive ? 'Stop Scanner' : 'Start Scanner'}
    </button>
    <style>
        video {
            border: 2px solid #000;
            border-radius: 8px;
            margin-bottom: 1rem;
        }
        h2 {
            color: #333;
        }
        ul {
            list-style-type: none;
            padding: 0;
        }
        li {
            background: #f0f0f0;
            margin: 0.5rem 0;
            padding: 0.5rem;
            border-radius: 4px;
        }
        button {
            padding: 0.5rem 1rem;
            background: #007bff;
            color: #fff;
            border: none;
            border-radius: 4px;
            cursor: pointer;
        }
        button:hover {
            background: #0056b3;
        }
        button:disabled {
            background: #ccc;
            cursor: not-allowed;
        }
        button:disabled:hover {
            background: #ccc;
        }
    </style>