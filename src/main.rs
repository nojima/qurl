mod arguments;
mod yaml_to_json;

extern crate clap;
extern crate hyper;
extern crate yaml_rust;

use arguments::*;
use hyper::rt::{self, Future, Stream};
use std::str::from_utf8;
use yaml_to_json::*;

type Client = hyper::Client<hyper::client::HttpConnector>;
type Request = hyper::Request<hyper::Body>;

fn create_http_client() -> Client {
    let mut builder = hyper::Client::builder();
    builder.keep_alive(false);
    builder.build_http()
}

fn build_request(args: &Arguments) -> hyper::http::Result<Request> {
    let mut builder = hyper::Request::builder();
    builder.uri(&args.url);
    builder.method(&args.method as &str);
    builder.header("User-Agent", "qurl/0.1.0");
    if args.yaml.is_some() {
        builder.header("Content-Type", "application/json; charset=utf-8");
    }
    let body = match args.yaml {
        None => hyper::Body::empty(),
        Some(ref y) => yaml_string_to_json_string(y).into(),
    };
    builder.body(body)
}

fn send_request(client: &Client, request: Request) -> impl Future<Item = (), Error = ()> {
    client
        .request(request)
        .and_then(|res| {
            println!("{:?} {}", res.version(), res.status());
            for (key, value) in res.headers() {
                let value_str = from_utf8(value.as_bytes()).unwrap();
                println!("{}: {}", key, value_str);
            }
            println!("");
            res.into_body().concat2()
        }).and_then(|body| {
            let s = ::std::str::from_utf8(&body).expect("Invalid UTF-8");
            println!("{}", s);
            Ok(())
        }).map_err(|err| {
            println!("Error: {}", err);
        })
}

fn main() {
    let args = parse_arguments();

    let client = create_http_client();
    let request = build_request(&args).unwrap();
    let future = send_request(&client, request);
    rt::run(future);
}
