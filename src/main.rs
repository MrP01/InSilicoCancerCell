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
use patchclampdata::{CellPhase, PatchClampData, PatchClampProtocol};
use pulseprotocol::ProtocolGenerator;

fn evaluate_on_langthaler_et_al_counts(measurements: PatchClampData) {
  let pulse_protocol = ProtocolGenerator {
    proto: measurements.protocol.clone(),
  };
  let mut cell = A549CancerCell::new();
  cell.set_langthaler_et_al_channel_counts(measurements.phase.clone());
  let mut recorded = TotalCurrentRecord::empty();
  cell.simulate(
    pulse_protocol,
    &mut recorded,
    measurements.current.len(),
    true,
    constants::default_delta_tolerance,
  );
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
  let subsampled_measurements = measurements.subsampled(match subsampling {
    Some(subsamp) => subsamp,
    None => measurements.current.len() / 800,
  });
  let file = std::fs::File::create(&path).unwrap();
  let writer = std::io::BufWriter::new(file);
  serde_json::to_writer(writer, &subsampled_measurements).unwrap();
  log::info!("Wrote to {}", &path);
}

pub fn compare_adaptivity() {
  let mut cell = A549CancerCell::new();
  let mut iterations = [0; 2];
  for adaptive_dt in vec![true, false] {
    for protocol in enum_iterator::all::<PatchClampProtocol>() {
      for phase in enum_iterator::all::<CellPhase>() {
        cell.set_langthaler_et_al_channel_counts(phase);
        let pulse_protocol = ProtocolGenerator { proto: protocol };
        let mut recorded = TotalCurrentRecord::empty();
        let result = cell.simulate(
          pulse_protocol,
          &mut recorded,
          800,
          adaptive_dt,
          constants::default_delta_tolerance,
        );
        iterations[adaptive_dt as usize] += result.n_iterations;
      }
    }
  }
  println!("Without adaptive timestepping: {}", iterations[0]);
  println!("With adaptive timestepping: {}", iterations[1]);
}

pub fn compare_delta_tolerance(measurements: PatchClampData) {
  let mut cell = A549CancerCell::new();
  cell.set_clarabel_channel_counts(measurements.phase);
  println!("[");
  for delta_tolerance in (0..28).map(|i| (10.0_f64).powf(-(i as f64) / 4.0)) {
    let pulse_protocol = ProtocolGenerator {
      proto: measurements.protocol,
    };
    let mut recorded = TotalCurrentRecord::empty();
    let result = cell.simulate(
      pulse_protocol,
      &mut recorded,
      measurements.current.len(),
      true,
      delta_tolerance,
    );
    println!(
      "{{ \"tolerance\": {:.2e}, \"average_dt\": {:.2e}, \"accept_rate\": {:.1}, \"iterations_k\": {}, \"runtime\": {:.3}, \"error\": {:.3} }},",
      delta_tolerance,
      result.average_dt,
      result.accept_rate,
      (result.n_iterations / 1000) as u32,
      result.runtime.expect("Did not produce runtime (wasm?)"),
      evaluate_match(&measurements, recorded)
    );
  }
  println!("]");
}

#[derive(Parser)]
#[clap(arg_required_else_help = true)]
#[command(
  about = "In-Silico Cancer Cell Model Simulator",
  author = "Peter Waldert <peter@waldert.at>",
  version = "0.2.3"
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

  /// The measurement subsampling
  #[arg(long)]
  subsampling: Option<usize>,

  /// The measurement smoothing
  #[arg(long)]
  smoothing: Option<usize>,

  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
enum Command {
  #[command(about = "Evaluate the model on the parameters supplied by Langthaler et al.")]
  Single,
  #[command(about = "Compare adaptivity")]
  CompareAdaptivity,
  #[command(about = "Compare delta tolerance")]
  CompareDeltaTolerance,
  #[command(about = "Perform a large-scale optimisation on the number of channels per type")]
  Fit { using: optimisation::InSilicoMethod },
  #[command(about = "Save patch clamp data (measurements) to a JSON file")]
  SavePatchClampData { subsampling: Option<usize> },
}

fn main() {
  utils::setup_logging();
  let cli = Cli::parse();
  let mut measurements = PatchClampData::load(cli.protocol, cli.phase).unwrap();
  match cli.subsampling {
    Some(subsamp) => measurements = measurements.subsampled(subsamp),
    _ => {}
  }
  match cli.smoothing {
    Some(smoothing) => measurements = measurements.smoothed(smoothing),
    _ => {}
  }
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
    Command::CompareAdaptivity => compare_adaptivity(),
    Command::CompareDeltaTolerance => compare_delta_tolerance(measurements),
  }
}
