# transmute
drag and drop wasm and file inputs to produce outputs

## problem
I want to explore more with the capabilities of WASM and WASI especially around safe computing and cross-platform single binaries.  
I also want to explore more with svelte and tauri.  
So, as you can see there's no real problem.  

## approach
Make a tauri UI where you can drag and drop a wasm file onto it, and then drop another file onto the application to seed the working directory for the wasm application.

## technical roadmap
[ ] embedded WASI program executes and prints stdout and stderr to program UI.  
[ ] drag and drop WASM file onto screen to load and execute it.  
[ ] drag and drop WASM file onto screen to load and present file upload interface for additional file inputs.  
[ ] display resulting file outputs from working directory.  
[ ] open directory in finder button.  

## issues

* drag and drop to filesystem not implemented in tauri https://github.com/tauri-apps/tauri/issues/2593  
