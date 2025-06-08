use logsty_rs::{Logger, LogLevel};

fn main() {
    let logger = Logger::new()
        .with_module_name("WEB_SERVER");

    let logger_2 = Logger::new()
        .with_module_name("CLIENT")
        .with_level(LogLevel::FATAL);

    logger.debug("Logger initialized!");
    logger.info("<3");
    logger_2.warn("Cannot find .env file");
    logger.error("System crushed (joke)");
}
