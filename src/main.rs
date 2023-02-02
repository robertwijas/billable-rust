use billable::reports::Billable;
use billable::reports::Month;
use clap::Parser;
use config::Config;
use colored::*;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    months: u8,
}

fn main() {
    let project_dirs = directories::ProjectDirs::from("com", "robertwijas", "billable")
        .expect("Failed to find project directory");

    let user_config_path = project_dirs.config_dir().join("config");

    let config = Config::builder()
        .add_source(config::File::with_name("config").required(false))
        .add_source(config::File::with_name(user_config_path.to_str().unwrap()))
        .build()
        .expect("Cannot build config.")
        .try_deserialize::<billable::Config>()
        .expect("Failed to read configuration.");

    let args = Args::parse();

    let billable = billable::reports::toggl::Billable::new(config.api_token);

    let mut month = Month::current();
    for _ in 0..args.months {
        report(&billable, month.clone());
        month = month.previous();
    }
}

fn report(billable: &dyn Billable, month: Month) {
    println!("{}", format!("{}", month).bold().reversed());
    println!(
        "{}",
        billable
            .report(month.into())
            .expect("Failed to prepare report.")
    );
}
