mod randomgenerator;

use std::collections::HashMap;
use getopts::Options;
use std::env;
use std::io::Read;
use scraper::{Html, Selector};
use std::str::FromStr;
use reqwest::header::UserAgent;


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

    let num: Option<String> = matches.opt_str("n");

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
            process_page(url.unwrap(), delay, num);
        }
        None => {
            print_usage(&program, opts);
            process_page("http://www.andydixon.com/form.html".to_string(), delay, num);
        }
    }
}

fn process_page(url: String, delay: i32, num: i32) {
    let mut res = reqwest::blocking::get(url).unwrap();
    let mut body = String::new();
    let mut loop_count: i32 = 0;
    res.read_to_string(&mut body);
    let fragment = Html::parse_fragment(&body);
    let form_selector = Selector::parse("form").unwrap();
    let input_selector = Selector::parse("input").unwrap();
    let select_selector = Selector::parse("select").unwrap();
    let option_selector = Selector::parse("option").unwrap();

    for form in fragment.select(&form_selector) {
        let mut params = HashMap::new();

        println!("Hitting target {}", form.value().attr("action").unwrap());

        for element in form.select(&input_selector) {

            // If the form element is hidden, this may contain some verification shiz, so lets not mangle it!
            if element.value().attr("type").unwrap() == "hidden" {
                params.insert(element.value().attr("name").unwrap(), element.value().attr("value").unwrap());
            } else if element.value().attr("name").unwrap() == "user" ||
                element.value().attr("name").unwrap() == "username" ||
                element.value().attr("name").unwrap() == "user_name" ||
                element.value().attr("name").unwrap() == "uname" ||
                element.value().attr("name").unwrap() == "email" ||
                element.value().attr("name").unwrap() == "em" ||
                element.value().attr("name").unwrap() == "e" ||
                element.value().attr("name").unwrap() == "u" ||
                element.value().attr("name").unwrap() == "login" {
                params.insert(element.value().attr("name").unwrap(), &randomgenerator::generate_email());
            } else if element.value().attr("name").unwrap() == "pass" ||
                element.value().attr("name").unwrap() == "pword" ||
                element.value().attr("name").unwrap() == "pw" {
                params.insert(element.value().attr("name").unwrap(), &randomgenerator::generate_bollocks(8));
            } else if element.value().attr("name").unwrap() == "name" {
                params.insert(element.value().attr("name").unwrap(), &randomgenerator::generate_name());
            } else if element.value().attr("name").unwrap() == "name1" ||
                element.value().attr("name").unwrap() == "firstname" ||
                element.value().attr("name").unwrap() == "fname" ||
                element.value().attr("name").unwrap() == "first_name"
            {
                params.insert(element.value().attr("name").unwrap(), &randomgenerator::generate_firstname());
            } else if element.value().attr("name").unwrap() == "name2" ||
                element.value().attr("name").unwrap() == "lastname" ||
                element.value().attr("name").unwrap() == "lname" ||
                element.value().attr("name").unwrap() == "last_name" ||
                element.value().attr("name").unwrap() == "surname"
            {
                params.insert(element.value().attr("name").unwrap(), &randomgenerator::generate_lastname());
            } else {
                params.insert(element.value().attr("name").unwrap(), &randomgenerator::generate_bolllocks());
            }
        }
        for element in form.select(&select_selector) {
            println!("{} is a select with the following options:", element.value().attr("name").unwrap());
            for option in element.select(&option_selector) {
                print!("{}\t", option.value().attr("value").unwrap());
            }
            println!();
        }

        /**
        Do the needful here to send the request
        **/

        let client = reqwest::Client::new().unwrap();
        client.post("http://httpbin.org")
            .header(UserAgent(&randomgenerator::get_random_useragent()))
            .form(&params)
            .send();
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}