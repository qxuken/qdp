import fs from 'node:fs';
import path from 'node:path';

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
 * @param {RegExp} [extension]
 * @returns {Array<string>}
 */
export default function searchModules(baseDir, srcPath, extension = /mod.ts$/) {
  return getAllFiles(path.join(baseDir, srcPath))
    .filter((path) => path.match(extension))
    .map((path) => path.replace(`${baseDir}/`, ''));
}
