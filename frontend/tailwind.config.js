/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs"
  ],
  theme: {
    extend: {
      colors: {
        "base": "#000000",
        "base-light": "#212529",
        "text": "#ffffff",
        "text-dark": "#878687",
        "accent": "#2e2e56"
      }
    },
  },
  plugins: [],
}

