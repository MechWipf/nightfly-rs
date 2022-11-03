//! `cargo run --example blocking --features=blocking`
#![deny(warnings)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Some simple CLI args requirements...
    let url = match std::env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("No CLI URL provided, using default.");
            "http://eu.httpbin.org/cookies".into()
        }
    };

    eprintln!("Fetching {:?}...", url);

    // nightfly::blocking::get() is a convenience function.
    //
    // In most cases, you should create/build a nightfly::Client and reuse
    // it for all requests.
    let res = nightfly::get(url)?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    // copy the response body directly to stdout
    println!("Body: {:?}", res.body());

    Ok(())
}
