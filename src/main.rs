use lambda_http::{
    handler,
    lambda::{self, Context},
    IntoResponse, Request,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    lambda::run(handler(func)).await?;
    Ok(())
}

async fn func(_event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok("Hello World!")
}
