<script>
  import QR from '../lib/qr.svelte';
</script>

<div class="container">
  <h1>Scan QR Code</h1>
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