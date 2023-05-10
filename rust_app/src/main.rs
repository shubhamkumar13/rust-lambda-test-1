use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as Json;

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    command: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    status_code: i32,
    body: String,
}

impl Response {
    fn new(status_code: i32, body: String) -> Self {
        Self { status_code, body }
    }
}

fn generate_valid_response<'a>(input: &'a str) -> Result<Json, Error> {
    let request = serde_json::from_str::<Request>(input)?;
    let response = Response::new(200, request.command);

    Ok(json!({
        "isBase64Encoded" : false,
        "statusCode" : 200,
        "headers" : { },
        "body" : format!("{:#?}", response)
    }))
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Json>) -> Result<Json, Error> {
    // extract body from the event
    let body = event.payload["body"]
        .as_str()
        .ok_or("Request Body is empty")?;

    // this takes the body and generates a response
    generate_valid_response(body)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
