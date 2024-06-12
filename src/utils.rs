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

pub trait Roundable {
  fn round(&self) -> Self;
}

impl<const D: usize> Roundable for nalgebra::SVector<f64, D> {
  fn round(&self) -> Self {
    nalgebra::SVector::<f64, D>::from_vec(self.iter().cloned().map(|x| x.round()).collect::<Vec<f64>>())
  }
}

pub trait Cappable {
  fn cap_all_values_to(&self, val: f64) -> Self;
  fn lower_cap_all_values_to(&self, val: f64) -> Self;
}

impl<const D: usize> Cappable for nalgebra::SVector<f64, D> {
  fn cap_all_values_to(&self, val: f64) -> Self {
    nalgebra::SVector::<f64, D>::from_vec(self.iter().cloned().map(|x| x.min(val)).collect::<Vec<f64>>())
  }
  fn lower_cap_all_values_to(&self, val: f64) -> Self {
    nalgebra::SVector::<f64, D>::from_vec(self.iter().cloned().map(|x| x.max(val)).collect::<Vec<f64>>())
  }
}

// Creates a Gaussian kernel with the given size and standard deviation
pub fn gaussian_kernel(size: usize, sigma: f64) -> Vec<f64> {
  let mut kernel = vec![0.0; size];
  let half_size = size / 2;
  for i in 0..size {
    let x = i as f64 - half_size as f64;
    kernel[i] = (1.0 / (sigma * (2.0 * std::f64::consts::PI).sqrt())) * (-0.5 * (x.powi(2) / sigma.powi(2))).exp();
  }
  kernel
}
