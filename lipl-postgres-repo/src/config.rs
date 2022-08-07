use clap::{crate_authors, crate_name, crate_version, App, Arg};

const CONNECTION: &str = "connection";
const USER: &str = "user";
const DATABASE: &str = "database";
const SOURCE_DIRECTORY: &str = "source-directory";

pub fn clap() -> (String, String) {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name(CONNECTION)
            .short("c")
            .long(CONNECTION)
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name(USER)
            .short("u")
            .long(USER)
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name(SOURCE_DIRECTORY)
            .short("s")
            .long(SOURCE_DIRECTORY)
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name(DATABASE)
            .index(1)
            .required(true)
            .takes_value(true)
        )
        .get_matches();

    (
        format!(
            "host={} user={} dbname={}",
            matches.value_of(CONNECTION).unwrap(),
            matches.value_of(USER).unwrap(),
            matches.value_of(DATABASE).unwrap(),
        ),
        matches.value_of(SOURCE_DIRECTORY).unwrap().to_string(),
    )
}
