use nalgebra::DVector;

pub struct PatchClampData {
  voltages: DVector<f32>,
}

impl PatchClampData {
  pub fn load() -> Result<PatchClampData, Box<dyn std::error::Error>> {
    let file: std::fs::File = std::fs::File::open("data/provision/patch_clamp_data_ramp.mat")?;
    let mat_file = matfile::MatFile::parse(file)?;
    let pos = mat_file
      .find_by_name("mG0_10_ramp20")
      .expect("Could not find correct array in .mat file");
    println!("{:#?}", pos);
    return Ok(PatchClampData {
      voltages: DVector::from_element(2000, 1.0),
    });
  }
}
