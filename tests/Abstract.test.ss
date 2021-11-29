'use strict';

require( 'wTesting' );

const Puppeteer = require( 'puppeteer' );
const Revisions = require( 'puppeteer/lib/cjs/puppeteer/revisions.js' );
const Express = require( 'express' );
const cv = require( 'opencv4nodejs-prebuilt' );

const __ = _globals_.testing.wTools;

//

async function onSuiteBegin ( suite ) 
{
  const self = this;
  const suiteDirPath = __.path.dir( suite.suiteFilePath );
  self.suiteTempPath = __.path.join( suiteDirPath, '.tmp' );

  await self.chromiumDownloadMaybe( suite );

  return self.serverStart();

}

//

function onSuiteEnd () 
{
  const self = this;
  return self.serverStop();
}

//

let ChromiumInstalled = false;

async function chromiumDownloadMaybe () 
{
  if( ChromiumInstalled ) return true;
  const browserFetcher = Puppeteer.createBrowserFetcher();
  const targetRevision = Revisions.PUPPETEER_REVISIONS.chromium;
  const localRevisions = await browserFetcher.localRevisions();
  if( !__.longHas( localRevisions, targetRevision ) ) {

      suite.logger.log( `Downloading Chromium ${targetRevision}...` );
      await browserFetcher.download( targetRevision );
      suite.logger.log( `Downloaded.` );

  }
  ChromiumInstalled = true;
  return true;

}

//

function serverStart () 
{
  const context = this;
  const distPath = __.path.join( __dirname, '../module/renderer/platform/web/target/web' );

  __.assert( __.fileProvider.fileExists( distPath ), `The dist path: ${distPath} doesn't exist.` );

  const ready = __.Consequence();

  context.app = new Express();
  context.app.use( '/', Express.static( __.path.nativize( distPath ) ) );
  context.server = context.app.listen( 0, () => 
  {
      context.port = context.server.address().port;
      ready.take( null )
  });

  return ready;

}

//

function serverStop () 
{
  const context = this;
  const ready = __.Consequence();
  context.server.close( () => 
  {
      ready.take( null )
  });
  return ready;
}

//

