# simple-spa-http-server

Rust app / docker image that serves files from a directory for single page apps (redirects missing to -> index.html)

## Config env defaults
```bash
BIND=0.0.0.0:8080
CACHE_CONTROL=public, max-age=86400
SERVE=./www
```

## BUILD
```bash
cargo build --release
```

```bash
docker pull hortopan/simple-spa-http-server
```
