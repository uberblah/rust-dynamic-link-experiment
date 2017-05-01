extern crate libloading as lib;

use lib::{Library, Symbol};

fn main() {
    println!("loading library");
    let a = Library::new("./liba.dylib").unwrap();

    println!("loading symbols");
    let f: Symbol<extern fn()>;
    let g: Symbol<extern fn() -> Box<Iterator<Item=usize>>>;
    unsafe {
        f = a.get(b"heythere").unwrap();
        g = a.get(b"primes").unwrap();
    }

    println!("calling functions");
    f();
    let x = 20;
    println!("unrolling {} primes", x);
    let _: Vec<(usize, usize)> = g().take(x).enumerate()
        .inspect(|p| println!("{}: {}", p.0 + 1, p.1)).collect();
}
