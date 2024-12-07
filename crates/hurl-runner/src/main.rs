use hurl::{
    output::write_last_body,
    runner::{run, HurlResult, RunnerOptions, RunnerOptionsBuilder, Value, VariableSet},
    util::{
        logger::{LoggerOptions, LoggerOptionsBuilder},
        term::{Stdout, WriteMode},
    },
};
use std::env::var;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

mod constant;

fn variableset() -> Result<VariableSet> {
    let mut variables = VariableSet::new();
    variables
        .insert(
            constant::PREFIX_NAME.into(),
            Value::String(constant::PREFIX_VALUE.into()),
        )
        .map_err(|_| "Cannot insert key value into variableset".to_owned())?;
    Ok(variables)
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

fn run_with_options(
    script: &str,
    variables: &VariableSet,
    runner_options: &RunnerOptions,
    logger_options: &LoggerOptions,
) -> Result<HurlResult> {
    run(script, None, runner_options, variables, logger_options).map_err(Into::into)
}

fn handle_output(out: &mut Stdout) -> impl FnMut(HurlResult) -> Result<()> + '_ {
    move |result| {
        write_last_body(&result, true, true, None, out, true)
            .map_err(|error| format!("{:?} {:?}", error.kind, error.source_info).into())
    }
}

fn run_script<'a>(
    out: &'a mut Stdout,
    variables: &'a VariableSet,
    runner_options: &'a RunnerOptions,
    logger_options: &'a LoggerOptions,
) -> impl FnMut(&str) -> Result<()> + 'a {
    move |script| {
        run_with_options(script, variables, runner_options, logger_options)
            .and_then(handle_output(out))
    }
}

fn main() -> Result<()> {
    let out = &mut Stdout::new(WriteMode::Immediate);
    let user = user()?;
    let runner_options = RunnerOptionsBuilder::new().user(Some(user)).build();
    let variables = variableset()?;
    let logger_options = logger_options();

    constant::HURL_SCRIPTS.into_iter().try_for_each(run_script(
        out,
        &variables,
        &runner_options,
        &logger_options,
    ))
}
