use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use libloading::{Library, Symbol};

/// signature of function which should be called from plugin
type AddFunc = unsafe fn(isize, isize) -> isize;

/// Create a plugin file at runtime which will be converted to shared library
fn write_file()  -> std::io::Result<()> {
    let mut file = File::create("plugin.rs")?;
    file.write_all(b"fn main() {\n")?;
    file.write_all(b"\t#[no_mangle]\n")?;
    file.write_all(b"\tpub extern \"C\" fn add(a: isize, b: isize) -> isize {\n")?;
    file.write_all(b"\t\ta + b\n")?;
    file.write_all(b"\t}\n")?;
    file.write_all(b"}\n")?;
    Ok(())
}
/// compile plugin code file to shared library
/// todo 1) should allow to pass file path. 
///      2) should return path to generated shared library
fn compile_file() {
    let mut compile_file = Command::new("cmd");
    compile_file.args(&["/C", "rustc", "--crate-type", "cdylib", "plugin.rs"]).status().expect("process failed to execute");
}
/// call function from shared library
/// todo suffix should be selected based on OS.
fn call_plugin(a: isize, b: isize) -> isize {
    let lib = Library::new("plugin.dll").unwrap();
    unsafe {
        let func: Symbol<AddFunc> = lib.get(b"add").unwrap();
        let answer = func(a, b);
        answer
    }
}
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        write_file();
        compile_file();
        /// get argument from commandline to pass to function
        let a: isize = args[1].trim().parse().expect("number required");
        let b: isize = args[2].trim().parse().expect("number required");
        println!("{}+{}:{}",a,b,call_plugin(a,b));
    }
    else {
        println!("USAGE: main.exe NUM NUM");
    }
}
