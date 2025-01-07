/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "../../**/*.{html,rs}",
    "../../target/tailwindcss.txt",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
