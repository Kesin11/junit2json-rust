use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  /// JUnit XML path
  pub path: String,

  /// Output pretty JSON
  #[arg(short, long, default_value = "false")]
  pub pretty: bool,

  /// Filter XML tag names
  #[arg(short, long)]
  pub filter_tags: Option<Vec<String>>,
}
