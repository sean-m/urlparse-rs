extern crate url;
extern crate clap;

use url::{Url};
use clap::{App, Arg};

fn main() {

    let arg_matches = App::new("urlparse")
        .version("1.0.0")
        .about("Parses a url string and will return parts of it.")
        .author("Sean McArdle")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .value_name("URL")
            .help("The url to parse")
            .takes_value(true))
        .arg(Arg::with_name("scheme")
            .long("scheme")
            .value_name("SCHEME")
            .help("Return scheme from parsed url")
            .takes_value(false))
        .arg(Arg::with_name("username")
            .long("username")
            .value_name("USERNAME")
            .help("Return username, if any, from parsed url")
            .takes_value(false))
        .arg(Arg::with_name("password")
            .long("password")
            .value_name("PW")
            .help("Return password, if any, from parsed url")
            .takes_value(false))
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .value_name("HOST")
            .help("Return host from parsed url")
            .takes_value(false))
        .arg(Arg::with_name("port")
            .long("port")
            .value_name("PORT")
            .help("Return port, if specified, from parsed url")
            .takes_value(false))
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("PATH")
            .help("Return path from parsed url")
            .takes_value(false))
        .arg(Arg::with_name("query")
            .short("q")
            .long("query")
            .value_name("QUERY")
            .help("Return query string, if any, from parsed url")
            .takes_value(false))
        .get_matches();


        let mut url_parts: Vec<&str> = vec![];
        let mut port_str = "".to_string();

        if let Some(url_str) = arg_matches.value_of("url") {
            match Url::parse(url_str) {
                Ok(url) => {
                    if arg_matches.is_present("scheme") {
                        url_parts.push(&url.scheme());
                    }
                    if arg_matches.is_present("username") {
                        url_parts.push(&url.username());
                    }
                    if arg_matches.is_present("host") {
                        if let Some(host_str) = url.host_str() {
                            url_parts.push(host_str);
                        }
                    }
                    if arg_matches.is_present("port") {
                        if let Some(p) =  url.port() {
                            port_str = format!("{}", p).to_string();
                            url_parts.push(&port_str);
                        }
                    }
                    if arg_matches.is_present("path") {
                        url_parts.push(&url.path());
                    }
                    if arg_matches.is_present("query") {
                        if let Some(query) = url.query() {
                            url_parts.push(&query);
                        }
                    }
                    println!("{}", url_parts.join("\n"));
                },
                Err(err) => eprintln!("Error parsing url: {}\n > {}", url_str, err),
            }
        }
}
