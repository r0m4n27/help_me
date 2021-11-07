use std::{fmt::Arguments, io};

use chrono::Local;
use fern::{colors::ColoredLevelConfig, log_file, Dispatch, FormatCallback};
use log::{LevelFilter, Record};

// Add colors for stdout
//
// https://github.com/daboross/fern/issues/45
pub fn setup_logging() -> Result<(), fern::InitError> {
    let log_level = LevelFilter::Debug;

    let file_dispatcher = Dispatch::new()
        .format(create_formatter(false))
        .level(log_level)
        .chain(log_file("help_me.log")?);

    let user_actions_dispatcher = Dispatch::new()
        .format(create_formatter(false))
        .level(log_level)
        .filter(|metadata| metadata.target() == "USER-ACTION")
        .chain(log_file("user_actions.log")?);

    let stdout_dispatcher = Dispatch::new()
        .format(create_formatter(true))
        .level(log_level)
        .chain(io::stdout());

    Dispatch::new()
        .chain(file_dispatcher)
        .chain(stdout_dispatcher)
        .chain(user_actions_dispatcher)
        .apply()?;

    Ok(())
}

fn create_formatter(use_colors: bool) -> impl Fn(FormatCallback, &Arguments, &Record) {
    let colors = ColoredLevelConfig::default();

    move |out: FormatCallback, message: &Arguments, record: &Record| {
        out.finish(format_args!(
            "{}[{}][{}] {}",
            Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            record.target(),
            if use_colors {
                colors.color(record.level()).to_string()
            } else {
                record.level().to_string()
            },
            message
        ))
    }
}
