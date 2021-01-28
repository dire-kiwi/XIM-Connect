#[macro_use]
extern crate lazy_static;
mod input;
mod xim;
mod manager;
mod hook;
mod register;
mod config;
use std::thread;
mod window;

fn main() {
    manager::Manager::start();
}