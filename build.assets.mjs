import esbuild from 'esbuild';
import path from 'node:path';
import { cwd, env } from 'node:process';
import { z } from 'zod';

import postcssConfig from './postcss.config.js';

import clean from './buildPlugins/clean.mjs';
import copy from './buildPlugins/copy.mjs';
import envPlugin from './buildPlugins/env.mjs';
import postcss from './buildPlugins/postcss.mjs';
import searchModules from './buildPlugins/searchModules.mjs';

const SRC_PATH = 'src';
const STATIC_PATH = 'public';
const TARGET_PATH = 'dist';

const envSchema = z.object({
  APPLICATION_MODE: z.enum(['production', 'development']).default('production'),
});

/**
 * @async
 * @param {Record<string, string>} define - pass variables to define
 * @returns {Promise<import('esbuild').BuildResult>}
 */
export function build(define = {}) {
  console.log('[Running assets build]');

  let envVariables = envSchema.parse(env);

  let rootDir = cwd();

  let entryPoints = [
    path.join(SRC_PATH, 'lib.ts'),
    path.join(SRC_PATH, 'liveReload.ts'),
  ].concat(searchModules(rootDir, SRC_PATH));

  return esbuild.build({
    logLevel: 'info',
    entryPoints,
    bundle: true,
    splitting: true,
    format: 'esm',
    minify: true,
    sourcemap: true,
    outdir: TARGET_PATH,
    legalComments: 'none',
    write: true,
    define,
    plugins: [
      envPlugin(envVariables),
      clean(),
      copy(STATIC_PATH),
      postcss({ rootDir, plugins: postcssConfig.plugins }),
    ],
  });
}
