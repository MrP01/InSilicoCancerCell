fn get_log_level() -> log::LevelFilter {
  let log_level_str = std::env::var("LOG_LEVEL").unwrap_or_else(|_| String::from("Info"));
  let log_level = match log_level_str.to_lowercase().as_str() {
    "error" => simplelog::LevelFilter::Error,
    "warn" => simplelog::LevelFilter::Warn,
    "info" => simplelog::LevelFilter::Info,
    "debug" => simplelog::LevelFilter::Debug,
    "trace" => simplelog::LevelFilter::Trace,
    _ => {
      eprintln!("Invalid log level in LOG_LEVEL environment variable. Defaulting to Info.");
      simplelog::LevelFilter::Info
    }
  };
  log_level
}

#[cfg_attr(feature = "pyo3", pyo3::pyfunction)]
pub fn setup_logging() {
  let log_level = get_log_level();
  let mut config_builder = simplelog::ConfigBuilder::new();
  #[cfg(target_arch = "wasm32")]
  config_builder.set_time_level(simplelog::LevelFilter::Off);
  simplelog::CombinedLogger::init(vec![
    simplelog::TermLogger::new(
      log_level,
      config_builder.build(),
      simplelog::TerminalMode::Mixed,
      simplelog::ColorChoice::Auto,
    ),
    // simplelog::WriteLogger::new(
    //   simplelog::LevelFilter::Info,
    //   simplelog::Config::default(),
    //   std::fs::File::create("a549-in-silico.log").unwrap(),
    // ),
  ])
  .unwrap();
}
