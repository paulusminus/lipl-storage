use hurl::{
    output::write_last_body,
    runner::{run, RunnerOptions, RunnerOptionsBuilder, Value},
    util::{
        logger::{LoggerOptions, LoggerOptionsBuilder},
        term::Stdout,
    },
};
use std::{collections::HashMap, env::var};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

mod constant;

fn variables() -> HashMap<String, Value> {
    let mut variables = HashMap::<String, Value>::new();
    variables.insert(
        constant::PREFIX_NAME.into(),
        Value::String(constant::PREFIX_VALUE.into()),
    );
    variables
}

fn logger_options() -> LoggerOptions {
    LoggerOptionsBuilder::new()
        .color(constant::OUTPUT_COLOR)
        .build()
}

fn user() -> Result<String> {
    let username = var("LIPL_USERNAME")?;
    let password = var("LIPL_PASSWORD")?;
    let user = format!("{}:{}", username, password);
    Ok(user)
}

fn runner_options(user: String) -> RunnerOptions {
    RunnerOptionsBuilder::new().user(Some(user)).build()
}

fn run_script(script: &str, out: &mut Stdout) -> Result<()> {
    user()
        .and_then(|user| {
            run(
                script,
                None,
                &runner_options(user),
                &variables(),
                &logger_options(),
            )
            .map_err(Into::into)
        })
        .and_then(|result| {
            write_last_body(&result, true, true, None, out, true)
                .map_err(|error| format!("{:?} {:?}", error.kind, error.source_info))
                .map_err(Into::into)
        })
}

fn main() -> Result<()> {
    let mut out = Stdout::new(hurl::util::term::WriteMode::Immediate);
    constant::HURL_SCRIPTS
        .into_iter()
        .try_for_each(|script| run_script(script, &mut out))
}
