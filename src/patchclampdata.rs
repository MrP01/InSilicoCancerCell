use core::fmt;

use nalgebra::DVector;

#[allow(dead_code)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
#[derive(Clone)]
#[cfg_attr(feature = "default", derive(serde::Serialize, serde::Deserialize))]
pub enum PatchClampProtocol {
  Activation,
  Deactivation,
  Ramp,
}
impl fmt::Display for PatchClampProtocol {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PatchClampProtocol::Activation => write!(f, "Activation"),
      PatchClampProtocol::Deactivation => write!(f, "Deactivation"),
      PatchClampProtocol::Ramp => write!(f, "Ramp"),
    }
  }
}

#[allow(dead_code)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
#[derive(Clone)]
#[cfg_attr(feature = "default", derive(serde::Serialize, serde::Deserialize))]
pub enum CellPhase {
  G0,
  G1,
}
impl fmt::Display for CellPhase {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CellPhase::G0 => write!(f, "G0"),
      CellPhase::G1 => write!(f, "G1"),
    }
  }
}

#[cfg_attr(feature = "default", derive(serde::Serialize, serde::Deserialize, Clone))]
pub struct PatchClampData {
  pub protocol: PatchClampProtocol,
  pub phase: CellPhase,
  pub current: DVector<f64>,
}

impl PatchClampData {
  #[cfg(feature = "default")]
  pub fn load(protocol: PatchClampProtocol, phase: CellPhase) -> Result<PatchClampData, Box<dyn std::error::Error>> {
    let data_path = std::path::Path::new("data").join("provision");
    let file: std::fs::File = std::fs::File::open(match protocol {
      PatchClampProtocol::Activation => data_path.join("patch_clamp_data_activation.mat"),
      PatchClampProtocol::Deactivation => data_path.join("patch_clamp_data_deactivation.mat"),
      PatchClampProtocol::Ramp => data_path.join("patch_clamp_data_ramp.mat"),
    })?;
    let mat_file = matfile::MatFile::parse(file)?;
    let mat_arrays = mat_file.arrays();

    let array_name_regex = match protocol {
      PatchClampProtocol::Activation => regex::Regex::new(format!(r"m{}_\d+", phase).as_str()),
      PatchClampProtocol::Deactivation => regex::Regex::new(format!(r"m{}_\d+_deact", phase).as_str()),
      PatchClampProtocol::Ramp => regex::Regex::new(format!(r"m{}_\d+_ramp20", phase).as_str()),
    }
    .unwrap();
    let raw_data: Vec<&matfile::Array> = mat_arrays
      .iter()
      .filter(|array| array_name_regex.is_match(array.name()))
      .collect();
    log::info!("Found {} measurements for phase {}", raw_data.len(), phase);
    assert!(!raw_data.is_empty());
    let mut current: Option<nalgebra::DVector<f64>> = None;
    for measurement in &raw_data {
      if let matfile::NumericData::Double { real, imag: _ } = measurement.data() {
        match current {
          Some(curr) => current = Some(curr + DVector::from_vec(real.to_vec())),
          None => current = Some(DVector::from_vec(real.to_vec())),
        }
      } else {
        return Err(Box::new(matfile::Error::ConversionError));
      }
    }
    current = Some(current.unwrap() / (raw_data.len() as f64));
    match (&phase, &protocol) {
      (CellPhase::G0, PatchClampProtocol::Activation) => {
        let scaled = current.unwrap() * 1e11;
        current = Some(DVector::from_vec(
          scaled.iter().cloned().map(|x| x.max(-24.0)).collect::<Vec<f64>>(),
        ));
      }
      _ => {}
    }
    Ok(PatchClampData {
      protocol,
      phase,
      current: current.unwrap(),
    })
  }

  #[cfg(feature = "default")]
  pub fn to_json(&self) -> Result<String, serde_json::Error> {
    serde_json::to_string(self)
  }

  pub fn demo() -> PatchClampData {
    let mut c = DVector::zeros(100);
    for i in 0..c.len() {
      c[i] = 0.0 + 0.1 * (i as f64);
    }
    PatchClampData {
      protocol: PatchClampProtocol::Activation,
      phase: CellPhase::G0,
      current: c,
    }
  }
}
