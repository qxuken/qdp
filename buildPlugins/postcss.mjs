import fs from 'node:fs/promises';
import util from 'node:util';
import path from 'node:path';
import postcssCompiler from 'postcss';

/**
 * @param {{ rootDir: string, plugins: import('postcss').AcceptedPlugin }} options
 * @returns {import('esbuild').Plugin}
 */
export default function postcss(options = { rootDir, plugins: [] }) {
  return {
    name: 'postcss',
    setup(build) {
      let { outdir } = build.initialOptions;
      let { rootDir } = options;
      let tmpFiles = [];
      let tmpDirPath = path.join(outdir, '.tmp-postcss');
      build.onStart(() => fs.mkdir(tmpDirPath));
      build.onResolve({ filter: /.\.(css)$/, namespace: 'file' }, async (args) => {
        let sourceFullPath = path.resolve(args.resolveDir, args.path);
        let sourceExt = path.extname(sourceFullPath);
        let sourceBaseName = path.basename(sourceFullPath, sourceExt);
        let sourceDir = path.dirname(sourceFullPath);
        let sourceRelDir = path.relative(path.dirname(rootDir), sourceDir);

        let tmpDir = path.resolve(tmpDirPath, sourceRelDir);
        let tmpFilePath = path.resolve(tmpDir, `${sourceBaseName}.css`);

        await fs.mkdir(tmpDir, { recursive: true });

        let css = await fs.readFile(sourceFullPath);

        let result = await postcssCompiler(options.plugins).process(css, {
          from: sourceFullPath,
          to: tmpFilePath,
        });

        // Write result file
        await fs.writeFile(tmpFilePath, result.css);

        return {
          path: tmpFilePath,
          namespace: 'file',
        };
      });
      build.onDispose(() => fs.rm(tmpDirPath, { recursive: true }));
    },
  };
}
