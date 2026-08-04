# 別画面でweb inspector起動時の黒画面
対処法
```
WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri dev
```
で起動

# version変更
./src-tauri/tauri.conf.json
./src-tauri/Cargo.toml
./package.json