# rustiohook

**High-performance global input listener (keyboard & mouse) using Rust + Node.js via NAPI-RS.**

Cross-platform native Node.js module built with [rdev](https://crates.io/crates/rdev) under the hood, suitable for barcode scanners, keyloggers (for legitimate use!), hotkey tools, and more.

---

## ✨ Features

- Fast native performance (Rust backend)
- Captures global keyboard events (mouse support optional)
- Works on **Windows**, **Linux**, and **macOS**
- Easy integration with Node.js via `start()` / `stop()`

---

## 🚀 Installation

```bash
npm install rustiohook
````

> Prebuilt `.node` files are provided for common platforms (x64, arm64).

---

## 📦 Usage

```js
const { start, stop } = require('rustiohook');

let buffer = '';

start((event) => {
  if (!event.includes('KeyPress')) return;

  if (event.includes('KeyPress(Return)')) {
    console.log(`✅ Scanned Data: [${buffer}]`);
    buffer = '';
    return;
  }

  const match = event.match(/name: Some\("([^"]+)"\)/);
  if (match && match[1]) {
    buffer += match[1];
  }
});

process.on('SIGINT', () => {
  stop();
  process.exit();
});
```

---

## 🧪 Test

```bash
npm run test
```

This will run a manual test script (`__test__/index.spec.js`) that listens for keypress events. Press `Enter` to submit.

---

## 🔧 Development

```bash
npm run build         # Build native module
npm run build:debug   # Build debug .node with symbols
npm run clean         # Remove Rust target dir
```

---

## 📄 License

MIT © 2025 [Xzdes]