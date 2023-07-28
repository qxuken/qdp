import fs from 'node:fs';
import path from 'node:path';
import { SRC_PATH, TARGET_PATH } from './rollup.consts.mjs';

/**
 * @param {string} dirPath
 * @returns {Array<string>}
 */
function getAllFiles(dirPath) {
  return fs.readdirSync(dirPath).flatMap((file) => {
    let entryPath = path.join(dirPath, file);
    if (fs.statSync(entryPath).isDirectory()) {
      return getAllFiles(entryPath);
    } else {
      return entryPath;
    }
  });
}

/**
 * @param {string} baseDir
 * @param {string} file
 * @returns {{ file: string, outputDir: string }}
 */
export function pathToEntry(baseDir, file) {
  return {
    file: path.join(SRC_PATH, file),
    outputDir: path.join(TARGET_PATH, file.substring(0, file.lastIndexOf('/'))),
  };
}

/**
 * @param {string} baseDir
 * @param {RegExp} [extension]
 * @returns {Array<{ file: string, outputDir: string }>}
 */
export function searchModules(baseDir, extension = /mod.ts$/) {
  let srcPath = path.join(baseDir, SRC_PATH);
  return getAllFiles(srcPath)
    .filter((path) => path.match(extension))
    .map((path) => path.replace(srcPath, ''))
    .map((path) => pathToEntry(baseDir, path));
}
