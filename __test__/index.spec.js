// --- __test__/index.spec.js (ФИНАЛ) ---
const { start, stop } = require('../index.js');

console.log('--- Scanner Listener Active ---');
console.log('Listener starting... Please scan a barcode now.');
console.log('Press Ctrl+C to exit.');

let buffer = '';

// start() вызывает этот коллбэк для КАЖДОГО события от Rust
start((...args) => {
    // Ваш рабочий способ получения данных
    const [rawEventString] = args;

    // В JS может прийти null, если что-то пошло не так при передаче
    if (!rawEventString) {
        return;
    }

    // Игнорируем все, кроме нажатий клавиш
    if (!rawEventString.includes('KeyPress')) {
        return;
    }

    // Если это Enter, выводим результат
    if (rawEventString.includes('KeyPress(Return)') || rawEventString.includes('KeyPress(KpReturn)')) {
        if (buffer.length > 0) {
            console.log(`\n✅ Scanned Data: [${buffer}]\n`);
            buffer = '';
        }
        return;
    }

    // Извлекаем символ из поля 'name'
    const match = rawEventString.match(/name: Some\("([^"]+)"\)/);
    if (match && match[1]) {
        const char = match[1];
        // Игнорируем управляющие символы
        if (char !== '\\r' && char !== '\\n' && char !== '\\t') {
            buffer += char;
        }
    }
});

process.on('SIGINT', () => {
    stop();
    process.exit();
});

// Держим процесс живым
setInterval(() => {}, 1000 * 60);