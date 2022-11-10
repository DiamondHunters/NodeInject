use std::convert::AsRef;
use std::fs::{File, OpenOptions};
use std::io::Write;
use random_string::generate;


const BYTES: &[u8] = include_bytes!("hooklog.js");
const LICENSE_CHARS: &str = "L23456789ABCDEFGHJKMNPQRSTUVWXYZ";
fn main() {
    generate_license();
    if file_exist("./node"){
        println!("You may have already installed the hook.");
        return;
    }
    if !file_exist("./resources/node_modules.asar") {
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

fn write_js_to_file() {
    let mut file = File::create("./node/raven/hook.js").unwrap();
    file.write_all(BYTES).unwrap();
}
fn append_require_to_file() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("./node/raven/index.js")
        .unwrap();
    file.write_all("\nrequire('./hook')".as_ref()).unwrap();
}
fn generate_license(){
    let mut license = generate(22, LICENSE_CHARS);
    for n in  0..2 {
        let mut o = 0;
        for i in (0..16).step_by(2) {
            o += LICENSE_CHARS.find(&license[n+i..=n+i]).unwrap()
        }
        o %= LICENSE_CHARS.len();
        license += &LICENSE_CHARS[o..=o];
    }
    license.insert(6, '-');
    license.insert(13, '-');
    license.insert(20, '-');
    println!("License for you: {}", license);
}
