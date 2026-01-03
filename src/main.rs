use regex::Regex;
use reqwest::blocking::get;
use std::collections::HashSet;
use std::env;
use url::Url;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("[!] Require: argument <targetURL>");
        std::process::exit(1);

    }

    let base_url = &args[1];
    

    let parsed_base = Url::parse(base_url)
        .expect("[!] Invalid URL");


    let response = get(base_url)
        .expect("[!] Failed to request URL")
        .text()
        .expect("[!] Failed to read response body");

    let re = Regex::new(r"/wp-content/plugins/([^/]+)/")
        .expect("Invalid regex");

    let mut plugins = HashSet::new();
    
    for cap in re.captures_iter(&response) {
        plugins.insert(cap[1].to_string());
    }

    let mut plugins: Vec<String> = plugins.into_iter().collect();
    plugins.sort();

    for plugin in plugins {
        let plugin_path = format!("/wp-content/plugins/{}",plugin);
        let absolute_url = parsed_base
            .join(&plugin_path)
            .expect("Failed to build URL");

        
        println!("{}",&absolute_url);
        println!("  └ Plugin: {}",plugin);
        println!("[+]Try Access: {}/readme.txt",absolute_url);
        println!("");

    }
}
