use std::io::Error;

use hyper::header::ContentType;
use hyper::method::Method;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::server::{Server, Request,  Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;

macro_rules! content_type {
    ($sub_level:ident) => {
        ContentType(Mime(
            TopLevel::Text,
            SubLevel::$sub_level,
            vec![(Attr::Charset, Value::Utf8)]
        ))
    }
}

macro_rules! url {
    ($href:expr) => { RequestUri::AbsolutePath($href.to_string()) }
}

struct Router {
    ws_url: String,
}

impl Router {
    fn new(ws_url: &str) -> Router {
        Router { ws_url: ws_url.to_string() }
    }

    fn route(&self, url: &str, mut res: Response) -> Result<(), Error> {
        match url {
            "/img/termen.png" => {
                res.headers_mut().set(ContentType::png());
                res.send(include_bytes!("../../client/img/termen.png"))
            },

            "/style.css" => {
                res.headers_mut().set(content_type!(Css));
                res.send(include_bytes!("../../client/style.css"))
            },

            "/audio-context.js" => {
                res.headers_mut().set(content_type!(Javascript));
                res.send(include_bytes!("../../client/audio-context.js"))
            },

            "/main.js" => {
                res.headers_mut().set(content_type!(Javascript));
                res.send(
                    include_str!("../../client/main.js")
                        .replace("{{ws_url}}", &self.ws_url)
                        .as_bytes()
                )
            },

            "/list.js" => {
                res.headers_mut().set(content_type!(Javascript));
                res.send(
                    include_str!("../../client/list.js")
                        .replace("{{ws_url}}", &self.ws_url)
                        .as_bytes()
                )
            },

            "/help" => {
                res.headers_mut().set(ContentType::html());
                res.send(include_bytes!("../../client/help.html"))
            },

            "/list" => {
                res.headers_mut().set(ContentType::html());
                res.send(include_bytes!("../../client/list.html"))
            },

            _ => {
                res.headers_mut().set(ContentType::html());
                res.send(include_bytes!("../../client/index.html"))
            }
        }
    }
}

pub fn serve(http_host: &str, ws_url: &str) {

    let router = Router::new(ws_url);

    let server = Server::http(http_host).unwrap();
    let _guard = server.handle(move |req: Request, mut res: Response| {
        let result = if req.method == Method::Get {
            if let RequestUri::AbsolutePath(url) = req.uri {
                router.route(&url, res)
            } else {
                *res.status_mut() = StatusCode::InternalServerError;
                res.send(b"500 server error")
            }
        } else {
            *res.status_mut() = StatusCode::NotFound;
            res.send(b"404 not found")
        };

        match result {
            Ok(v) => v,
            Err(e) => {
                println!("Error in http server: {}", e);
                return;
            }
        }
    });

    println!("Serving HTTP on {}", &http_host);
}
