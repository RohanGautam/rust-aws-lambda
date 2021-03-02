#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;
use lambda::{handler_fn, Context};
mod call_endpoint;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    data: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    quote: String,
}
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    let func = handler_fn(my_handler);
    lambda::run(func).await?;
    Ok(())
}

/// you can invoke the lambda with a JSON payload, which is parsed using the CustomEvent struct.
async fn my_handler(e: CustomEvent, c: Context) -> Result<CustomOutput, Error> {
    if e.data != "" {
        error!("Got {} from {}", e.data, c.invoked_function_arn);
    } else {
        error!("Got no data passed to the lambda")
    }
    let quote: String = match call_endpoint::get_inspiring_quote().await {
        Ok(q) => q,
        Err(_) => String::from("Error retrieving quote"),
    };

    Ok(CustomOutput { quote: quote })
}
