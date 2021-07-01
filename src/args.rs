use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

#[derive(Clone)]
pub struct Args {
  pub rocket_port: u16,
  pub zeronet_path: String,
  pub site_address: String,
}

fn is_u16(v: String) -> Result<(), String> {
  match v.parse::<u16>() {
    Ok(_) => Ok(()),
    _ => Err(format!("'{}' cannot be parsed to u16.", v)),
  }
}

pub fn get_arguments() -> Option<Args> {
  let mut app = app_from_crate!();
  let matches = app
    .arg(
      Arg::with_name("rocket_port")
        .short("p")
        .long("rocket_port")
        .alias("port")
        .help("Port to serve the site on.")
        .env("ROCKET_PORT")
        .validator(is_u16)
        .default_value("8000"),
    )
    .arg(
      Arg::with_name("zeronet_path")
        .short("P")
        .long("zeronet_path")
        .alias("path")
        .help("Path to the ZeroNet data directory.")
        .env("ZERONET_PATH"),
    )
    .arg(
      Arg::with_name("site_address")
        .short("a")
        .long("site_address")
        .aliases(&["site", "address"])
        .help("Address of the ZeroBlog to statify.")
        .env("SITE_ADDRESS"),
    )
    .get_matches();

  let args = Args {
    rocket_port: matches.value_of("rocket_port")?.parse().ok()?,
    zeronet_path: matches.value_of("zeronet_path")?.to_string(),
    site_address: matches.value_of("site_address")?.to_string(),
  };

  Some(args)
}
