use tokio::fs::File;
use warp::Filter;
use warp::Rejection;
use warp::Reply;

// These are the corrected hyper imports.
use hyper::Body;
use hyper::Response;

#[tokio::main]
async fn main() {
    let mp4 = warp::path!("video" / String)
        .and_then(video_handler); // Using and_then to directly handle the async function without further mapping.

    warp::serve(mp4).run(([127, 0, 0, 1], 3030)).await;
}

async fn video_handler(param: String) -> Result<impl Reply, Rejection> {
    let file_path = format!("./video/{}", param);
    match File::open(&file_path).await {
        Ok(file) => {
            let stream = tokio_util::io::ReaderStream::new(file); // Ensure tokio_util is in your Cargo.toml dependencies.
            let body = Body::wrap_stream(stream);

            let response = Response::builder()
                .header("Content-Type", "video/mp4")
                .body(body)
                .unwrap();

            Ok(response)
        }
        Err(_) => {
            // Handle the error gracefully, e.g., by returning a 404 response.
            let response = Response::builder()
                .status(404)
                .body(Body::from("File not found")) // Body::from(...) is used to convert &str to Body.
                .unwrap();

            Ok(response)
        }
    }
}
