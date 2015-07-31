#![feature(convert)]

extern crate server;
extern crate shared;
extern crate client;

mod tests;

use client::Client;

fn main() {
    let client = Client::new();

    client.join();
}
