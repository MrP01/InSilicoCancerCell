use crate::patchclampdata::PatchClampProtocol;

pub struct PulseProtocolStep {
  pub label: String,
  pub voltage: f64,
  pub duration: f64,
}

pub struct ProtocolGenerator {
  pub proto: PatchClampProtocol,
}
impl ProtocolGenerator {
  pub fn generator(&self) -> impl Iterator<Item = PulseProtocolStep> + '_ {
    std::iter::from_coroutine(
      #[coroutine]
      || {
        match self.proto {
          PatchClampProtocol::Activation => {
            let mut v_test = -40e-3; // -40 mV start
            while v_test <= 50e-3 {
              // increment to +50 mV
              yield PulseProtocolStep {
                label: String::from("hold"),
                voltage: -100e-3,
                duration: 100e-3,
              };
              yield PulseProtocolStep {
                label: String::from("initial"),
                voltage: -80e-3,
                duration: 30e-3,
              };
              yield PulseProtocolStep {
                label: String::from("test"),
                voltage: v_test,
                duration: 870e-3,
              };
              yield PulseProtocolStep {
                label: String::from("post"),
                voltage: -80e-3,
                duration: 100e-3,
              };
              v_test += 10e-3;
            }
          }
          PatchClampProtocol::Deactivation => {
            let mut v_test = -40e-3; // -40 mV start
            while v_test <= 50e-3 {
              // increment to +50 mV
              yield PulseProtocolStep {
                label: String::from("hold"),
                voltage: -100e-3,
                duration: 100e-3,
              };
              yield PulseProtocolStep {
                label: String::from("initial"),
                voltage: -80e-3,
                duration: 30e-3,
              };
              yield PulseProtocolStep {
                label: String::from("test"),
                voltage: v_test,
                duration: 870e-3,
              };
              yield PulseProtocolStep {
                label: String::from("post"),
                voltage: -80e-3,
                duration: 100e-3,
              };
              v_test += 10e-3;
            }
          }
          PatchClampProtocol::Ramp => {
            let mut v_test = -40e-3; // -40 mV start
            while v_test <= 50e-3 {
              // increment to +50 mV
              yield PulseProtocolStep {
                label: String::from("hold"),
                voltage: -100e-3,
                duration: 100e-3,
              };
              yield PulseProtocolStep {
                label: String::from("initial"),
                voltage: -80e-3,
                duration: 30e-3,
              };
              yield PulseProtocolStep {
                label: String::from("test"),
                voltage: v_test,
                duration: 870e-3,
              };
              yield PulseProtocolStep {
                label: String::from("post"),
                voltage: -80e-3,
                duration: 100e-3,
              };
              v_test += 10e-3;
            }
          }
        }
      },
    )
  }
  pub fn single_duration(&self) -> f64 {
    self.generator().map(|step| step.duration).sum()
  }
}

// pub trait RepeatingGenerator {
//   fn chain_multiple(&self, times: usize) -> Box<dyn Iterator<Item = PulseProtocolStep>>;
// }
// impl<T> RepeatingGenerator for T
// where
//   T: ProtocolGenerator,
// {
//   fn chain_multiple(&self, times: usize) -> Box<dyn Iterator<Item = PulseProtocolStep>> {
//     Box::new(std::iter::from_coroutine(
//       #[coroutine]
//       move || {
//         for _i in 0..times {
//           for step in self.generator() {
//             yield step;
//           }
//         }
//       },
//     ))
//   }
// }
