use crate::globals::PROGRAM_NAME;

pub fn setup_logger() -> Result<(), fern::InitError> {
    let level = log::LevelFilter::Info;
    let logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                PROGRAM_NAME,
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stderr());

    logger.apply()?;
    Ok(())
}
