# transmute
drag and drop wasm and file inputs to produce outputs

## problem
I want to explore more with the capabilities of WASM and WASI especially around safe computing and cross-platform single binaries.  
I also want to explore more with svelte and tauri.  
So, as you can see there's no real problem.  

## approach
Make a tauri UI where you can drag and drop a wasm file onto it, and then drop another file onto the application to seed the working directory for the wasm application.

## issues

* drag and drop to filesystem not implemented in tauri https://github.com/tauri-apps/tauri/issues/2593  
