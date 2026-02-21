#![allow(unused)]

use std::io::StdoutLock;

struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {}", file_path)
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vypc {}", file_path)
    }
}

fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}
fn main() {
    let sol = Solidity {
        version: "2.0".to_string(),
    };

    let vyp = Vyper {
        version: "1.0".to_string(),
    };

    println!("sol: {}", sol.compile("hello.sol"));
    println!("vyp: {}", vyp.compile("hello.py"));
}
