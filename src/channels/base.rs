use nalgebra::SMatrix;

pub trait HasTransitionMatrix<const N_STATES: usize> {
  fn transition_matrix(&self, voltage: f64) -> SMatrix<f64, N_STATES, N_STATES>;
}

pub trait IsChannel {
  fn update_state(&mut self, voltage: f64);
  fn single_channel_current(&self, voltage: f64) -> f64;
  fn current(&self, voltage: f64) -> f64;
  fn internal_state(&self) -> Vec<f64>;
  fn display_name(&self) -> String;
  fn display_me(&self) -> String;
}

#[macro_export]
macro_rules! define_ion_channel {
  (
    $name: ident,
    $display_name: expr,
    $n_states: expr,
    $iontype: expr,
    $conductance: expr,
    ($($states_responsible_for_current: expr), *)
  ) => {
    pub struct $name {
      pub state: nalgebra::SVector<f64, $n_states>,
      pub n_channels: u32,
    }

    #[allow(non_upper_case_globals)]
    impl $name {
      pub const n_states: usize = $n_states;
      pub const conductance: f64 = $conductance;
      pub fn display_name() -> String {
        return String::from($display_name);
      }
      pub fn new() -> Self {
        let mut x0 = nalgebra::SVector::<f64, $n_states>::from_vec(vec![0.0; $n_states]);
        x0[0] = 1.0;
        return $name {
          n_channels: 1,
          state: x0,
        };
      }
    }
    impl $crate::channels::base::IsChannel for $name {
      fn update_state(&mut self, voltage: f64) {
        let transition = self.transition_matrix(voltage);
        #[cfg(debug_assertions)]
        $crate::channels::base::validate_transition_matrix::<$n_states>(Self::display_name(), transition);
        self.state = transition * self.state;
      }
      fn single_channel_current(&self, voltage: f64) -> f64 {
        let mut open = 0.0;
        $(open += self.state[$states_responsible_for_current];)+
        Self::conductance * open * (self.n_channels as f64) * (voltage - constants::reversal_potential($iontype))
      }
      fn current(&self, voltage: f64) -> f64 {
        (self.n_channels as f64) * self.single_channel_current(voltage)
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
    impl Default for $name {
      fn default() -> Self {
        Self::new()
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
  if (matrix.row_sum().transpose() - nalgebra::SVector::<f64, N_STATES>::from_element(1.0)).norm_squared() > 1e-6 {
    log::warn!("Transition matrix of {channel} does not sum to 1!");
    bad = true;
  }
  if bad {
    log::debug!("Matrix: {}", matrix);
    log::debug!("Row sum: {}", matrix.row_sum(),);
    log::debug!("Column sum: {}", matrix.column_sum());
  }
}
