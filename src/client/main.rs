use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("zoomies-client")
        .version("1.0")
        .arg(
            Arg::with_name("url")
                .long("url")
                .takes_value(true)
                .require_equals(true)
                .required(true),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();

    let client = reqwest::blocking::Client::new();
    client
        .get("http://localhost:8000/meeting")
        .query(&[("url", url.to_string())])
        .send()
        .unwrap();

    Ok(())
}
