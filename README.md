# rustiohook

`rustiohook` — это нативный Node.js-модуль на базе Rust, предназначенный для глобального перехвата событий клавиатуры и мыши. Подходит, например, для сканеров штрих-кодов, которые эмулируют ввод с клавиатуры.

## Установка

```bash
npm install rustiohook
```

> 📦 Поддержка только для `win32-x64-msvc`. Предсобранные бинарники автоматически публикуются при релизе.

## Использование

```js
const { start } = require('rustiohook');

start((rawEvent) => {
  if (!rawEvent) return;

  console.log('Received event from Rust:', rawEvent);

  if (rawEvent.includes('KeyPress(KeyA)')) {
    console.log('A key was pressed');
  }
});
```

> Для остановки модуля требуется завершить процесс Node.js (например, `Ctrl+C` в терминале).

## Пример использования

Пример доступен в `__test__/index.spec.js`.

```bash
node __test__/index.spec.js
```

Примерный вывод:

```
--- Scanner Listener Active ---
Listener starting... Please scan a barcode now.
Press Ctrl+C to exit.

✅ Scanned Data: [0123456789]
```

## API

### `start(callback: (rawEvent: string | null) => void)`

Запускает глобальный слушатель событий. Rust вызывает переданный колбэк при каждом событии. `rawEvent` — это строка, сериализованная из Rust-структуры `Event`, или `null`, если передача не удалась.

## Поддержка

* ✔ Windows (x64, MSVC)
* ✖ macOS, Linux — в планах

## Лицензия

MIT