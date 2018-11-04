use clap;

pub struct Arguments {
    pub method: String,
    pub url: String,
    pub yaml: Option<String>,
}

pub fn parse_arguments() -> Arguments {
    let matches = clap::App::new("qurl")
        .version("0.1.0")
        .author("Yusuke Nojima <nojima@ynojima.com>")
        .about("A command line HTTP client for RESTful services")
        .arg(
            clap::Arg::with_name("METHOD")
                .help("HTTP method")
                .required(true)
                .index(1),
        ).arg(
            clap::Arg::with_name("URL")
                .help("URL")
                .required(true)
                .index(2),
        ).arg(
            clap::Arg::with_name("YAML")
                .help("YAML translated to JSON and included in HTTP body")
                .required(false)
                .index(3),
        ).get_matches();

    Arguments {
        method: matches.value_of("METHOD").unwrap().to_string(),
        url: matches.value_of("URL").unwrap().to_string(),
        yaml: matches.value_of("YAML").map(|s| s.to_string()),
    }
}
