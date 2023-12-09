'use strict';

const fs = require('fs');

const OUTPUT_DIR = '_site';

module.exports = (req, res, next) => {
  const path = req.originalUrl || req.url;

  if (!fs.existsSync(`${OUTPUT_DIR}/${path}`) &&
      fs.existsSync(`${OUTPUT_DIR}/${path}.html`)) {
    req.url = `${path}.html`;
  }

  next();
};
