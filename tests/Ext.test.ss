'use strict';

require( 'wTesting' );
const Abstract = require( './Abstract.test.ss' );

const __ = _globals_.testing.wTools;

//

async function trivial ( test ) {

  const context = this;
  const a = context.assetFor( test );

  return a.inBrowser( async ( page ) =>
  { 
    await __.time.out( 1000 );

    if( a.tro.verbosity >= 5 ) 
    {
      await a.canvasSave();
    }   

    let got = await a.colorAt( 492, 405 );
    let expected = [ 1.0, 0, 0 ];
    test.identical( got, expected );


  });

}

trivial.timeOut = 60000;

//

const Suite = {
  silencing : 1,

  tests : {
      trivial
  }
}

//

const Self = wTestSuite( Suite ).inherit( Abstract );
if( typeof module !== 'undefined' && !module.parent ) wTester.test( Self.name );
