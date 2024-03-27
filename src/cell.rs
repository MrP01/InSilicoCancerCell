use nalgebra::DVector;

use crate::{
  channels::{
    base::{Constructable, HasTransitionMatrix, IonChannelCat, Named, Simulatable},
    crac1, kv71,
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
  kv71_channel: kv71::KV71IonChannelCat,
}

impl A549CancerCell {
  pub fn new() -> A549CancerCell {
    return A549CancerCell {
      crac1_channel: crac1::CRAC1IonChannelCat::new(),
      kv71_channel: kv71::KV71IonChannelCat::new(),
    };
  }

  pub fn channels(&mut self) -> Vec<Box<&mut dyn Simulatable>> {
    return vec![Box::new(&mut self.crac1_channel), Box::new(&mut self.kv71_channel)];
  }

  pub fn simulate(&mut self, pulse_protocol: PulseProtocol) -> MembraneCurrentThroughput {
    let mut total_time = 0.0;
    let mut recorded = MembraneCurrentThroughput::empty();
    log::info!(
      "Simulating {} with {} channels.",
      crac1::CRAC1IonChannelCat::name(),
      self.crac1_channel.n_channels
    );
    log::info!(
      "Simulating {} with {} channels.",
      kv71::KV71IonChannelCat::name(),
      self.kv71_channel.n_channels
    );
    for step in pulse_protocol {
      log::info!(
        "Pulse protocol step {} ({:.3} V) for {:.3} s",
        step.label,
        step.voltage,
        step.duration
      );
      let mut time: f64 = 0.0;
      while time < step.duration {
        let mut total_current = 0.0;
        for channel in self.channels() {
          channel.update_state(step.voltage);
          total_current += channel.current(step.voltage);
        }
        recorded.current.push(total_current);
        time += constants::dt;
      }
      total_time += time;
    }
    log::info!("Total simulation time: {total_time:.3} s");
    return recorded;
  }
}
