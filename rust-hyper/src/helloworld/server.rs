use hyper::{Body, Request, Response, Server};
use hyper::body::HttpBody;
use hyper::service::{make_service_fn, service_fn};

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(req.into_body()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = ([0, 0, 0, 0], 10000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
