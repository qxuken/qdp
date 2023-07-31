import { spawn } from 'node:child_process';
import chokidar from 'chokidar';
import { build } from './build.mjs';

let cargo;

function spawnBack() {
  console.log('[Running back build]');
  cargo?.kill();
  cargo = spawn('cargo', ['run'], {
    stdio: ['inherit', 'inherit', 'inherit'],
  });
}
spawnBack();

let back = chokidar
  .watch(['src/**/*.{rs,hbs}', 'cargo.toml'], {
    awaitWriteFinish: {
      stabilityThreshold: 250,
      pollInterval: 100,
    },
  })
  .on('change', spawnBack);

let assets = chokidar
  .watch(['src/**/*.{ts,css}', 'postcss.config.js', 'build.mjs', 'tsconfig.json', 'package.json'], {
    awaitWriteFinish: {
      stabilityThreshold: 250,
      pollInterval: 100,
    },
  })
  .on('change', build);

process.on('exit', () => {
  cargo.kill();
  back.close();
  assets.close();
});
