<!-- src/routes/+page.svelte -->
<script>
    import { auth } from '$lib/firebase';
    import { GoogleAuthProvider, signInWithPopup, signOut, onAuthStateChanged } from 'firebase/auth';
  
    let user = null;
  
    // Svelte 5: use `onMount` instead of reactive `$:` blocks
    import { onMount } from 'svelte';
  
    onMount(() => {
      return onAuthStateChanged(auth, (firebaseUser) => {
        user = firebaseUser;
      });
    });
  
    function login() {
      const provider = new GoogleAuthProvider();
      signInWithPopup(auth, provider).catch(console.error);
    }
  
    function logout() {
      signOut(auth);
    }
  </script>
  
  {#if user}
    <p>Welcome, {user.displayName}!</p>
    <button on:click={logout}>Logout</button>
  {:else}
    <button on:click={login}>Login with Google</button>
  {/if}
  