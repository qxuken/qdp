import { spawn } from 'node:child_process';
import chokidar from 'chokidar';
import { fromEvent, debounceTime, merge, tap, switchMap, startWith, catchError, EMPTY } from 'rxjs';
import { build } from './build.assets.mjs';

let cargo;

let back = chokidar.watch(['src/**/*.rs', 'cargo.toml'], {
  ignoreInitial: true,
});

let handlebars = chokidar.watch(['src/**/*.hbs'], {
  ignoreInitial: true,
});

let assets = chokidar.watch(['src/**/*.{ts,css}', 'postcss.config.js', 'build.assets.mjs', 'tsconfig.json'], {
  ignoreInitial: true,
});

let backSub = merge(fromEvent(back, 'change'), fromEvent(handlebars, 'add'))
  .pipe(
    debounceTime(200),
    startWith('init'),
    tap(() => cargo?.kill()),
    tap(() => {
      cargo = spawn('cargo', ['run'], {
        stdio: ['inherit', 'inherit', 'inherit'],
      });
    }),
    catchError((e) => {
      console.error(e);
      return EMPTY;
    }),
  )
  .subscribe();

let buildAssetsSub = merge(
  fromEvent(assets, 'change'),
  fromEvent(assets, 'unlink'),
  fromEvent(handlebars, 'change'),
)
  .pipe(
    debounceTime(200),
    startWith('init'),
    switchMap(() => build()),
    catchError((e) => {
      console.error(e);
      return EMPTY;
    }),
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
