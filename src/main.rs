#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate log;

mod rocket_loader;
mod uploader;

fn main() {
    rocket_loader::fly()
}