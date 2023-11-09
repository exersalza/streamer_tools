module.exports = {
  purge: {
    mode: "all",
    content: [
      "./src/**/*.rs",
      "./index.html",
      "./src/**/*.html",
      "./src/**/*.css",
    ],
  },
  theme: {
    colors: {
      "base": "#000000",
      "base-light": "#212529",
      "text": "#ffffff",
      "text-dark": "#878687",
      "accent": "#2e2e56"
    }
  },
  variants: {},
  plugins: [],
};
