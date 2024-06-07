#![feature(coroutines, iter_from_coroutine, type_alias_impl_trait, cfg_eval)]
#![allow(dead_code)]

mod cell;
mod channels;
mod constants;
mod optimisation;
mod patchclampdata;
mod pulseprotocol;
mod utils;

use cell::evaluate_match;
use cell::A549CancerCell;
use cell::TotalCurrentRecord;
use clap::{Parser, Subcommand};
use nalgebra::DVector;
use patchclampdata::{CellPhase, PatchClampData, PatchClampProtocol};
use pulseprotocol::ProtocolGenerator;

fn evaluate_on_langthaler_et_al_counts(measurements: PatchClampData) {
  let pulse_protocol = ProtocolGenerator {
    proto: measurements.protocol.clone(),
  };
  let mut cell = A549CancerCell::new();
  cell.set_langthaler_et_al_channel_counts(measurements.phase.clone());
  let mut recorded = TotalCurrentRecord::empty();
  cell.simulate(pulse_protocol, &mut recorded, measurements.current.len());
  evaluate_match(&measurements, recorded);
}

fn save_to_json(measurements: PatchClampData, subsampling: Option<usize>) {
  let path = format!(
    "frontend/pkg/patchclampdata-{}-{}{}.json",
    measurements.phase.to_string().to_lowercase(),
    measurements.protocol.to_string().to_lowercase(),
    match subsampling {
      Some(subsamp) => format!("-sub{}", subsamp),
      None => String::from(""),
    }
  );
  let mut subsampled_measurements = measurements.clone();
  match subsampling {
    Some(subsamp) => {
      subsampled_measurements.current =
        DVector::from_vec(measurements.current.iter().cloned().step_by(subsamp).collect())
    }
    None => {
      let subsamp = measurements.current.len() / 800;
      subsampled_measurements.current =
        DVector::from_vec(measurements.current.iter().cloned().step_by(subsamp).collect())
    }
  };
  let file = std::fs::File::create(&path).unwrap();
  let writer = std::io::BufWriter::new(file);
  serde_json::to_writer(writer, &subsampled_measurements).unwrap();
  log::info!("Wrote to {}", &path);
}

#[derive(Parser)]
#[clap(arg_required_else_help = true)]
#[command(
  about = "In-Silico Cancer Cell Model Simulator",
  author = "Peter Waldert <peter@waldert.at>",
  version = "0.2.0"
)]
struct Cli {
  /// Turn debugging information on
  #[arg(short, long, action = clap::ArgAction::Count)]
  debug: u8,

  /// The voltage protocol
  #[arg(long, default_value = "activation")]
  protocol: PatchClampProtocol,

  /// The cell cycle phase
  #[arg(long, default_value = "g0")]
  phase: CellPhase,

  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
enum Command {
  #[command(about = "Evaluate the model on the parameters supplied by Langthaler et al.")]
  Single,
  #[command(about = "Perform a large-scale optimisation on the number of channels per type")]
  Fit { using: optimisation::InSilicoMethod },
  #[command(about = "Save patch clamp data (measurements) to a JSON file")]
  SavePatchClampData { subsampling: Option<usize> },
}

fn main() {
  utils::setup_logging();
  let cli = Cli::parse();
  let measurements = PatchClampData::load(cli.protocol, cli.phase).unwrap();
  match cli.command {
    Command::Single => {
      evaluate_on_langthaler_et_al_counts(measurements);
    }
    Command::Fit { using } => {
      optimisation::find_best_fit_for(measurements, using);
    }
    Command::SavePatchClampData { subsampling } => {
      save_to_json(measurements, subsampling);
    }
  }
}
