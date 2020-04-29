#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate log4rs;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        LogConfig(log4rs::config::Errors);
        SetLogger(log::SetLoggerError);
    }
}

fn main() -> Result<()> {
	let logfile = FileAppender::builder()
		.encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
		.build("log/output.log")?;

	let config = Config::builder()
		.appender(Appender::builder().build("logfile", Box::new(logfile)))
		.build(Root::builder()
			.appender("logfile")
			.build(LevelFilter::Info))?;

	log4rs::init_config(config)?;

	info!("Hello, world!");

	Ok(())
}
