<script>
// @ts-nocheck

  import { Router, Link, Route } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import Search from "./routes/Search.svelte";
  import { FirebaseApp } from 'sveltefire';
  import { initializeApp } from 'firebase/app';
  import { getFirestore } from 'firebase/firestore';
  import { getAuth } from 'firebase/auth';
  import Login from "./routes/auth/login/login.svelte";

    // Initialize Firebase
    const app = initializeApp(/* your firebase config */);
    const firestore = getFirestore(app);
    const auth = getAuth(app);
  export let url = "";

</script>


<FirebaseApp {auth} {firestore}>
<Router {url}>
  <main>
    <nav>
      <Link to="/">Home</Link>
      <Link to="/search">Search</Link>
      <Link to="/auth/login">Login</Link>
    </nav>

    <div class="content">
      <Route path="/search" component={Search}>
      </Route>
      <Route path="/auth/login" component={Login}>
      </Route>
      <Route path="/" component={Home}>
      </Route>
    </div>
  </main>
</Router>
</FirebaseApp>

<style>
  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem;
  }

  nav {
    padding: 1rem;
    background: #f0f0f0;
    margin-bottom: 2rem;
    border-radius: 0 0 8px 8px;
  }
  
  nav :global(a) {
    padding: 0.5rem 1rem;
    text-decoration: none;
    color: #333;
    margin-right: 1rem;
  }
  
  nav :global(a.active) {
    background: #ddd;
    border-radius: 4px;
  }

  .content {
    padding: 1rem;
  }
</style>
