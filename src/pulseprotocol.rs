pub struct PulseProtocolStep {
  pub label: String,
  pub voltage: f64,
  pub duration: f64,
}

pub trait ProtocolGenerator {
  fn generator(&self) -> impl Iterator<Item = PulseProtocolStep>;
  fn total_duration(&self) -> f64;
}

pub struct DefaultPulseProtocol {}
impl ProtocolGenerator for DefaultPulseProtocol {
  fn generator(&self) -> impl Iterator<Item = PulseProtocolStep> {
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

  fn total_duration(&self) -> f64 {
    self.generator().map(|step| step.duration).sum()
  }
}
