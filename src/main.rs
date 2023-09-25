use std::convert::Infallible;
use tokio::fs::File;
use warp::http::Response;
use warp::hyper::Body;
use warp::Filter;

#[tokio::main]
async fn main() {
    let mp4 = warp::path("video").and(warp::path("meow-meow-meow.mp4")).and_then(video_handler);

    warp::serve(mp4).run(([127, 0, 0, 1], 3030)).await;
}

async fn video_handler() -> Result<impl warp::Reply, Infallible> {
    let file = File::open("./video/meow-meow-meow.mp4").await.unwrap();
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::wrap_stream(stream);

    Ok(Response::builder().header("Content-Type", "video/mp4").body(body).unwrap())
}
