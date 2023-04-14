use clap::Parser;
use hyper::Uri;
use std::path::Path;
use std::ffi::OsStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct InputParams {

  #[arg(short, long, default_value_t = 4, value_parser = connections_in_range)]
  pub connections: u16,

  #[arg(short, long, default_value_t = 1000)]
  pub requests: u64,

  pub target_url: Uri,

  #[arg(short, long, value_parser = file_exists)]
  pub output_file: Option<String>
}

fn file_exists(output_file: &str) -> Result<String, std::io::Error> {

  let file_path = Path::new(output_file);

  if file_path.extension().and_then(OsStr::to_str) != Some("csv") {
    return Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "The provided file must have .csv extension!"))
  }

  if file_path.try_exists()? {
    Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "The provided report file already exists!"))
  } else {
    Ok(String::from(output_file))
  }
}

fn connections_in_range(s: &str) -> Result<u16, String> {
  let error_msg = String::from("Value must be in the range [1..512]");
  match s.parse::<u16>() {
    Ok(value) => {
      if (1..65_000).contains(&value) {
        return Ok(value)
      } else {
        return Err(error_msg)
      }
    },
    Err(_err) => Err(error_msg)
  }
}

#[cfg(test)]
mod test_param_parser {
    use clap::Parser;
    use crate::input_params::InputParams;

  #[test]
  fn test_low_connections_in_range() {
    let args = InputParams::try_parse_from([
      "http-load-gen",
      "http://example.com",
      "-c", "0",
      ]);
    assert_eq!(args.is_err(), true);
  }

  #[test]
  fn test_high_connections_in_range() {
    let args = InputParams::try_parse_from([
      "http-load-gen",
      "http://example.com",
      "-c", "65123",
      ]);
    assert_eq!(args.is_err(), true);
  }

  #[test]
  fn test_valid_connections_in_range() {
    let args = InputParams::try_parse_from([
      "http-load-gen",
      "http://example.com",
      "-c", "12",
      ]);
    assert_eq!(args.is_ok(), true);
  }

  #[test]
  fn test_valid_url() {
    let args = InputParams::try_parse_from([
        "http-load-gen",
        "http:/example@com",
        "-c", "12",
        ]);
      assert_eq!(args.is_err(), true);
  }
}