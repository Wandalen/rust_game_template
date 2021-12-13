'use strict';

require( 'wTesting' );

const Express = require( 'express' );
const cv = require( 'opencv4nodejs-prebuilt' );
require( 'wpuppet' );


const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

__.include( 'wTestVisual' );

//

async function onSuiteBegin ( suite ) 
{
  const self = this;
  const suiteDirPath = __.path.dir( suite.suiteFilePath );
  self.suiteTempPath = __.path.join( suiteDirPath, '.tmp' );

  await self.chromiumDownloadMaybe( suite );
  await self.browserStackBegin();
  await self.serverStart();

}

//

async function onSuiteEnd () 
{
  const self = this;
  await self.browserStackEnd();
  await self.browserStackSessionClose();
  await self.serverStop();
}

//

async function onRoutineEnd ( tro )
{
  const self = this;
  await self.browserStackSessionStatusSet( tro );
  await self.browserStackSessionClose();
     
}

//

//

async function browserStackBegin () {

  const self = this;

  if( !self.browserStackEnabled )
  return false;

  __.assert( __.strDefined( self.browserStackUser ), "Env variable BROWSERSTACK_USER should be set." )
  __.assert( __.strDefined( self.browserStackAccessKey ), "Env variable BROWSERSTACK_KEY should be set." )

  self.browserStackLocal = await _.test.visual.browserstack.localBegin( self.browserStackAccessKey )
  return null;

}

//

function browserStackEnd () {

  const self = this;

  if( !self.browserStackEnabled )
  return false;
  return _.test.visual.browserstack.localEnd( self.browserStackLocal );

}

//

async function browserStackSessionStatusSet( tro )
{
    let context = this;

    if( !context.browserStackEnabled ) return;
    if( !context.browserStackCurrentSession ) return 

    return _.test.visual.browserstack.sessionStatusSet
    ({ 
      sid: context.browserStackCurrentSession, 
      user: context.browserStackUser, 
      key: context.browserStackAccessKey,
      tro 
    })
}

//

async function browserStackSessionClose()
{
  let context = this;

  if( !context.browser ) return;
  if( !context.browser.isConnected() ) return;
      
  try
  {
    return context.browser.close();
  }
  catch( err )
  {
    __.errLogOnce( err );
  }
}

//

let ChromiumInstalled = false;

async function chromiumDownloadMaybe ( suite ) 
{
  if( ChromiumInstalled ) return true;
  const downloadedVersion = await _.test.visual.puppeteer.chromiumDownload();
  if( downloadedVersion ) 
  {
    suite.logger.log( `Downloaded Chromium ${downloadedVersion}` );
  }
  ChromiumInstalled = true;
  return true;
}

//

function serverStart () 
{
  const context = this;
  const distPath = __.path.join( __dirname, '../target/web' );

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

  const a = _.test.visual.assetFor
  ({
    test,
    assetName,
    routinePath,
    browserDimensions: context.browserDimensions,
    browserStackEnabled: context.browserStackEnabled,
    browserStackUser: context.browserStackUser,
    browserStackAccessKey: context.browserStackAccessKey,
    browserStackIdleTimeoutInSec: context.browserStackIdleTimeoutInSec,
    browserStackConfigs: context.configs,

  });

  a.serverPath = __.uri.join( `http://localhost:${context.port}` );
  a.canvasSelector = 'canvas';
  a.onBrowserStackSessionChanged = onBrowserStackSessionChanged;
  a.onPageLoad = onPageLoad;
  a.onBeforeRoutine = onBeforeRoutine;
  a.colorAt = colorAt;
  a.avgColorAt = avgColorAt;
  a.canvasSave = canvasSave;
  a.circleDraw = circleDraw;

  return a;

  //

  function onBrowserStackSessionChanged( sid ) 
  {
    context.browserStackCurrentSession = sid;
  }

  //

  async function onPageLoad()
  {
    await this.page.goto( this.serverPath );
  }   

  //

  async function onBeforeRoutine() 
  {
  }

  //

  async function colorAt( x, y ) 
  {
    const path = this.mobile ? this.abs( 'canvas.png' ) : undefined;
    const buffer = await this.page.elementScreenshot( this.canvasSelector, path );
    const src = cv.imdecode( buffer );
    const raw = src.atRaw( y, x );
    const result = [ raw[ 2 ] / 255, raw[ 1 ] / 255, raw[ 0 ] / 255 ];
    return result;
  }

  //

  async function avgColorAt( x, y, x2, y2 ) 
  {
    __.assert( x2 > x );
    __.assert( y2 > y );

    const path = this.mobile ? this.abs( 'canvas.png' ) : undefined;
    const buffer = await this.page.elementScreenshot( this.canvasSelector, path );
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
    const canvasPath = this.abs( 'canvas.png' );
    await this.page.elementScreenshot( this.canvasSelector, canvasPath );
  }

  //

  async function circleDraw( points, name ) 
  {
    const path = this.mobile ? this.abs( 'canvas.png' ) : undefined;
    const buffer = await this.page.elementScreenshot( this.canvasSelector, path );
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

  onRoutineEnd,

  context: 
  {
    port: null,
    app: null,
    server: null,

    suiteTempPath: null,

    browserDimensions: [ 800, 600 ],

    //BrowserStack toggle

    browserStackEnabled : false,

    //BrowserStack general settings

    browserStackUser: process.env.BROWSERSTACK_USER,
    browserStackAccessKey : process.env.BROWSERSTACK_KEY,
    browserStackIdleTimeoutInSec : 30,

    //BrowserStack testing configurations
    //Format: "[os]-[os_version];[browser]-[browser_version]"
    //Format: "device_name"

    configs : [ 

      'Windows-10;Chrome-90', 
      // 'OS X-Catalina;Chrome-90',
      // 'Samsung Galaxy S20'
    ],

    //

    browserStackLocal : null,
    browserStackCurrentSession : null,

    browser: null,

    chromiumDownloadMaybe,
    serverStart,
    serverStop,
    browserStackBegin,
    browserStackEnd,
    browserStackSessionStatusSet,
    browserStackSessionClose,
    assetFor,
  }
}

//

module.exports = wTestSuite( Suite );
