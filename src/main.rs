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
use pulseprotocol::DefaultPulseProtocol;

fn evaluate_on_langthaler_et_al_counts(measurements: PatchClampData) {
  let pulse_protocol = DefaultPulseProtocol {};
  let mut cell = A549CancerCell::new();
  cell.set_langthaler_et_al_channel_counts(measurements.phase.clone());
  let mut recorded = TotalCurrentRecord::empty();
  cell.simulate(pulse_protocol, &mut recorded, measurements.current.len());
  evaluate_match(&measurements, recorded);
}

#[derive(Parser)]
#[clap(arg_required_else_help = true)]
#[command(
  about = "In-Silico Cancer Cell Model Simulator",
  author = "Peter Waldert <peter@waldert.at>",
  version = "0.1.0"
)]
struct Cli {
  /// Turn debugging information on
  #[arg(short, long, action = clap::ArgAction::Count)]
  debug: u8,

  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
enum Command {
  #[command(about = "Evaluate the model on the parameters supplied by Langthaler et al.")]
  RunSingle,
  #[command(about = "Perform a large-scale optimisation on the number of channels per type")]
  Optimise,
}

fn main() {
  utils::setup_logging();
  let measurements = PatchClampData::load(PatchClampProtocol::Ramp, CellPhase::G0).unwrap();

  let cli = Cli::parse();
  match cli.command {
    Command::RunSingle => {
      evaluate_on_langthaler_et_al_counts(measurements);
    }
    Command::Optimise => {
      optimisation::find_best_fit_for(measurements, optimisation::InSilicoOptimiser::ParticleSwarm);
    }
  }
}
