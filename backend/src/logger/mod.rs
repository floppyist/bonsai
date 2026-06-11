use std::fs::File;
use std::io;

use fern::Dispatch;
use fern::InitError;
use fern::colors::Color;
use fern::colors::ColoredLevelConfig;
use log::LevelFilter;

use crate::config::BonsaiOptions;

pub fn setup_logging(options: &BonsaiOptions) -> Result<(), InitError> {
    let log_level = match options.logging.level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let colors = ColoredLevelConfig::new()
        .info(Color::Blue)
        .debug(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    let mut main_dispatch = Dispatch::new().level(log_level);

    if options.logging.console {
        let console_dispatch = Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{}]: {}",
                    colors.color(record.level()),
                    message,
                ))
            })
            .chain(io::stdout());

        main_dispatch = main_dispatch.chain(console_dispatch);
    }

    if options.logging.file {
        let log_file_path = format!("{}{}", options.logging.path, "fury.log");

        let file_dispatch = Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!("[{}]: {}", record.level(), message,))
            })
            .chain(File::create(log_file_path)?);

        main_dispatch = main_dispatch.chain(file_dispatch);
    }

    main_dispatch.apply()?;

    Ok(())
}
