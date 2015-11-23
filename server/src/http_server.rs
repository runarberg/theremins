use hyper::header::{ContentType};
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
    ws_host: String,
}

impl Router {
    fn new(ws_host: &str) -> Router {
        Router { ws_host: ws_host.to_string() }
    }

    fn route(&self, url: &str, mut res: Response) {
        match url {
            "/style.css" => {
                res.headers_mut().set(content_type!(Css));
                res.send(include_bytes!("../../client/style.css")).unwrap();
            },

            "/audio-context.js" => {
                res.headers_mut().set(content_type!(Javascript));
                res.send(include_bytes!("../../client/audio-context.js")).unwrap();
            },

            "/main.js" => {
                res.headers_mut().set(content_type!(Javascript));
                res.send(
                    include_str!("../../client/main.js")
                        .replace("{{ws_host}}", self.ws_host.as_str())
                        .as_bytes()
                ).unwrap();
            },

            "/list.js" => {
                res.headers_mut().set(content_type!(Javascript));
                res.send(
                    include_str!("../../client/list.js")
                        .replace("{{ws_host}}", self.ws_host.as_str())
                        .as_bytes()
                ).unwrap();
            },

            "/help" => {
                res.headers_mut().set(ContentType::html());
                res.send(include_bytes!("../../client/help.html")).unwrap();
            },

            "/list" => {
                res.headers_mut().set(ContentType::html());
                res.send(include_bytes!("../../client/list.html")).unwrap();
            },

            _ => {
                res.headers_mut().set(ContentType::html());
                res.send(include_bytes!("../../client/index.html")).unwrap();
            }
        }
    }
}

pub fn serve(http_host: &str, ws_host: &str) {

    let router = Router::new(ws_host);

    let server = Server::http(http_host).unwrap();
    let _guard = server.handle(move |req: Request, mut res: Response| {
        if req.method == Method::Get {
            if let RequestUri::AbsolutePath(url) = req.uri {
                router.route(url.as_str(), res)
            } else {
                *res.status_mut() = StatusCode::InternalServerError;
                let _ = res.send(b"500 server error");
            }
        } else {
            *res.status_mut() = StatusCode::NotFound;
            let _ = res.send(b"404 not found");
        }
    });

    println!("Serving HTTP on {}", &http_host);
}
