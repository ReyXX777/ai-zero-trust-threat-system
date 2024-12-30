// rollup.config.js
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import babel from '@rollup/plugin-babel';
import { terser } from 'rollup-plugin-terser';
import postcss from 'rollup-plugin-postcss';
import serve from 'rollup-plugin-serve';
import livereload from 'rollup-plugin-livereload';
import json from '@rollup/plugin-json';
import image from '@rollup/plugin-image';

const production = !process.env.ROLLUP_WATCH;

export default {
  input: 'src/index.js', // Entry point of your application
  output: {
    file: 'dist/bundle.js',
    format: 'iife', // Immediately Invoked Function Expression for inclusion in HTML <script>
    name: 'MyApp', // Global variable name for your app
    sourcemap: true, // Generate source maps for easier debugging
  },
  plugins: [
    resolve(), // Resolves node_modules for external dependencies
    commonjs(), // Converts CommonJS modules to ES6 for bundling
    babel({
      babelHelpers: 'bundled', // Use bundled Babel helpers for compatibility
      exclude: 'node_modules/**', // Only transpile source code, not node_modules
      presets: ['@babel/preset-env', '@babel/preset-react'], // JSX & ES6 transpiling
    }),
    postcss(), // Processes CSS files (you may want to configure plugins like autoprefixer)
    json(), // Allows importing JSON files directly
    image(), // Allows importing image files
    production && terser(), // Minifies the code in production builds
    !production && serve({
      open: true,
      contentBase: 'dist', // Serve the 'dist' folder
      port: 3000, // Serve on port 3000
    }),
    !production && livereload('dist'), // Enables live reloading in development mode
  ],
  watch: {
    clearScreen: false, // Prevents clearing the console when watching for changes
  },
};
