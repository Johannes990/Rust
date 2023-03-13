use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Result as IoResult;

fn function_1() -> fmt::Result      // <- bringing two types with the same name into scope
{                                   //  | requires using their parent modules
                                    //  |
}                                   //  |
                                    //  |
fn function_2() -> io::Result<()>   // <-
{                                   //
                                    //
}                                   //
                                    //
fn function_3() -> IoResult<()>     // <- or we specify an alias for the type after the path
{                                   // like 'use std::io::Result as IoResult;'
    //
}

fn main()
{
    let mut map = HashMap::new();
    map.insert(1, 2);
}