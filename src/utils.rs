pub fn exit_on_error<T>(res: Result<T, Box<dyn std::error::Error>>) -> T {
  match res {
    Ok(data) => data,
    Err(error) => {
      println!("In Silico Error: {}", error);
      std::process::exit(-1);
    }
  }
}
