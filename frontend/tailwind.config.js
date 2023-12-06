/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs"
  ],
  theme: {
    extend: {
      colors: {
        "text": "#ffffff",
        "base": "#000000",
        "base-light": "#212529",
        "text-dark": "#878687",
        "accent": "#2e2e56",

        // https://colorhunt.co/palette/164863427d9d9bbec8ddf2fd
        "light-base": "#164863",
        "light-base-light": "#427D9D",
        "light-accent": "#9BBEC8",
        "light-accent-light": "#DDF2FD"
      }
    },
  },
  plugins: [],
}

