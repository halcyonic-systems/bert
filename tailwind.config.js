/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: [ '*.html', './src/**/*.rs' ],
    transform: {
      rs: ( content ) => content.replace( /(?:^|\s)class:/g, ' ' ),
    },
  },
  theme: {
    extend: {
      fontFamily: {
        tree: [ 'roboto-condensed', 'Avenir', 'sans-serif' ],
      },
    },
  },
  plugins: [
    require( '@tailwindcss/forms' ),
  ],
};