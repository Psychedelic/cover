import fs from 'fs';

import FileHound from 'filehound';

const args = process.argv;
if (args.length < 3) throw new Error('Missing path arguments');

const files = FileHound.create().paths(args[2]).discard('node_modules').ext('js').find();

files
  .then(filePaths =>
    filePaths.forEach(filepath => {
      fs.readFile(filepath, 'utf8', (err, data) => {
        if (err) throw err;
        const re = /(?<before>import.+['"]\..+)(?<after>['"])/gu;
        if (!data.match(re)) {
          return;
        }
        const newData = data.replace(re, '$1.js$2');
        fs.writeFile(filepath, newData, e => {
          if (e) throw e;
        });
      });
    })
  )
  .catch(err => {
    throw err;
  });
