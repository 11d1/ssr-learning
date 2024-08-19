#### how to run

1. Build Hydration Bundle
```
trunk build index.html
```

2. Run the server
```
cargo run --features=ssr --bin ssr_router_server -- --dir dist
```