use nalgebra::{SMatrix, SVector};

pub struct IonChannelCat<const N_STATES: usize> {
  pub n_channels: u32,
  pub state: SVector<f32, N_STATES>,
}

pub trait Named {
  fn name() -> String;
}

pub trait Simulatable<const N_STATES: usize> {
  fn update(&mut self);
}

pub trait HasTransitionMatrix<const N_STATES: usize> {
  fn initial_state(&self) -> SVector<f32, N_STATES>;
  fn transition_matrix(&self, V: f32) -> SMatrix<f32, N_STATES, N_STATES>;
}

impl<const N_STATES: usize> Simulatable<N_STATES> for IonChannelCat<N_STATES>
where
  IonChannelCat<N_STATES>: HasTransitionMatrix<N_STATES>,
{
  fn update(&mut self) {
    let V = 3.5;
    self.state = self.transition_matrix(V) * self.state;
  }
}
