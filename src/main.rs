mod randomgenerator;

use std::{collections::HashMap};
use getopts::Options;
use std::{thread, time::Duration, env, io::Read, str::FromStr};
use crabquery::{Document, Element};
use url::{Url};
use std::io::{self, Write};

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
        }
    }
}

fn process_page(url: &String, delay: i32, num: i32) {
    let mut formTargetUrl: String;
    let mut params: HashMap<String, String> = HashMap::new();
    let mut hidden_params: HashMap<String, String> = HashMap::new();
    let mut request_params: HashMap<String, String> = HashMap::new();
    let mut body = String::new();

    // Read page contents
    let mut res = reqwest::blocking::get(url).unwrap();
    res.read_to_string(&mut body);

    let doc = Document::from(body.to_string());
    let forms = doc.select("form");
    let mut inputs: Vec<Element>;
    let mut selects: Vec<Element>;
    let mut formNum: i32 = 0;

    for form in &forms {
        
        formNum = formNum+1;

        // Work out what the form target should be
        match generate_target_url(url.clone(),form.attr("action").unwrap()) {
            Some(target) => {
                formTargetUrl = target;
            }
            _ => { // Catch any problems, and assume that the form is hitting itself
                formTargetUrl = url.clone();
            }
        }
        println!("Form {}/{} - Computed target {}",formNum,forms.len(), formTargetUrl);

        // Get form components
        inputs = form.select("input");
        selects = form.select("select");
        println!("Found {} input fields and {} select fields",inputs.len(),selects.len());

        // Loop through input elements and parse into param arrays
        for element in inputs{
            if element.attr("type").unwrap() == "hidden" {
                match element.attr("name") {
                    Some(_) => {
                        match element.attr("value") {
                            Some(_) => {
                                hidden_params.insert(element.attr("name").unwrap().to_string(), element.attr("value").unwrap().to_string());
                            }
                            None => {
                                hidden_params.insert(element.attr("name").unwrap().to_string(), "".to_string());
                            }
                        }
                    }
                    None => {
                        println!("Ignoring hidden field with no name");
                    }
                }

            } else {

                match element.attr("name") {
                    Some(_) => {
                        match element.attr("value") {
                            Some(_) => {
                                params.insert(element.attr("name").unwrap().to_string(), element.attr("value").unwrap().to_string());
                            }
                            None => {
                                params.insert(element.attr("name").unwrap().to_string(), "".to_string());
                            }
                        }
                    }
                    None => {
                        // This would be for crap like submit buttons
                        params.insert(element.attr("type").unwrap().to_string(), element.attr("type").unwrap().to_string());
                    }
               }
            }
        }
    
        // Hit the form target "num" times
        for hitnum in 0..num {

            // Build up form request, where visibile fields are blank, generate random information
            request_params = HashMap::new();
            for (key, value) in &params {
                if value == "" {
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
                        request_params.insert(key.to_string(), randomgenerator::generate_bollocks(randomgenerator::generate_random()));
                    }
                } else {
                    // There was pre-existing data. Use that as not to arouse suspicion
                    request_params.insert(key.to_string(), value.to_string());
                }
            }

            // Append hidden options
            for (key, value) in &hidden_params {
                request_params.insert(key.to_string(), value.to_string());
            }

            // Do the needful here to send the request
            let client = reqwest::blocking::Client::new();
            let res = client.post(&formTargetUrl)
                .header("User-Agent", &randomgenerator::get_random_useragent())
                .form(&request_params)
                .send();
            match res {
                Ok(res) => print!("\rHit {} - Response: {}\u{001b}[0K", hitnum+1,res.status()),
                Err(err) => print!("\rHit {} - Error: {}\u{001b}[0K", hitnum+1,err)
            }
            io::stdout().flush().expect("flush failed.");

            // Clear the request params ready for the next iteration, and sleep based on predefined args
            request_params.clear();
            thread::sleep(Duration::from_millis(delay as u64));
        } // form attack loop
    } // Form selector
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}


fn generate_target_url(url: String,target : String) -> Option<String>{

    // Generate target URL based on url if target is not already a full URL

    match Url::parse(&target) {
        Ok(_) => {Some(target)}
        Err(_) => {
            match Url::parse(&url) {
                Ok(uc) => {
                    match uc.join(&target) {
                        Ok(retval) => {
                            Some(retval.as_str().to_string())
                        }
                        _ => None
                        
                    }
                }
                Err(_) => None
            }
        }
    }


    

}