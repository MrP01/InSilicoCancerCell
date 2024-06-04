use nalgebra::SMatrix;

use crate::constants::IonType;

pub trait HasTransitionMatrix<const N_STATES: usize> {
  fn transition_matrix(&self, voltage: f64, dt: f64) -> SMatrix<f64, N_STATES, N_STATES>;
}

pub struct ChannelMetadata {
  pub n_states: usize,
  pub n_channels: u32,
  pub ion_type: IonType,
}

pub trait IsChannel {
  fn update_state(&mut self, voltage: f64, dt: f64) -> f64;
  fn reset_state(&mut self);
  fn single_channel_current(&self, voltage: f64) -> f64;
  fn current(&self, voltage: f64) -> f64;
  fn internal_state(&self) -> Vec<f64>;
  fn display_name(&self) -> String;
  fn display_me(&self) -> String;
  fn metadata(&self) -> ChannelMetadata;
  fn set_n_channels(&mut self, n_channels: u32);
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
      pub fn initial_state() -> nalgebra::SVector::<f64, $n_states> {
        let mut x0 = nalgebra::SVector::<f64, $n_states>::from_vec(vec![0.0; $n_states]);
        x0[0] = 1.0;
        x0
      }
      pub fn new() -> Self {
        return $name {
          n_channels: 1,
          state: Self::initial_state(),
        };
      }
    }
    impl $crate::channels::base::IsChannel for $name {
      fn update_state(&mut self, voltage: f64, dt: f64) -> f64 {
        let transition = self.transition_matrix(voltage, dt);
        #[cfg(debug_assertions)]
        $crate::channels::base::validate_transition_matrix::<$n_states>(Self::display_name(), transition);
        let previous = self.state.clone();
        // use $crate::utils::{Roundable,Cappable};
        // self.state = (((transition * previous) * (self.n_channels as f64)).round() / (self.n_channels as f64)).cap_all_values_to(1.0).lower_cap_all_values_to(0.0);
        self.state = transition * previous;
        #[cfg(debug_assertions)]
        $crate::channels::base::validate_state::<$n_states>(Self::display_name(), self.state);
        return (self.state - previous).norm_squared() * (self.n_channels as f64);  // delta
      }
      fn reset_state(&mut self) {
        self.state = Self::initial_state();
      }
      fn set_n_channels(&mut self, n_channels: u32) {
        self.n_channels = n_channels;
      }
      fn single_channel_current(&self, voltage: f64) -> f64 {
        let mut open = 0.0;
        $(open += self.state[$states_responsible_for_current];)+
        Self::conductance * open * (voltage - $crate::constants::reversal_potential($iontype))
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
          "Simulating {} ({} states, conductance {:e}) with {} channel(s)",
          Self::display_name(),
          Self::n_states,
          Self::conductance,
          self.n_channels
        )
      }
      fn metadata(&self) -> crate::channels::base::ChannelMetadata {
        crate::channels::base::ChannelMetadata {
          n_channels: self.n_channels,
          n_states: Self::n_states,
          ion_type: $iontype,
        }
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

#[cfg(debug_assertions)]
pub fn validate_state<const N_STATES: usize>(channel: String, state: nalgebra::SVector<f64, N_STATES>) {
  if (state.sum() - 1.0).abs() > 1e-8 {
    log::warn!(
      "State (probability distribution) of {channel} does not sum to 1! Instead: {}",
      state.sum()
    );
  }
}
