<!-- src/routes/+layout.svelte -->
<script>
    import { onMount } from 'svelte';
    import { initializeApp } from 'firebase/app';
    import { getAuth, onAuthStateChanged } from 'firebase/auth';
    import { getFirestore } from 'firebase/firestore';
    import { setContext } from 'svelte';
    import { writable } from 'svelte/store';
  
    const firebaseConfig = {
        apiKey: "AIzaSyC1SqzOnIhEeLRBH5NjWqluG1sL0dxeWKw",
        authDomain: "orobytes-85a8e-de3f9.firebaseapp.com",
        projectId: "orobytes-85a8e",
        storageBucket: "orobytes-85a8e.firebasestorage.app",
        messagingSenderId: "595719709173",
        appId: "1:595719709173:web:4515969ebc07c5a2f380bc",
        measurementId: "G-QHHS8BEM1M"
    };

    const user = writable(null); // <- You can access this in any child component
  
    onMount(() => {
      const app = initializeApp(firebaseConfig);
      const auth = getAuth(app);
      const firestore = getFirestore(app);
  
      // Optional: Set context to access in any component using getContext()
      setContext('firebase', { app, auth, firestore, user });
  
      // Auth listener
      onAuthStateChanged(auth, (currentUser) => {
        user.set(currentUser);
      });
    });
  </script>
  
  <slot />
  