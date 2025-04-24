<script>
  import { onMount, onDestroy } from 'svelte';
  import { BrowserMultiFormatReader } from '@zxing/browser';

  let videoElement;
  let results = [];
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
      const devices = await navigator.mediaDevices.enumerateDevices();
      const frontCamera = devices.find(device =>
        device.kind === 'videoinput' && device.label.toLowerCase().includes('front')
      );


      const stream = await navigator.mediaDevices.getUserMedia(constraints);
      videoElement.srcObject = stream;
      await videoElement.play();

      reader.decodeFromVideoElement(videoElement, (result, err) => {
        if (result && !results.find(r => r.text === result.getText())) {
          results = [...results, { text: result.getText(), format: result.getBarcodeFormat() }];
        }
        if (err) {
          if (err.name !== 'NotFoundException') {
            console.error(err);
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

<h3>Scanned QR Codes:</h3>
<ul>
  {#each results as result}
    <li>{result.text} ({result.format})</li>
  {/each}
</ul>
