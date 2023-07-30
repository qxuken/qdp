import fs from 'node:fs';
import path from 'node:path';

/**
 * @returns {import('esbuild').Plugin}
 */
export default function clean() {
  return {
    name: 'clean',
    setup(build) {
      let outdir = build.initialOptions.outdir;
      if (!outdir) {
        throw new Error('outdir should be defined');
      }
      build.onStart(function () {
        fs.rmSync(outdir, { recursive: true, force: true });
      });
    },
  };
}
