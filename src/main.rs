use webserver::server::listner;

fn main() -> anyhow::Result<()> {
    match listner::listen("1024") {
        Ok(o) => o,
        Err(e) => {
            println!("something went wrong {:?}", e);
        }
    };

    Ok(())
}
