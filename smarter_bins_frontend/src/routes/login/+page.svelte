<script lang="ts">
    import type { Auth } from 'firebase/auth'
    import {
      GoogleAuthProvider,
      signInWithPopup,
      signOut
    } from 'firebase/auth'
    import {
      getFirebaseContext,
      userStore,
      SignedIn,
      SignedOut
    } from 'sveltefire'
  
    // these two lines are the key to using sveltefire stores 
    // from inside the component's script tag
    const { auth } = getFirebaseContext()
    const user = userStore(auth as Auth)
  
    async function signInWithGoogle (auth: Auth) {
      const provider = new GoogleAuthProvider()
      provider.addScope('profile')
      provider.addScope('email')
      await signInWithPopup(auth, provider)
    }
  </script>
  
  <nav>
    <p>Data from component (not the store): {$user?.email}</p>
    <!-- 
      components can bind the store using let: so you can 
      leverage it directly in the markup 
    -->
    <SignedIn let:user let:auth>
      <p>Data from store: {user.email}</p>
      <button on:click={() => signOut(auth)}>
        Sign Out
      </button>
    </SignedIn>
  </nav>
  
  <SignedOut let:auth>
    <main>
      <button on:click={() => signInWithGoogle(auth)}>
        Sign In with Google
      </button>
    </main>
  </SignedOut>