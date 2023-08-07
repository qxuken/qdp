import process from 'node:process';
import { spawn } from 'node:child_process';
import chokidar from 'chokidar';
import dotenv from 'dotenv';
import { postLiveReloadEvent, runLiveReloadServer } from './dev.livereload.mjs';
import {
  fromEvent,
  debounceTime,
  merge,
  tap,
  switchMap,
  startWith,
  retryWhen,
  delayWhen,
  timer,
} from 'rxjs';
import { build } from './build.assets.mjs';

dotenv.config();

const LIVE_RELOAD_PORT = 5555;
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

let back = chokidar.watch(['src/**/*.rs', 'cargo.toml'], {
  ignoreInitial: true,
});

let handlebars = chokidar.watch(['src/**/*.hbs'], {
  ignoreInitial: true,
});

let assets = chokidar.watch(
  ['src/**/*.{ts,css}', 'postcss.config.js', 'build.assets.mjs', 'tsconfig.json'],
  {
    ignoreInitial: true,
  },
);

let backSub = merge(
  fromEvent(env, 'change'),
  fromEvent(back, 'change'),
  fromEvent(handlebars, 'add'),
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
  fromEvent(handlebars, 'change'),
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
  handlebars.close();
  assets.close();
});
