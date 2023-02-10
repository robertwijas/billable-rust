use billable::reports::Billable;
use billable::reports::Month;
use clap::Parser;
use config::Config;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    months: usize,
}

fn main() {
    let project_dirs = directories::ProjectDirs::from("com", "robertwijas", "billable")
        .expect("failed to find project directory");

    let user_config_path = project_dirs.config_dir().join("config");

    let config = Config::builder()
        .add_source(config::File::with_name(user_config_path.to_str().unwrap()))
        .add_source(config::File::with_name("config").required(false))
        .build()
        .expect("cannot build config")
        .try_deserialize::<billable::Config>()
        .expect("failed to read configuration");

    let args = Args::parse();

    let billable = billable::reports::toggl::Billable::new(config.api_token);

    for month in Month::current().iter().rev().take(args.months) {
        billable.print_report(month.clone(), &config.clients);
    }
}
