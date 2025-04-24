<script>
  import QR from './lib/qr.svelte'
  import { Router, Link, Route } from "svelte-routing";
  import Home from "./routes/Home.svelte";

  export let url = "";
</script>

<Router {url}>
  <nav>
    <Link to="/">Home</Link>
    <Link to="/search">Search</Link>
  </nav>
  <div>
    <!-- <Route path="/blog/:id" component={BlogPost} /> -->
    <!-- <Route path="/blog" component={Blog} /> -->
    <!-- <Route path="/about" component={About} /> -->
    <Route path="/"><Home /></Route>
  </div>
</Router>

<main>
  <div class="card">
    <QR />
  </div>
</main>

<style>

</style>
