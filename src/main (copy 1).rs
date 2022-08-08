// website: https://www.fpcomplete.com/blog/http-status-codes-async-rust/
// MUUUUUCh faster than python so far

//Right now, wiht no threading on recusrive, it goes through a directory in order (pretty clean), may be okay if threading doesn't wanna work
//NOTE: IT keeps searching recursively forever, might be solved by threading by allowing it to move on to another word, but need
//To find a way to stop it if it gets stuck --> line 85 for jank fix

//Need to solve # problem with it thinking that directories are there that don't exist (line49)

use std::io::BufRead;
use colored::Colorize;

// We'll return _some_ kind of an error
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the file for input
    let file = std::fs::File::open("rockyou.txt")?;
    // Make a buffered version so we can read lines
    let buffile = std::io::BufReader::new(file);


    // Create a client so we can make requests
    let client = reqwest::blocking::Client::new();

    for line in buffile.lines() {
        // Error handling on reading the lines in the file
        let line = line?;



        // Make a request and send it, getting a response
        
        let full_url =  format!("{}{}", "http://127.0.0.1/", line); 

        //println!("{:?}", full_url); //<-- prints full url TROUBLESHOOTING



        //getting response + sending request at same time
        let resp = client.get(&full_url).send()?;

        //println!("{:?}", line); //-< URL extension (ex: images) TROUBLESHOOTING
        //println!("{:?}", resp); //<- text response  TROUBLESHOOTING

        // Print the status code
        //println!("{}, Response Code: {}", full_url, resp.status().as_u16()); //<- printing URL + response
        if resp.status().as_str() == "404" { //Have to put this here to avoid 404 flood
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

fn recursive(url: String) -> Result<(), Box<dyn std::error::Error>> {
    //println!("RECURSIVE {}", url);

    let rec_file = std::fs::File::open("rockyou.txt")?;
    let rec_buffile = std::io::BufReader::new(rec_file);

    let rec_client = reqwest::blocking::Client::new();

    for line in rec_buffile.lines() {
        let line = line?;
        
        let full_url =  format!("{}{}/", url, line); 
        //println!("URL2 {}", full_url);
        //println!("{:?}", full_url); <-- prints full url TROUBLESHOOTING

        //getting response + sending request at same time
        let resp = rec_client.get(&full_url).send()?;
        //println!("{}, Response Code: {}", full_url, resp.status().as_u16());

        if resp.status().as_str() == "404" { //Have to put this here to avoid 404 flood
            continue;
        }

        else if full_url.contains("///") {
            continue;
        }
        else {
            //let temp_full_url = full_url.clone();
            term_output(full_url, resp);
            //println!("PRE TERM FULL URL {}", temp_full_url);
        }
    }
    Ok(())
}

//Global output, everyhting runs through this filter, then either goes back to recursive for a recursive search, or ends
fn term_output(full_url: String, resp: reqwest::blocking::Response) {

    if resp.status().as_str() == "200"{
        
        println!("{}, Response Code: {}", full_url, resp.status().as_u16()); //<- printing URL + response
        println!("{}", "= = = = = = = = = = = = = = = = = = = = ".blue());
        //let temp_full_url = full_url.clone();

        recursive(full_url);

        //println!("Pre Recursive Call {}", temp_full_url)
    }
    else if resp.status().as_str() == "500"{
        println!("{}, Response Code: {} <-- Internal Server Error!! Check this out", full_url, resp.status().as_u16()); //<- printing URL + response
        println!("{}", "= = = = = = = = = = = = = = = = = = = = ".red());
    }

    else {
        println!("{}, Response Code: {}", full_url, resp.status().as_u16()); //<- printing URL + response
    }
}