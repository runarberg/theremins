use hyper::header::{ContentType};
use hyper::method::Method;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::server::{Server, Request,  Response};
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

fn handle(req: Request, res: Response) {
    if req.method == Method::Get {
        handle_get(req, res)
    }
}

fn handle_get(req: Request, mut res: Response) {
    if let RequestUri::AbsolutePath(url) = req.uri {
        match url.as_str() {
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
                res.send(include_bytes!("../../client/main.js")).unwrap();
            },

            "/list.js" => {
                res.headers_mut().set(content_type!(Javascript));
                res.send(include_bytes!("../../client/list.js")).unwrap();
            },

            "/help" => {
                res.headers_mut().set(ContentType::html());
                res.send(b"<h1>About!</h1>").unwrap();
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

pub fn serve(host: &str) {
    Server::http(host).unwrap()
        .handle(handle).unwrap();
}
