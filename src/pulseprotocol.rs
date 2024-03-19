pub struct PulseProtocolStep {
  pub label: String,
  pub voltage: f64,
  pub duration: f64,
}
pub type PulseProtocol = impl Iterator<Item = PulseProtocolStep>;

pub trait ProtocolGenerator {
  fn default() -> Self;
}

impl ProtocolGenerator for PulseProtocol {
  fn default() -> Self {
    std::iter::from_coroutine(|| {
      let mut v_test = -0.04;
      while v_test <= 0.04 {
        yield PulseProtocolStep {
          label: String::from("hold"),
          voltage: -100e-3,
          duration: 100e-3,
        };
        yield PulseProtocolStep {
          label: String::from("initial"),
          voltage: -80e-3,
          duration: 115.6e-3,
        };
        yield PulseProtocolStep {
          label: String::from("test"),
          voltage: v_test,
          duration: 800e-3,
        };
        yield PulseProtocolStep {
          label: String::from("post"),
          voltage: -80e-3,
          duration: 84.4e-3,
        };
        v_test += 0.01;
      }
    })
  }
}
