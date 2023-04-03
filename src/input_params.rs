use clap::Parser;
use hyper::Uri;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct InputParams {

  #[arg(short, long, default_value_t = 4, value_parser = connections_in_range)]
  pub connections: u16,

  #[arg(short, long, default_value_t = 1000)]
  pub requests: u64,

  pub target_url: Uri,

  #[arg(short, long)]
  pub output_file: Option<String>
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