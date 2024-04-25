use nalgebra::SMatrix;

pub trait HasTransitionMatrix<const N_STATES: usize> {
  fn transition_matrix(&self, voltage: f64) -> SMatrix<f64, N_STATES, N_STATES>;
}

pub trait IsChannel {
  fn update_state(&mut self, voltage: f64);
  fn current(&self, voltage: f64) -> f64;
  fn internal_state(&self) -> Vec<f64>;
  fn display_name(&self) -> String;
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
        let transition = self.transition_matrix(voltage);
        #[cfg(debug_assertions)]
        crate::channels::base::validate_transition_matrix::<$N_STATES>(Self::display_name(), transition);
        self.state = transition * self.state;
      }
      fn current(&self, voltage: f64) -> f64 {
        Self::conductance * self.state[1] * (voltage - constants::EvK)
      }
      fn internal_state(&self) -> Vec<f64> {
        self.state.iter().cloned().collect()
      }
      fn display_name(&self) -> String {
        Self::display_name()
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

#[cfg(debug_assertions)]
pub fn validate_transition_matrix<const N_STATES: usize>(
  channel: String,
  matrix: nalgebra::SMatrix<f64, N_STATES, N_STATES>,
) {
  let mut bad = false;
  if matrix.min() < 0.0 {
    log::warn!("Transition matrix of {channel} has negative values!");
    bad = true;
  }
  if matrix.max() > 1.0 {
    log::warn!("Transition matrix of {channel} has values > 1!");
    bad = true;
  }
  if (matrix.row_sum().transpose() - nalgebra::SVector::<f64, N_STATES>::from_element(1.0)).norm_squared()
    > 1000.0 * f64::EPSILON
  {
    log::warn!("Transition matrix of {channel} does not sum to 1!");
    bad = true;
  }
  if bad {
    log::debug!("Matrix: {}", matrix);
    log::debug!("Row sum: {}", matrix.row_sum(),);
    log::debug!("Column sum: {}", matrix.column_sum());
  }
}
