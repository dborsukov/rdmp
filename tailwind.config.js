/* eslint-disable no-undef */
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    fontFamily: {
      sans: ['Roboto', 'sans-serif'],
      mono: ['RobotoMono', 'monospace'],
    },
    extend: {},
  },
  darkMode: 'class',
  plugins: [require('tailwind-scrollbar')({ nocompatible: true })],
};
