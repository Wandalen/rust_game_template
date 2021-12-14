let browserSync = require('browser-sync');
let path = require( 'path' );
let rootPath = path.join( __dirname, '../../../../..' );

let o = 
{
  server: path.join( rootPath, 'target/web' ),
  watch: true
}
browserSync( o );