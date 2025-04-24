<script>
// @ts-nocheck

  import { Router, Link, Route } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import Search from "./routes/Search.svelte";

  export let url = "";
</script>

<Router {url}>
  <nav>
    <Link to="/home">Home</Link>
    <Link to="/search">Search</Link>
  </nav>
  <div>
    <Route path="/search" component={Search} />
    <Route path="/home" component={Home} />
  </div>
</Router>

<style>

</style>
