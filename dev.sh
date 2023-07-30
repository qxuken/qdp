#!/bin/bash

cargo watch --ignore "*/**.{js,mjs,ts,css,sh}" -x run &
npm run build:watch
