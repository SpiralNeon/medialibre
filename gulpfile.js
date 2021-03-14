const { src, dest, series, parallel, watch } = require("gulp")
const mode = require("gulp-mode")()
const webpack = require("webpack-stream")
const postcss = require("gulp-postcss")
const csso = require("gulp-csso")
const gzip = require("gulp-gzip")
const sourcemaps = require("gulp-sourcemaps")
const printSpaceSavings = require("gulp-print-spacesavings")
const del = require("del")

const paths = {
  styles: "static/styles/*.css",
  js: "static/js/*.js",
}

function clean() {
  return del("static/build")
}

function styles() {
  return src(paths.styles)
    .pipe(mode.development(sourcemaps.init()))
    .pipe(postcss([
      require("autoprefixer"),
      require("tailwindcss"),
    ]))
    .pipe(mode.development(sourcemaps.write()))
    .pipe(mode.production(printSpaceSavings.init()))
    .pipe(mode.production(csso()))
    .pipe(mode.production(printSpaceSavings.print()))
    .pipe(gzip())
    .pipe(dest("static/build"))
}

function js() {
  return src(paths.js)
    .pipe(mode.development(sourcemaps.init()))
    .pipe(webpack({
      mode: process.env.NODE_ENV,
      output: {
        filename: "index.js",
      },
      module: {
        rules: [
          {
            test: /\.jsx?$/,
            use: {
              loader: "babel-loader",
              options: {
                presets: ["@babel/preset-env"],
                plugins: ["@babel/plugin-proposal-optional-chaining"],
              },
            },
          },
        ],
      },
    }))
    .pipe(mode.development(sourcemaps.write()))
    .pipe(gzip())
    .pipe(dest("static/build"))
}

module.exports = {
  styles,
  js,
  watch() {
    watch("styles/**/*.css", styles)
    watch(["src/**/*.js", "src/**/*.jsx"], js)
  },
  build: series(clean, parallel(styles, js)),
}
