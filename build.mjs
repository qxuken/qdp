import path from 'node:path';
import esbuild from 'esbuild';
import autoprefixer from 'autoprefixer';
import postcssMinify from 'postcss-minify';
import tailwindcss from 'tailwindcss';
import postcssConfig from './postcss.config.js';

import clean from './buildPlugins/clean.mjs';
import copy from './buildPlugins/copy.mjs';
import searchModules from './buildPlugins/searchModules.mjs';
import postcss from './buildPlugins/postcss.mjs';

const SRC_PATH = 'src';
const STATIC_PATH = 'public';
const TARGET_PATH = 'dist';

const rootDir = process.cwd();

let entryPoints = [path.join(SRC_PATH, 'lib.ts')].concat(searchModules(rootDir, SRC_PATH));

await esbuild.build({
  logLevel: 'info',
  entryPoints,
  bundle: true,
  minify: true,
  sourcemap: true,
  outdir: TARGET_PATH,
  legalComments: 'none',
  plugins: [clean(), copy(STATIC_PATH), postcss({ rootDir, plugins: postcssConfig.plugins })],
});
