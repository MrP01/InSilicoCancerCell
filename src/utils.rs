use crate::{cell::MembraneCurrentThroughput, patchclampdata::PatchClampData};

pub fn setup_logging() {
  simplelog::CombinedLogger::init(vec![
    simplelog::TermLogger::new(
      simplelog::LevelFilter::Info,
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

pub fn evaluate_match(measurements: PatchClampData, simulation: MembraneCurrentThroughput) {
  let rows = measurements.current.len();
  let error = (simulation.as_dvec().rows_range(0..rows) - measurements.current).norm_squared();
  log::info!("Simulation match with measurements: {:.3}", error);
}
