import fs from 'node:fs/promises';
import util from 'node:util';
import path from 'node:path';
import postcssCompiler from 'postcss';

/**
 * @param {Record<string, string>} variables
 * @returns {import('esbuild').Plugin}
 */
export default function env(variables) {
  return {
    name: 'env',
    setup(build) {
      let options = build.initialOptions;
      if (!options.define) {
        options.define = {};
      }

      let envObject = Object.entries(variables).reduce((env, [key, value]) => {
        env[key] = JSON.stringify(value);
        return env;
      }, {});

      options.define = { ...options.define, ...envObject };
    },
  };
}
