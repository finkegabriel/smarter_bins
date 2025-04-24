<script>
  import { onMount, onDestroy } from 'svelte';
  import { BrowserMultiFormatReader } from '@zxing/browser';

  let videoElement;
  let currentResult = null;  // Changed from results array to single result
  let error = '';
  let reader;

  const constraints = {
    video: {
      facingMode: { ideal: 'environment' }
    }
  };

  onMount(async () => {
    try {
      reader = new BrowserMultiFormatReader();
      const stream = await navigator.mediaDevices.getUserMedia(constraints);
      videoElement.srcObject = stream;
      await videoElement.play();

      reader.decodeFromVideoElement(videoElement, (result, err) => {
        if (result) {
          currentResult = { 
            text: result.getText(), 
            format: result.getBarcodeFormat() 
          };
        }
        
        if (err) {
          if (err.name !== 'NotFoundException') {
            return;
          }
        }
      });
    } catch (e) {
      error = 'Could not access camera: ' + e.message;
    }
  });

  onDestroy(() => {
    reader?.reset();
    videoElement?.srcObject?.getTracks()?.forEach(track => track.stop());
  });
</script>

<video bind:this={videoElement} width="300" height="200" autoplay muted playsinline></video>

{#if error}
  <p style="color: red">{error}</p>
{/if}

{#if currentResult}
  <div>
    <a href="https://{currentResult.text}" target="_blank" rel="noopener noreferrer">
      {currentResult.text} ({currentResult.format})
    </a>
  </div>
{/if}
