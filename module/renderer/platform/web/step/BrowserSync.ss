let browserSync = require('browser-sync');
let path = require( 'path' );
let rootPath = path.join( __dirname, '../../../../..' );

let o = 
{
  open: 'local',
  server: 
  { 
    baseDir : path.join( rootPath, 'target/web' ),
    index: 'index.html'
  },
  startPath : 'index.html',
  watch: true,
  ui: false
}
browserSync( o );