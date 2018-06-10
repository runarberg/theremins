use futures::future;
use http;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::rt::{Future, run};
use hyper::service::service_fn;
use std::vec::Vec;

static FAVICON_ICO: &[u8] = include_bytes!("../client/favicon.ico");
static TERMEN_PNG: &[u8] = include_bytes!("../client/img/termen.png");

type BoxFut = Box<Future<Item=Response<Body>, Error=http::Error> + Send>;

struct App {
    list_js: Vec<u8>,
    main_js: Vec<u8>,
}

impl App {
    fn new(ws_url: &str) -> App {
        App {
            list_js: include_str!("../client/list.js")
                .replace("{{ws_url}}", ws_url)
                .into(),

            main_js: include_str!("../client/main.js")
                .replace("{{ws_url}}", ws_url)
                .into(),
        }
    }

    fn handle(&self, req: Request<Body>) -> BoxFut {
        let response = match (req.method(), req.uri().path()) {
            (&Method::GET, "/favicon.ico") => Response::builder()
                .header("Content-Type", "image/vnd.microsoft.icon")
                .body(Body::from(FAVICON_ICO)),

            (&Method::GET, "/robots.txt") => Response::builder()
                .header("Content-Type", "text/plain")
                .body(Body::from(include_str!("../client/robots.txt"))),

            (&Method::GET, "/img/termen.png") => Response::builder()
                .header("Content-Type", "image/png")
                .body(Body::from(TERMEN_PNG)),

            (&Method::GET, "/img/clouds.svg") => Response::builder()
                .header("Content-Type", "image/svg+xml")
                .body(Body::from(include_str!("../client/img/clouds.svg"))),

            (&Method::GET, "/style.css") => Response::builder()
                .header("Content-Type", "text/css")
                .body(Body::from(include_str!("../client/style.css"))),

            (&Method::GET, "/audio-context.js") => Response::builder()
                .header("Content-Type", "text/javascript")
                .body(Body::from(include_str!("../client/audio-context.js"))),

            (&Method::GET, "/main.js") => Response::builder()
                .header("Content-Type", "text/javascript")
                .body(Body::from(self.main_js.clone())),

            (&Method::GET, "/list.js") => Response::builder()
                .header("Content-Type", "text/javascript")
                .body(Body::from(self.list_js.clone())),

            (&Method::GET, "/help") => Response::builder()
                .header("Content-Type", "text/html")
                .body(Body::from(include_str!("../client/help.html"))),

            (&Method::GET, "/list") => Response::builder()
                .header("Content-Type", "text/html")
                .body(Body::from(include_str!("../client/list.html"))),

            (&Method::GET, _) => Response::builder()
                .header("Content-Type", "text/html")
                .body(Body::from(include_str!("../client/index.html"))),

            _ => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 not found")),

        };

        Box::new(future::result(response))
    }
}

pub fn serve(port: u16, ws_url: String) {
    let addr = ([0, 0, 0, 0], port).into();
    let server = Server::bind(&addr)
        .serve(move || {
            let app = App::new(&ws_url);

            service_fn(move |req| app.handle(req))
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Serving HTTP on {}", &addr);

    run(server)
}
