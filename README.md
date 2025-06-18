# rustiohook

[![npm version](https://img.shields.io/npm/v/rustiohook)](https://www.npmjs.com/package/rustiohook)
[![CI](https://github.com/Xzdes/rustiohook/actions/workflows/ci.yml/badge.svg)](https://github.com/Xzdes/rustiohook/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> High-performance global input listener (keyboard & mouse) using Rust and Node.js via NAPI-RS.

**`rustiohook`** — это нативный модуль Node.js, который позволяет перехватывать глобальные события клавиатуры и мыши во всех окнах и приложениях.  
Полезен для сканеров штрих-кодов, хоткеев, глобального логгирования и автоматизации.

---

## 📦 Установка

```bash
npm install rustiohook
````

Или через `yarn`:

```bash
yarn add rustiohook
```

> Модуль содержит предварительно собранные `.node`-бинарники для основных платформ:
>
> * `win32-x64-msvc`
> * `linux-x64-gnu`
> * `darwin-x64`, `darwin-arm64`

Если ваша платформа не поддерживается, можно пересобрать вручную через `napi`.

---

## 🚀 Быстрый старт

```js
const { start, stop } = require('rustiohook');

let buffer = '';

start((event) => {
  if (!event.includes('KeyPress')) return;

  if (event.includes('KeyPress(Return)')) {
    console.log(`✅ Scanned: ${buffer}`);
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

## 🔌 Использование в проектах

### 1. Добавление в ваш проект

```bash
npm install rustiohook
```

### 2. Импорт и запуск

```js
const { start, stop } = require('rustiohook');

start((event) => {
  console.log('Got event:', event);
});
```

### 3. Обработка событий

Rust возвращает события в формате строки, например:

```text
Event { time: ..., name: Some("a"), event_type: KeyPress(KeyA) }
```

Вы можете использовать RegExp или парсить по-своему.

### 4. Остановка

```js
stop();
```

---

## 📚 API

### `start(callback: (event: string) => void)`

Запускает глобальный слушатель и вызывает `callback` при каждом событии.

* **callback** — функция, которая получает строковое описание события.

### `stop()`

Останавливает слушатель.

---

## 🔧 Разработка

Если вы собираетесь использовать `rustiohook` в dev-среде или вне NPM:

```bash
npm install
npm run build       # Release build
npm run build:debug # Debug + отладочная информация
```

---

## 🧪 Тестирование

```bash
npm test
```

Это запустит `__test__/index.spec.js`, где можно ввести символы и посмотреть события.

---

## ❓ Часто задаваемые вопросы

### Почему события приходят как строки?

Поскольку это low-level hook через Rust, события возвращаются в debug-строке. Это позволяет легко видеть всю информацию без сериализации/десериализации.

### Какие платформы поддерживаются?

* ✅ Windows (MSVC)
* ✅ Linux (x64)
* ✅ macOS (Intel + Apple Silicon)

> Другие архитектуры/OS могут потребовать ручную сборку.

---

## 🛠️ Требования

* Node.js >= 18
* npm >= 8
* Rust (`rustup`, `cargo`) — только для ручной сборки

---

## 📝 Лицензия

MIT © 2025 [Xzdes](https://github.com/Xzdes)

```