// Option 2: without as
use std::io;
use std::fmt;

fn function3() -> fmt::Result {

}

fn function4() -> io::Result<()> {
    
}


// Option 1: using as
use std::fmt::Result;
use std::io::Result as IoResult;  // the "as" keyword allows us to create an alias avoiding name conflicts

fn function1() -> Result {  // here we mean std::fmt::Result
    // --snip--
}

fn function2() -> IoResult<()> { // here we mean std::io::Result
    // --snip--
}