function assetFor ( test, assetName ) 
{
  const context = this;
  const routinePath = __.path.join( context.suiteTempPath, test.name );

  const a = test.assetFor({ assetName, routinePath });

  a.serverPath = __.uri.join( `http://localhost:${context.port}` );
  a.canvasSelector = 'canvas';
  a.inBrowser = inBrowser;
  a.abs = abs;
  a.memoryUsedGb = memoryUsedGb;
  a.colorAt = colorAt;
  a.avgColorAt = avgColorAt;
  a.canvasSave = canvasSave;
  a.circleDraw = circleDraw;

  return a;

  /* */

  async function inBrowser ( routine ) 
  {
    try 
    {
      const width = 800;
      const height = 600;

      this.browser = await Puppeteer.launch
      ({
        headless: true,
        args: [ '--no-sandbox', '--enable-precise-memory-info', `--window-size=${ width },${ height }` ],
        defaultViewport: { width, height }
      });
      this.page = await this.browser.newPage();

      if( this.tro.verbosity >= 5 ) 
      {
        registerPageHandlers( this.page );
      }

      // __.assert( __.strDefined( this.entryPath ), 'Field entryPath should be defined on object returned by assetFor before execution of routine inBrowser.' );

      // const entryPathFull = this.fileProvider.path.join( __dirname, this.entryPath );
      // __.assert( this.fileProvider.fileExists( entryPathFull ), `Entry path: ${entryPathFull} doesn't exist.` );

      await this.page.goto( this.serverPath, { waitUntil: [ 'load' ] } );

      await routine( this.page );

      await this.browser.close();

    }
    catch ( err )
    {
      __.errAttend( err );

      if( this.browser ) 
      {
          const pages = await this.browser.pages();
          await __.Consequence.And.apply( __.Consequence, pages.map( ( page ) => page.close() ) )
          await this.browser.close();
      }

      throw __.err( err );

    }

    return null;

    //

    function registerPageHandlers( page ) 
    {
      page.on( 'console', async ( msg ) => 
      {
        console.log( msg.text() );
      });

      page.on( 'requestfailed', ( request ) => 
      {
        console.log( __.errBrief( `Failed to load: ${request.url()} ${request.failure().errorText}` ) );
      });

      page.on( 'pageerror', ( err ) => 
      {
        __.errLogOnce( `Page error: ${err.toString()}` );
      });
    }

  }

  //

  function abs ( ...args ) 
  {
    const tro = this.tro;

    let routinePath = this.routinePath;

    if( tro.case ) 
    {
      const splitted = __.strSplitNonPreserving( tro.case, ' ' );
      splitted.unshift( routinePath );
      routinePath = splitted.join( '_' );
    }

    args.unshift( routinePath );

    return __.uri.join.apply( __.uri, args );
  }

  //

  async function memoryUsedGb () {

    return this.page.evaluate( () =>
    {
      const global = window;
      let performance = global.performance;
      return performance.memory.totalJSHeapSize / Math.pow( 1024, 3 );
    });
  }

  //

  async function colorAt( x, y ) {

    const canvas = await this.page.$( this.canvasSelector );
    const buffer = await canvas.screenshot
    ({
        omitBackground: true,
    });
    const src = cv.imdecode( buffer );
    const raw = src.atRaw( y, x );
    const result = [ raw[ 2 ] / 255, raw[ 1 ] / 255, raw[ 0 ] / 255 ];
    return result;
  }

  //

  async function avgColorAt( x, y, x2, y2 ) {

    __.assert( x2 > x );
    __.assert( y2 > y );

    const canvas = await this.page.$( this.canvasSelector );
    const buffer = await canvas.screenshot
    ({
        omitBackground: true,
    });
    const src = cv.imdecode( buffer );

    let results = 0;
    const raw = [ 0, 0, 0 ];

    for( let _x = x; _x <= x2; _x++ ) 
    for( let _y = y; _y <= y2; _y++ ) 
    {

      const r = src.atRaw( _y, _x );
      results += 1;

      raw[ 0 ] += r[ 0 ]
      raw[ 1 ] += r[ 1 ]
      raw[ 2 ] += r[ 2 ]

    }
    return [ raw[ 2 ] / results / 255, raw[ 1 ] / results / 255, raw[ 0 ] / results / 255 ];
  }

  //

  async function canvasSave() 
  {
    const canvas = await this.page.$( this.canvasSelector );
    const canvasPath = __.path.nativize( this.abs( 'canvas.png' ) )
    this.fileProvider.dirMakeForFile( canvasPath );
    await canvas.screenshot
    ({
        path: canvasPath,
        omitBackground: true,
    });
  }

  //

  async function circleDraw( points, name ) 
  {
    const canvas = await this.page.$( this.canvasSelector );
    const buffer = await canvas.screenshot
    ({
        omitBackground: true,
    });
    const src = cv.imdecode( buffer );

    name = name ?? 'area.png';

    const filePath = __.path.nativize( this.abs( name ) );
    this.fileProvider.dirMakeForFile( filePath );

    const radius = 5;
    const thickness = 1;
    const color = new cv.Vec3( 0, 0, 0 );

    points.forEach( ( point ) => 
    {
        const center = new cv.Point2( point[ 0 ], point[ 1 ] );
        src.drawCircle( center, radius, color, thickness, cv.LINE_AA );
    });

    cv.imwrite( filePath, src );

  }
}

//

const Suite = 
{
  silencing: 1,

  abstract: 1,

  onSuiteBegin,
  onSuiteEnd,

  context: 
  {
    port: null,
    app: null,
    server: null,

    suiteTempPath: null,

    chromiumDownloadMaybe,
    serverStart,
    serverStop,
    assetFor,
  }
}

//

module.exports = wTestSuite( Suite );
