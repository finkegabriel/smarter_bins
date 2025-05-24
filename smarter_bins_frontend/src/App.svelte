<script>
// @ts-nocheck

  import { Router, Link, Route } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import Search from "./routes/Search.svelte";
  // import { FirebaseApp } from 'sveltefire';
  // import { initializeApp } from 'firebase/app';
  // import { getFirestore } from 'firebase/firestore';
  // import { getAuth } from 'firebase/auth';
  // import Login from "./routes/login.svelte";
  import DyanmicParts from './routes/[id]/+page.svelte';
  import { FirebaseApp } from 'sveltefire';
  import { initializeApp } from 'firebase/app';
  import { getFirestore } from 'firebase/firestore';
  import { getAuth } from 'firebase/auth';
  import Search from "./routes/Search.svelte";
  import Login from "./routes/login/+page.svelte";
  const firebaseConfig = {
  apiKey: "your-api-key",
  authDomain: "your-auth-domain",
  projectId: "your-project-id",
  storageBucket: "your-storage-bucket",
  messagingSenderId: "your-messaging-sender-id",
  appId: "your-app-id"
};

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
      <Link to="/:id">Dynamic Parts</Link>
    </nav>

    <div class="content">
      <Route path="/" component={Home}>
      </Route>
      <Route path="/:id" component={DyanmicParts}>
      </Route>
      <Route path="/search" component={Search}>
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
