const express = require('express');
const { createServer: createViteServer } = require('vite');

const isProduction = process.env.NODE_ENV === 'production';
const server = express();

if (!isProduction) {
  (async () => {
    const vite = await createViteServer({
      server: { middlewareMode: true },
    });
    server.use(vite.middlewares);
  })();
} else {
  server.use(express.static('dist'));
}

server.listen(3001, () => {
  console.log('Server is running on http://localhost:3001');
});
