use hyper::{Body, Request, Response, Server, Method};
use hyper::service::{service_fn};
use std::fs::File;
use std::io::prelude::*;
use std::convert::Infallible;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let response = Response::new(Body::from("Hello, World!"));
            Ok(response)
        },
        (&Method::GET, "/hello") => {
            let response = Response::new(Body::from("Hello from Rust HTTP Server!"));
            Ok(response)
        }
        (&Method::GET, "/page") => {
            match serve_html("/home/geronimo/Desktop/rust-play/http-server-web/src/index.html") {
                Ok(html_content) => Ok(Response::new(Body::from(html_content))),
                Err(_) => {
                    // Infallible means we can't return an error, so we return a Response instead
                    Ok(Response::builder()
                        .status(404)
                        .body(Body::from("Page not found"))
                        .unwrap())
                    
                }
            }
        },
        
        _ => {
            let response = Response::builder()
                .status(404)
                .body(Body::from("Not Found"))
                .unwrap();
            Ok(response)
        },
    }
}


fn serve_html(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();

    let make_svc = service_fn(handle_request);

    let server = Server::bind(&addr).serve(hyper::service::make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(handle_request))

    }));

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
