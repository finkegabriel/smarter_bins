wasm-pack --target web build --release || wasm-pack build --target web --out-dir ../smarter_bins_frontend/src/pkg

need to supply your own firebase config file in src/lib/firebase.js