<div align="center">

  <h1><code>watcher</code></h1>

<strong>WiFi and BLE snooper service based on captured Kismet data</strong>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

`watcher` is a web service that ingests your captured Kismetdb data locally and lets you query
the data.

## Features

- [ ] Merge multiple Kismetdb captures into one.
  - Take in account of unique device IDs (should merge data, keep old/new, etc.)
- [ ] Map out captured WiFi AP's and stations, and BLE devices (requires GPS).
- [ ] Allow querying for devices, including providing pre-made queries.
  - "What devices have I passed the most?"
  - "Where have I seen X device?"
  - "At what point did I see the most devices?"
  - Filter for ESSIDs, BSSIDs, OUIs, etc.
- [ ] (?) Load an LLM in-browser allowing users to query the data through AI.
  - https://www.adlrocha.com/blog/2024-02-18-wasm-llm/
  - https://github.com/huggingface/candle

## License

Licensed under:

* MIT License, ([LICENSE](LICENSE) or https://opensource.org/license/mit)
