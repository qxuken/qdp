import { spawn } from 'node:child_process';
import chokidar from 'chokidar';
import { fromEvent, debounceTime, merge, tap, switchMap, startWith } from 'rxjs';
import { build } from './build.assets.mjs';

let cargo;

let back = chokidar.watch(['src/**/*.rs', 'cargo.toml'], {
  ignoreInitial: true,
});

let backHbs = chokidar.watch(['src/**/*.hbs'], {
  ignoreInitial: true,
});

let backSub = merge(fromEvent(back, 'change'), fromEvent(backHbs, 'add'), fromEvent(backHbs, 'unlink'))
  .pipe(
    debounceTime(200),
    startWith('init'),
    tap(() => cargo?.kill()),
    tap(() => {
      cargo = spawn('cargo', ['run'], {
        stdio: ['inherit', 'inherit', 'inherit'],
      });
    }),
  )
  .subscribe();

let assets = chokidar.watch(
  ['src/**/*.{ts,css}', 'postcss.config.js', 'build.assets.mjs', 'tsconfig.json', 'package.json'],
  {
    ignoreInitial: true,
  },
);

let buildAssetsSub = fromEvent(assets, 'change')
  .pipe(
    debounceTime(200),
    startWith('init'),
    switchMap(() => build()),
  )
  .subscribe();

process.on('exit', () => {
  backSub.unsubscribe();
  buildAssetsSub.unsubscribe();
  cargo.kill();
  back.close();
  backHbs.close();
  assets.close();
});
