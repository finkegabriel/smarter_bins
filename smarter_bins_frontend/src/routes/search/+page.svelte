<script>
  import QR from '../../lib/qr.svelte';
  import { onMount } from 'svelte';
  import * as wasm from '../../pkg/api.js';

  wasm.default().then(() => {
    console.log('WASM module loaded');
    wasm.greet();
  }).catch(err => {
    console.error('Error loading WASM module:', err);
  });
</script>

<div class="container">
  <div class="scanner-wrapper">
    <QR />
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
    background: #ffffff;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  h1 {
    color: #333;
    margin-bottom: 2rem;
  }
</style>