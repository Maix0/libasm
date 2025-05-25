#![allow(unused)]

mod libasm;

#[cfg(test)]
mod memcmp;
#[cfg(test)]
mod strcmp;
#[cfg(test)]
mod strdup;
#[cfg(test)]
mod strlen;

fn main() {
    println!("USE `cargo test` :)");
}
