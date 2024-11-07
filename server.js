// server.js
const express = require('express');
const app = express();
const PORT = 8080;

// ヘッダーの設定
app.use((req, res, next) => {
    res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
    res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
    next();
});

// カレントディレクトリを静的ファイル提供のために設定
app.use(express.static(__dirname + '/dist/'));

// ルートにGETリクエストがあった場合のレスポンス
app.get('/', (req, res) => {
    res.sendFile(__dirname + 'index.html');
});

// サーバーの起動
app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});