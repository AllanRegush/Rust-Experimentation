## Testing Yew in Rust

#### Requires 
- wasm-pack

#### To build
`wasm-pack build --target web --out-name wasm --out-dir ./static`

#### To Run
- Run on any webserver you would like
- `miniserve ./static --index index.html`