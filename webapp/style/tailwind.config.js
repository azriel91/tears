/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    // Relative to workspace root / where you run `cargo leptos` from.
    '**/src/*.rs',
    '**/src/**/*.rs',
  ],
  theme: {
    extend: {
      backgroundImage: {
        'cross': "url('cross.svg')",
      }
    },
  },
  plugins: [],
}
