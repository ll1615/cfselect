pub mod configure;
fn main() -> anyhow::Result<()> {
    let ac = configure::AppConfig::read();
    println!("{:?}", ac);

    Ok(())
}

