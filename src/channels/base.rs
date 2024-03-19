use nalgebra::{SMatrix, SVector};

use crate::constants;

pub struct IonChannelCat<const N_STATES: usize> {
  pub n_channels: u32,
  pub state: SVector<f64, N_STATES>,
}

pub trait Named {
  fn name() -> String;
}

pub trait Simulatable<const N_STATES: usize> {
  fn update_state(&mut self, voltage: f64);
  fn current(&self, voltage: f64) -> f64;
}

pub trait HasTransitionMatrix<const N_STATES: usize> {
  #[allow(non_upper_case_globals)]
  const conductance: f64;
  fn initial_state() -> SVector<f64, N_STATES>;
  fn transition_matrix(&self, voltage: f64) -> SMatrix<f64, N_STATES, N_STATES>;
}

impl<const N_STATES: usize> Simulatable<N_STATES> for IonChannelCat<N_STATES>
where
  IonChannelCat<N_STATES>: HasTransitionMatrix<N_STATES>,
{
  fn update_state(&mut self, voltage: f64) {
    self.state = self.transition_matrix(voltage) * self.state;
  }

  fn current(&self, voltage: f64) -> f64 {
    IonChannelCat::conductance * self.state[1] * (voltage - constants::EvK)
  }
}

pub trait Constructable<const N_STATES: usize> {
  fn new() -> impl HasTransitionMatrix<N_STATES>;
}

#[allow(refining_impl_trait)]
impl<const N_STATES: usize> Constructable<N_STATES> for IonChannelCat<N_STATES>
where
  IonChannelCat<N_STATES>: HasTransitionMatrix<N_STATES>,
{
  fn new() -> Self {
    return IonChannelCat {
      n_channels: 10,
      state: Self::initial_state(),
    };
  }
}
