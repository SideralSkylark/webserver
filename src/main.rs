use webserver::server::listner;

fn main() -> anyhow::Result<()> {
    let result = match listner::listen() {
        Ok(o) => o,
        Err(e) => {
            println!("something went wrong {:?}", e);
        }
    };

    Ok(result)
}
