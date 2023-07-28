/** @type {import('postcss-load-config').Config} */
const config = {
  plugins: [require('tailwindcss'), require('autoprefixer'), require('postcss-minify')],
};

module.exports = config;
