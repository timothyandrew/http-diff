use clap::{App, Arg};
use reqwest::Error;
use tokio::time;

async fn get_url(url: &str) -> Result<String, Error> {
    reqwest::get(url).await?.text().await
}

#[tokio::main]
async fn main() {
    let app = App::new("http-diff")
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .takes_value(true)
                .default_value("5000")
                .help("Interval between requests in ms"),
        )
        .arg(Arg::with_name("url").required(true));

    let matches = app.get_matches();

    let interval = matches.value_of("interval").unwrap();
    let interval = interval
        .parse::<u64>()
        .expect("--interval needs to be an integer.");

    let url = matches.value_of("url").unwrap();

    let initial = get_url(url).await.expect("Download failed");
    let mut counter = 0;
    println!("Initial value is {}", initial.trim());

    loop {
        time::delay_for(time::Duration::from_millis(interval)).await;
        counter += 1;


        let response = get_url(url).await.expect("Download failed");

        if response == initial {
            eprintln!("After {}ms, there's been no change.", interval * counter);
        } else {
            println!("Response changed to {} from {}; stopping!", response.trim(), initial.trim());
            break;
        }
    };
}
