//! Logging facilities

use std::{env, path::Path};

use clap::ArgMatches;
use log::LevelFilter;
use log4rs::{
    append::console::{ConsoleAppender, Target},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};

/// Initialize logger ([log4rs](https://crates.io/crates/log4rs)) from yaml configuration file
pub fn init_with_file<P>(path: P)
where
    P: AsRef<Path>,
{
    log4rs::init_file(path, Default::default()).expect("init logging with file");
}

/// Initialize logger with default configuration
pub fn init_with_config(bin_name: &str, matches: &ArgMatches) {
    let mut debug_level = matches.occurrences_of("VERBOSE");
    if debug_level == 0 {
        // Override by SS_LOG_VERBOSE_LEVEL
        if let Ok(verbose_level) = env::var("SS_LOG_VERBOSE_LEVEL") {
            if let Ok(verbose_level) = verbose_level.parse::<u64>() {
                debug_level = verbose_level;
            }
        }
    }

    let mut without_time = matches.is_present("LOG_WITHOUT_TIME");
    if !without_time {
        if let Ok(log_without_time) = env::var("SS_LOG_WITHOUT_TIME") {
            if let Ok(log_without_time) = log_without_time.parse::<u32>() {
                without_time = log_without_time != 0;
            }
        }
    }

    let mut pattern = String::new();
    if !without_time {
        pattern += "{d} ";
    }
    pattern += "{h({l}):<5} ";
    if debug_level >= 1 {
        pattern += "[{P}:{I}] [{M}] ";
    }
    pattern += "{m}{n}";

    let logging_builder = Config::builder().appender(
        Appender::builder().build(
            "console",
            Box::new(
                ConsoleAppender::builder()
                    .encoder(Box::new(PatternEncoder::new(&pattern)))
                    .target(Target::Stderr)
                    .build(),
            ),
        ),
    );

    let config = match debug_level {
        0 => logging_builder
            .logger(Logger::builder().build(bin_name, LevelFilter::Info))
            .logger(Logger::builder().build("shadowsocks_rust", LevelFilter::Info))
            .logger(Logger::builder().build("shadowsocks", LevelFilter::Info))
            .logger(Logger::builder().build("shadowsocks_service", LevelFilter::Info))
            .build(Root::builder().appender("console").build(LevelFilter::Off)),
        1 => logging_builder
            .logger(Logger::builder().build(bin_name, LevelFilter::Debug))
            .logger(Logger::builder().build("shadowsocks_rust", LevelFilter::Debug))
            .logger(Logger::builder().build("shadowsocks", LevelFilter::Debug))
            .logger(Logger::builder().build("shadowsocks_service", LevelFilter::Debug))
            .build(Root::builder().appender("console").build(LevelFilter::Off)),
        2 => logging_builder
            .logger(Logger::builder().build(bin_name, LevelFilter::Trace))
            .logger(Logger::builder().build("shadowsocks_rust", LevelFilter::Trace))
            .logger(Logger::builder().build("shadowsocks", LevelFilter::Trace))
            .logger(Logger::builder().build("shadowsocks_service", LevelFilter::Trace))
            .build(Root::builder().appender("console").build(LevelFilter::Off)),
        3 => logging_builder
            .logger(Logger::builder().build(bin_name, LevelFilter::Trace))
            .logger(Logger::builder().build("shadowsocks_rust", LevelFilter::Trace))
            .logger(Logger::builder().build("shadowsocks", LevelFilter::Trace))
            .logger(Logger::builder().build("shadowsocks_service", LevelFilter::Trace))
            .build(Root::builder().appender("console").build(LevelFilter::Debug)),
        _ => logging_builder.build(Root::builder().appender("console").build(LevelFilter::Trace)),
    }
    .expect("logging");

    log4rs::init_config(config).expect("logging");
}
