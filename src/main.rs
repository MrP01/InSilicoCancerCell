use utils::exit_on_error;

mod cell;
mod channels;
mod constants;
mod patchclampdata;
mod utils;

fn main() {
  let data = exit_on_error(patchclampdata::PatchClampData::load());
}
