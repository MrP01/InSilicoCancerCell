use nalgebra::SMatrix;

pub trait HasTransitionMatrix<const N_STATES: usize> {
  fn transition_matrix(&self, voltage: f64) -> SMatrix<f64, N_STATES, N_STATES>;
}

pub trait IsChannel {
  fn update_state(&mut self, voltage: f64);
  fn current(&self, voltage: f64) -> f64;
  fn display_me(&self) -> String;
}

#[macro_export]
macro_rules! define_ion_channel {
  ( $name: ident, $display_name: expr, $N_STATES: expr, $conductance: expr, ($($initial_state: expr), *) ) => {
    pub struct $name {
      pub state: nalgebra::SVector<f64, $N_STATES>,
      pub n_channels: u32,
    }
    #[allow(non_upper_case_globals)]
    impl $name{
      pub const n_states: usize = $N_STATES;
      pub const conductance: f64 = $conductance;
      pub fn display_name() -> String {
        return String::from($display_name);
      }
      pub fn new() -> Self {
        return $name {
          n_channels: 10,
          state: nalgebra::SVector::<f64, $N_STATES>::from_vec(vec![$($initial_state), *]),
        };
      }
    }
    impl crate::channels::base::IsChannel for $name {
      fn update_state(&mut self, voltage: f64) {
        self.state = self.transition_matrix(voltage) * self.state;
      }
      fn current(&self, voltage: f64) -> f64 {
        Self::conductance * self.state[1] * (voltage - constants::EvK)
      }
      fn display_me(&self) -> String {
        format!(
          "Simulating {} ({} states) with {} channels.",
          Self::display_name(),
          Self::n_states,
          self.n_channels
        )
      }
    }
  };
}
