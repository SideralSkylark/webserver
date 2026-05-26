use webserver::server::listner;

fn main() -> anyhow::Result<()> {
    let result = match listner::listen("1024") {
        Ok(o) => o,
        Err(e) => {
            println!("something went wrong {:?}", e);
        }
    };

    Ok(result)
}
