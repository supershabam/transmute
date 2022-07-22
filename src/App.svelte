<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import { listen, Event } from '@tauri-apps/api/event'

	let status: string = 'awaiting file drop'

	const execute = async (wasmPath: string) => {
		status = `executing ${wasmPath}`
		try {
			type Result = {
				stdout: string
			}
			const result = await invoke<Result>('execute', {
				wasmPath
			})
			console.log(result)
			status = `result = ${result.stdout}`
		} catch (err) {
			status = `error = ${err}`
		}
	}

	listen('tauri://file-drop', (e: Event<string[]>) => {
		const wasmPath = e.payload[0]
		execute(wasmPath)
	})

	// listen('tauri://file-drop', (e: Event<string[]>) => {
	// })

	// listen("tauri://file-drop-hover", (e: Event<string[]>) => {
	// })

	// listen("tauri://file-drop-cancelled", (e: Event<string[]>) => {
	// })
</script>

<main>
	<h1>drag and drop a wasm file to execute</h1>
	<pre>{status}</pre>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>