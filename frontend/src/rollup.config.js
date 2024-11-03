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
    format: 'iife', // Immediately Invoked Function Expression suitable for <script> tags
    name: 'MyApp', // Name of the global variable that will hold the app
    sourcemap: true, // Generate sourcemaps
  },
  plugins: [
    resolve(), // Resolve node_modules
    commonjs(), // Convert CommonJS modules to ES6
    babel({
      babelHelpers: 'bundled',
      exclude: 'node_modules/**', // Only transpile our source code
      presets: ['@babel/preset-env', '@babel/preset-react'],
    }),
    postcss(), // Process CSS files
    json(), // Allow importing JSON files
    image(), // Import image files
    production && terser(), // Minify the code in production
    !production && serve({
      open: true,
      contentBase: 'dist', // Serve from the 'dist' directory
      port: 3000, // Serve on port 3000
    }),
    !production && livereload('dist'), // Live reload in development
  ],
  watch: {
    clearScreen: false,
  },
};
