use hyper::{Body, Client, Request, Response, Server, Uri,service::service_fn,rt::run};
use futures::{compat::Future01CompatExt,future::{FutureExt, TryFutureExt}};
use std::net::SocketAddr;

fn main() {
    println!("Hello, world!");
}
