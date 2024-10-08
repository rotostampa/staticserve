# STATIC SERVE

this simple static server is able to serve files from disk and automatically detect if they are GZIPPED

Sample usage

```
cargo run --release -- --port 9090 --folder /opt/homebrew/var/www/ --host '0.0.0.0'
```

## sample plist file

```
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>staticserve</string>
  <key>ProgramArguments</key>
  <array>
    <string>/Users/storage/Scripts/staticserve/target/release/staticserve</string>
    <string>--port</string>
    <string>9876</string>
    <string>--folder</string>
    <string>/opt/homebrew/var/www/</string>
  </array>
  <key>KeepAlive</key>
  <true/>
  <key>RunAtLoad</key>
  <true/>
    <key>UserName</key>
    <string>storage</string>

  <key>StandardOutPath</key>
  <string>/Users/storage/Scripts/staticserve/server.log</string>
  <key>StandardErrorPath</key>
  <string>/Users/storage/Scripts/staticserve/server.log</string>
</dict>
</plist>
```