import fs from 'node:fs';
import path from 'node:path';

/**
 * @param {string} from
 * @returns {import('esbuild').Plugin}
 */
export default function copy(from) {
  return {
    name: 'copy',
    setup(build) {
      let outdir = build.initialOptions.outdir;
      if (!outdir) {
        throw new Error('outdir should be defined');
      }
      build.onStart(function () {
        fs.cpSync(from, outdir, { recursive: true, force: true });
      });
    },
  };
}
