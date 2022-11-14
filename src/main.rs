use std::convert::AsRef;
use std::fs::{File, OpenOptions};
use std::io::Write;


const HOOK_JS_WRITE_PATH: &str = "./node/raven/hook.js"; //path to hook.js in unpacked module, if you change it, change it in append_require_to_file too
#[cfg(feature = "no_embed")]
const NO_EMBED_HOOK_JS_PATH: &str = "./hook.js"; // if no_emded feature is enabled, this file will be used in runtime
#[cfg(not(feature = "no_embed"))]
const EMBED_HOOK_JS_BYTES: &[u8] = include_bytes!("hooklog.js");   // embedded hooking code file,will be embedded in binary file
const INJECT_JS_PATH: &str = "./node/raven/index.js"; //path for unpacked module to inject code into,if you want inject code into another module, change it here and in HOOK_JS_WRITE_PATH


fn main() {
    if file_exist("./node"){
        println!("You may have already installed the hook.Please check manually.");
        return;
    }
    if !file_exist("./resources/node_modules.asar") {
        println!("no node_modules.asar found");
        println!("move me to the root of your typora installation(the same directory as executable of electron)");
        return;
    }
    println!("extracting node_modules.asar");
    rasar::extract("./resources/node_modules.asar", "./node").unwrap();
    println!("adding hook.js");
    write_js_to_file();
    println!("applying patch");
    append_require_to_file();
    println!("packing node_modules.asar");
    rasar::pack("./node","./resources/node_modules.asar").unwrap();
    println!("done!");
}
fn file_exist(archive: &str) -> bool {
    return std::path::Path::new(archive).exists()
}


#[cfg(not(feature = "no_embed"))]
fn write_js_to_file() {
    let mut file = File::create(HOOK_JS_WRITE_PATH).unwrap();
    file.write_all(EMBED_HOOK_JS_BYTES).unwrap();
}
#[cfg(feature = "no_embed")]
fn write_js_to_file() {
    let mut file = File::create(HOOK_JS_WRITE_PATH).unwrap();
    let mut hook_js = File::open(NO_EMBED_HOOK_JS_PATH).unwrap();
    std::io::copy(&mut hook_js, &mut file).unwrap();
}

fn append_require_to_file() {
    let mut file = OpenOptions::new()
        .append(true)
        .open(INJECT_JS_PATH)
        .unwrap();
    file.write_all("\nrequire('./hook')".as_ref()).unwrap();
}
