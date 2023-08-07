import express from 'express';
import bodyParser from 'body-parser';
import cors from 'cors';
import { randomUUID } from 'node:crypto';

let clients = new Set();

const CORS_OPTIONS = {
  origin: function (origin, callback) {
    callback(null, true);
  },
  credentials: true,
};

export function runLiveReloadServer(liveReloadPort) {
  let app = express();

  app.use(cors(CORS_OPTIONS));
  app.use(bodyParser.json());
  app.use(bodyParser.urlencoded({ extended: false }));

  app.get('/status', (request, response) => response.json({ clients: clients.length }));

  function eventsHandler(request, response, next) {
    let headers = {
      'Content-Type': 'text/event-stream',
      Connection: 'keep-alive',
      'Cache-Control': 'no-cache',
    };
    response.writeHead(200, headers);
    response.flushHeaders();

    response.write('init');

    clients.add(response);

    request.on('close', () => {
      console.log(`Connection closed`);
      clients.delete(response);
    });
  }

  app.get('/', eventsHandler);

  app.listen(liveReloadPort, () => {
    console.log(`[Running live reload server]`);
    console.log(
      `Live Reload Events service listening at http://localhost:${liveReloadPort}`,
    );
  });
}

export function postLiveReloadEvent(update) {
  let data = JSON.stringify(update ?? 'no data');
  clients.forEach((client) => {
    client.write(`id: ${randomUUID()}\n`);
    client.write('event: update\n');
    client.write(`data: ${data} \n\n`);
  });
}
