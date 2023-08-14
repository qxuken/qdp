// eslint-disable-next-line @typescript-eslint/no-var-requires
const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,ts,tsx,hbs}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter var', 'Inter', ...defaultTheme.fontFamily.sans],
      },
    },
  },
  plugins: [],
};
