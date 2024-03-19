use nalgebra::DVector;

use crate::{
  channels::{
    base::{Constructable, Simulatable},
    crac1,
  },
  constants,
  pulseprotocol::PulseProtocol,
};

pub struct MembraneCurrentThroughput {
  current: Vec<f64>,
}

impl MembraneCurrentThroughput {
  pub fn empty() -> Self {
    Self { current: vec![] }
  }

  pub fn as_dvec(self) -> DVector<f64> {
    DVector::<f64>::from_vec(self.current)
  }
}

pub struct A549CancerCell {
  crac1_channel: crac1::CRAC1IonChannelCat,
}

impl A549CancerCell {
  pub fn new() -> A549CancerCell {
    return A549CancerCell {
      crac1_channel: crac1::CRAC1IonChannelCat::new(),
    };
  }

  pub fn simulate(&mut self, pulse_protocol: PulseProtocol) -> MembraneCurrentThroughput {
    let mut total_time = 0.0;
    let mut recorded = MembraneCurrentThroughput::empty();
    for step in pulse_protocol {
      log::info!(
        "Pulse protocol step {} ({:.3} V) for {:.3} s",
        step.label,
        step.voltage,
        step.duration
      );
      let mut time: f64 = 0.0;
      while time < step.duration {
        self.crac1_channel.update_state(step.voltage);
        recorded.current.push(self.crac1_channel.current());
        time += constants::dt;
      }
      total_time += time;
    }
    log::info!("Total simulation time: {total_time} s");
    return recorded;
  }
}
