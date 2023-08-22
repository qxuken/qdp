import process from 'node:process';
import { spawn } from 'node:child_process';
import chokidar from 'chokidar';
import dotenv from 'dotenv';
import { postLiveReloadEvent, runLiveReloadServer } from './dev.livereload.mjs';
import {
  of,
  fromEvent,
  debounceTime,
  merge,
  tap,
  switchMap,
  startWith,
  retryWhen,
  delayWhen,
  timer,
  retry,
  map,
} from 'rxjs';
import { build } from './build.assets.mjs';

dotenv.config();

const LIVE_RELOAD_PORT = 5555;
const BACKEND_PORT = process.env.APPLICATION_PORT ?? '8080';
const HEALTHCHECK_URL = `http://localhost:${BACKEND_PORT}/api/health`;
let assetsBuildDefines = {};

if (process.env.APPLICATION_MODE === 'development') {
  runLiveReloadServer(LIVE_RELOAD_PORT);
  assetsBuildDefines['LIVE_RELOAD_URL'] = JSON.stringify(
    `http://localhost:${LIVE_RELOAD_PORT}`,
  );
}

let cargo;

let env = chokidar.watch(['.env'], {
  ignoreInitial: true,
});

let back = chokidar.watch(['src/**/*.rs', 'cargo.toml', 'askama.toml'], {
  ignoreInitial: true,
});

let templates = chokidar.watch(['src/**/*.html'], {
  ignoreInitial: true,
});

let assets = chokidar.watch(
  [
    'src/**/*.{ts,css}',
    'public/**/*',
    'postcss.config.js',
    'tailwind.config.js',
    'build.assets.mjs',
    'tsconfig.json',
  ],
  {
    ignoreInitial: true,
  },
);

let backSub = merge(
  fromEvent(env, 'change'),
  fromEvent(back, 'change'),
  fromEvent(templates, 'change'),
)
  .pipe(
    debounceTime(200),
    startWith('init'),
    tap(() => cargo?.kill()),
    tap(() => {
      cargo = spawn('cargo', ['run'], {
        stdio: ['inherit', 'inherit', 'inherit'],
      });
    }),
    switchMap((event) =>
      of('health check').pipe(
        switchMap(() => fetch(HEALTHCHECK_URL)),
        map((res) => {
          if (!res.ok) {
            throw new Error('backend is not ready');
          }
          return event;
        }),
        retry({ delay: 250 }),
      ),
    ),
    tap((event) => postLiveReloadEvent({ event })),
    retryWhen((errors) =>
      errors.pipe(
        tap((e) => console.error(e)),
        delayWhen((val) => timer(1000)),
      ),
    ),
  )
  .subscribe();

let buildAssetsSub = merge(
  fromEvent(env, 'change'),
  fromEvent(assets, 'change'),
  fromEvent(assets, 'unlink'),
  fromEvent(templates, 'change'),
)
  .pipe(
    debounceTime(200),
    startWith('init'),
    switchMap((event) =>
      build(assetsBuildDefines).then((buildData) => ({
        event,
        buildData,
      })),
    ),
    switchMap((event) =>
      of('health check').pipe(
        switchMap(() => fetch(HEALTHCHECK_URL)),
        map((res) => {
          if (!res.ok) {
            throw new Error('backend is not ready');
          }
          return event;
        }),
        retry({ delay: 250 }),
      ),
    ),
    tap((data) => postLiveReloadEvent(data)),
    retryWhen((errors) =>
      errors.pipe(
        tap((e) => console.error(e)),
        delayWhen((val) => timer(1000)),
      ),
    ),
  )
  .subscribe();

process.on('exit', () => {
  backSub.unsubscribe();
  buildAssetsSub.unsubscribe();
  cargo?.kill();
  back.close();
  templates.close();
  assets.close();
});
