pub struct IonChannelCat<const N_STATES: usize> {
  n_channels: u32,
  state: nalgebra::SVector<f32, N_STATES>,
}

pub trait Named {
  fn name() -> String;
}

pub trait Simulatable<const N_STATES: usize> {
  fn update(&mut self);
}

pub trait HasTransitionMatrix<const N_STATES: usize> {
  fn transition_matrix(&self) -> nalgebra::SMatrix<f32, N_STATES, N_STATES>;
}

impl<const N_STATES: usize> Simulatable<N_STATES> for IonChannelCat<N_STATES>
where
  IonChannelCat<N_STATES>: HasTransitionMatrix<N_STATES>,
{
  fn update(&mut self) {
    self.state = self.transition_matrix() * self.state;
  }
}
