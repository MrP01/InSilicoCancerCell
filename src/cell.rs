use crate::{
  channels::{
    base::{Constructable, Simulatable},
    crac1,
  },
  constants,
};

pub struct A549CancerCell {
  crac1_channel: crac1::CRAC1IonChannelCat,
}

impl A549CancerCell {
  pub fn new() -> A549CancerCell {
    return A549CancerCell {
      crac1_channel: crac1::CRAC1IonChannelCat::new(),
    };
  }

  pub fn simulate(&mut self, total_time: f64) {
    let mut time = 0.0;
    while time < total_time {
      self.crac1_channel.update();
      time += constants::dt;
    }
  }
}
