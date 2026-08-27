use bridge_app::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bridge_app::init_tracing();
    run("0.0.0.0:3000").await
}
