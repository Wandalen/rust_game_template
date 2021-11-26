var browserSync = require( 'browser-sync' );

browserSync
({
  server : 'target/web',
  files : ['target/web/*.html', 'target/web/*.js' ],
});
