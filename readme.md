# STATIC SERVE

this simple static server is able to serve files from disk and automatically detect if they are GZIPPED

Sample usage

```
cargo run --release -- --port 9090 --folder /opt/homebrew/var/www/ --host '0.0.0.0'
```