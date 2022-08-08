// website: https://www.fpcomplete.com/blog/http-status-codes-async-rust/
// MUUUUUCh faster than python so far

//install req: 
// sudo apt-get install pkg-config libssl-dev

//building for windows
// rustup target add x86_64-pc-windows-gnu
// rustup toolchain install stable-x86_64-pc-windows-gnu
// cargo build --target x86_64-pc-windows-gnu

//Right now, wiht no threading on recusrive, it goes through a directory in order (pretty clean), may be okay if threading doesn't wanna work
//NOTE: IT keeps searching recursively forever, might be solved by threading by allowing it to move on to another word, but need
//To find a way to stop it if it gets stuck --> line 85 for jank fix

//Need to solve # problem with it thinking that directories are there that don't exist (line49) <<--

// as of now, can not file files due to the recursive func adding a / to each word, should be okay though and not a big deal if it can't
//be implemented

//Addntl note, there are some reapeated requests, I assume due to the // shtuff. maybe filter out url's that have already been tried?
//might solve some issues too

use std::io::BufRead;
use colored::Colorize;
use std::thread;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///Wordlist to crack against
    #[clap(short, long, value_parser)]
    wordlist: String,

    ///URL to target
    #[clap(short, long, value_parser)]
    url: String,
}

fn main() {
    startup_message();
    init_function();
}


fn startup_message() {
    println!("{}", "
=======================================
WebFlinger Rust! A Directory Bruteforcer
=======================================
".red().bold());
}


// We'll return _some_ kind of an error
fn init_function() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Open the file for input
    let file = std::fs::File::open(args.wordlist)?;
    // Make a buffered version so we can read lines
    let buffile = std::io::BufReader::new(file);


    // Create a client so we can make requests
    let client = reqwest::blocking::Client::new();

    for line in buffile.lines() {
        // Error handling on reading the lines in the file
        let line = line?;



        // Make a request and send it, getting a response
        
        let full_url =  format!("http://{}/{}", args.url, line); 

        //println!("{:?}", full_url); //<-- prints full url TROUBLESHOOTING

        //bad character handling
        if full_url.contains("#") {
            println!("{}", "= = = = = = = = = = = = = = = = = = = = ".red());
            println!("Found bad character in '{}', skipping", full_url);
            continue;
        }


        //getting response + sending request at same time
        let resp = client.get(&full_url).send()?;

        //println!("{:?}", line); //-< URL extension (ex: images) TROUBLESHOOTING
        //println!("{:?}", resp); //<- text response  TROUBLESHOOTING

        //prinitng url its trying, find a way to clear screen/the line it prints b4 this so it's somewhat clean
        //println!("Trying: {}", full_url);

        // Print the status code
        //println!("{}, Response Code: {}", full_url, resp.status().as_u16()); //<- printing URL + response
        if resp.status().as_str().contains("40") /* == "404"*/ { //Have to put this here to avoid 404 flood
            continue;
        }
        //else if full_url.contains("#") {
            //continue;
        //}

        else {
            term_output(full_url, resp);

        }

    }
    Ok(())
}

// This was a pain to get right, Doc here: https://users.rust-lang.org/t/convert-box-dyn-error-to-box-dyn-error-send/48856
fn recursive(url: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //println!("RECURSIVE {}", url);
    let args = Args::parse();

    let rec_file = std::fs::File::open(args.wordlist)?;
    let rec_buffile = std::io::BufReader::new(rec_file);

    let rec_client = reqwest::blocking::Client::new();

    let handle = thread::spawn(move || {
        for line in rec_buffile.lines() {
            let line = format!("/{}/",line?);
            if line.contains("///") {
                println!("multiple ///");
            }

            //println!("DEBUG LINE {}", line);
            let full_url =  format!("{}{}", url, line);  
            //println!("URL2 {}", full_url);
            //println!("{:?}", full_url); <-- prints full url TROUBLESHOOTING

            //Bad Character Handling
            if full_url.contains("#") {
                continue;
            }

            //getting response + sending request at same time
            let resp = rec_client.get(&full_url).send()?;
            //println!("{}, Response Code: {}", full_url, resp.status().as_u16());

            if resp.status().as_str().contains("40") { //Have to put this here to avoid 404 flood
                continue;
            }

            else if full_url.contains("///") {
                //break;
                continue;
            }
            else {
                //let temp_full_url = full_url.clone();
                println!("DEBUG: Threaded");
                term_output(full_url, resp);
                //println!("PRE TERM FULL URL {}", temp_full_url);
            }
        }
        Ok(())
    });
    
    handle.join().unwrap()
}

//Global output, everyhting runs through this filter, then either goes back to recursive for a recursive search, or ends
fn term_output(full_url: String, resp: reqwest::blocking::Response) {
    //println!("\x1b[1J");
    //println!("\x1b[5;1H");
    //println!("{}", full_url);
    if resp.status().as_str() == "200"{
        println!("{}", "= = = = = = = = = = = = = = = = = = = = ".blue());
        println!("{}, Response Code: {}", full_url, resp.status().as_u16()); //<- printing URL + response
        println!("{}", "= = = = = = = = = = = = = = = = = = = = ".blue());
        //let temp_full_url = full_url.clone();
        recursive(full_url); //(do an argument to stop recursive, its as easy as commenting out this line)


        //println!("Pre Recursive Call {}", temp_full_url)
    }
    else if resp.status().as_str() == "500"{
        println!("{}, Response Code: {} <-- Internal Server Error!! Check this out", full_url, resp.status().as_u16()); //<- printing URL + response
        println!("{}", "= = = = = = = = = = = = = = = = = = = = ".green().bold());
    }

    else {
        println!("{}, Response Code: {}", full_url, resp.status().as_u16()); //<- printing URL + response
    }
}