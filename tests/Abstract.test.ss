'use strict';

require( 'wTesting' );

const Puppeteer = require( 'puppeteer' );
const Revisions = require( 'puppeteer/lib/cjs/puppeteer/revisions.js' );
const Express = require( 'express' );
const cv = require( 'opencv4nodejs-prebuilt' );
require( 'wpuppet' );

const __ = _globals_.testing.wTools;
const puppet = _global_.wTools.puppet;

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
    if( context.browserStackEnabled ) return runOnBrowserStack.call( this );

    return act.call( this );

    function optionsForm( strategy, remoteConfig, mobile ) 
    {
      strategy = strategy ?? 'Puppeteer';
      mobile = mobile ?? false;
      remoteConfig = remoteConfig ?? null;
      let system = puppet.System({ strategy });
      system.form();
      let browser = remoteConfig ? 'browserstack' : null;
      let puppetOptions = 
      {
          dimensions: context.browserDimensions,
          system,
          remoteConfig,
          browser,
      }
      return { puppetOptions, mobile };
    }

    //
    
    function runOnBrowserStack() 
    {
        const { desktop, mobile } = generateBrowserStackConfigurations.call( this );
        let ready = __.Consequence().take( null );

        desktop.forEach( ( c ) => 
        {   
          const testGroup = `${c.os} ${c.os_version} ${c.browser} ${c.browser_version}`;
          const o = optionsForm( 'Puppeteer', c );
          ready.tap( () =>  test.open( testGroup ) );
          ready.then( () => __.Consequence.From( act.call( this, o ) ) );
          ready.tap( () =>  test.close( testGroup ) );
        })

        mobile.forEach( ( c ) => 
        {
          const { deviceName } = c.capabilities[ 'bstack:options' ];
          const testGroup = `${deviceName}`;
          const o = optionsForm( 'WebDriverIO', c, true );
          ready.tap( () =>  test.open( testGroup ) );
          ready.then( () => __.Consequence.From( act.call( this, o ) ) ) 
          ready.tap( () =>  test.close( testGroup ) );

        })

        return ready;
    }

    //

    function generateBrowserStackConfigurations() 
    {
      const desktop = [];
      const mobile = [];

      context.configs.forEach( ( src ) => 
      {
        const splits = __.strSplitNonPreserving
        ({ 
          src, 
          delimeter : [ ';', '-' ] 
        });

        if( splits.length === 1 ) 
        {
          const [ deviceName ] = splits;
          const capabilities = 
          {
            'bstack:options' : 
            {
              "projectName" : __.path.name( __.path.dir( test.suite.suiteFilePath ) ),
              "buildName" : __.path.name( __.path.dir( test.suite.suiteFilePath ) ),
              "sessionName" : test.name,
              "deviceName" : deviceName,
              "realMobile" : "true",
              "local" : true,
              "debug" : "true",
              "consoleLogs" : "verbose",
            }
          }
          const config = 
          {
            user : context.browserStackUser,
            key : context.browserStackAccessKey,
            capabilities,
            logLevel : 'error',
            host: 'hub-cloud.browserstack.com',
          }
          mobile.push( config );
        }
        else 
        {
          const [ os, os_version, browser, browser_version ] = splits;
          const config = 
          { 
            build: __.path.name( __.path.dir( test.suite.suiteFilePath ) ),
            name: test.name,
            
            browser,
            browser_version,
            os,
            os_version,

            'browserstack.username': context.browserStackUser,
            'browserstack.accessKey': context.browserStackAccessKey,
            'browserstack.idleTimeout' : context.browserStackIdleTimeoutInSec,
            'browserstack.local': 'true',
          }
          desktop.push( config );
        }
      });

      return { desktop, mobile };
    }

    async function act( o ) 
    {

      o = o || optionsForm();

      this.mobile = o.mobile;

      try 
      {
        this.browser = await puppet.windowOpen( o.puppetOptions );
        this.page = await this.browser.pageOpen();

        if( this.tro.verbosity >= 5 ) {

            registerPageHandlers( this.page );

        }

        if( context.browserStackEnabled ) {

            const sessionDetails = await this.page.sessionDetailsGet();
            context.browserStackCurrentSession = sessionDetails.hashed_id;
        }

        // __.assert( __.strDefined( this.entryPath ), 'Field entryPath should be defined on object returned by assetFor before execution of routine inBrowser.' );

        // const entryPathFull = this.fileProvider.path.join( __dirname, this.entryPath );
        // __.assert( this.fileProvider.fileExists( entryPathFull ), `Entry path: ${entryPathFull} doesn't exist.` );

        await this.page.goto( this.serverPath );

        await routine( this.page );

        await this.browser.close();

      }
      catch ( err ) 
      {
        __.errAttend( err );

        if( this.browser ) 
        {
          await this.browser.close();
        }

        throw __.err( err );

      }

      return null;

    }

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

    const path = this.mobile ? this.abs( 'canvas.png' ) : undefined;
    const buffer = await this.page.elementScreenshot( this.canvasSelector, path );
    const src = cv.imdecode( buffer );
    const raw = src.atRaw( y, x );
    const result = [ raw[ 2 ] / 255, raw[ 1 ] / 255, raw[ 0 ] / 255 ];
    return result;
  }

  //

  async function avgColorAt( x, y, x2, y2 ) {

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
    const path = this.mobile ? this.abs( 'canvas.png' ) : undefined;
    await this.page.elementScreenshot( this.canvasSelector, path );
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
      'OS X-Catalina;Chrome-90',
      // 'Samsung Galaxy S20'
    ],

    //

    browserStackLocal : null,
    browserStackCurrentSession : null,

    browser: null,

    chromiumDownloadMaybe,
    serverStart,
    serverStop,
    assetFor,
  }
}

//

module.exports = wTestSuite( Suite );
