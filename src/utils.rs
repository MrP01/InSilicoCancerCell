pub fn setup_logging() {
  simplelog::CombinedLogger::init(vec![
    simplelog::TermLogger::new(
      simplelog::LevelFilter::Debug,
      simplelog::Config::default(),
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
