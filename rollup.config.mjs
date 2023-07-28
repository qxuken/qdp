import fs from 'node:fs/promises';
import copy from 'rollup-plugin-copy';
import resolve from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import terser from '@rollup/plugin-terser';
import postcss from 'rollup-plugin-postcss';
import { defineConfig } from 'rollup';
import { scanPath } from './rollup.consts.mjs';
import { searchModules, pathToEntry } from './rollup.entries.mjs';

const staticModules = [
  {
    ...pathToEntry(scanPath, 'lib.ts'),
    plugins: [copy({ targets: [{ src: 'public/**/*', dest: 'dist' }] })],
  },
];

/**
 * @async
 * @returns {import('rollup').RollupOptions}
 */
async function generateConfigs() {
  let files = staticModules.concat(searchModules(scanPath));
  await Promise.allSettled(files.map((file) => fs.rm(file.outputDir, { recursive: true })));
  return files.map((file) =>
    defineConfig({
      input: file.file,
      output: {
        dir: file.outputDir,
        format: 'cjs',
        sourcemap: true,
      },
      plugins: [
        ...(file.plugins ?? []),
        typescript(),
        resolve(),
        postcss({
          extract: 'style.css',
        }),
        terser(),
      ],
    }),
  );
}

export default await generateConfigs();
