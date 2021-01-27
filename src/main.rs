#[macro_use]
extern crate lazy_static;
mod config;
mod hook;
mod input;
mod manager;
mod register;
mod xim;
use std::thread;
mod window;

fn main() {
    manager::Manager::start();
}
