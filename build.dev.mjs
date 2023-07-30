import nodemon from 'nodemon';

nodemon({
  script: 'build.mjs',
  ext: 'ts css hbs json',
  watch: ['src', 'buildPlugins', 'postcss.config.js', 'build.mjs', 'tsconfig.json', 'package.json'],
  delay: 1500,
});

nodemon.on('start', (files) => {
  console.log("[Running 'build.mjs']");
});

nodemon.on('restart', (files) => {
  console.log("[Running 'build.mjs']");
});
