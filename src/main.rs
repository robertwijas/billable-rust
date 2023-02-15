 use billable::reports::FormattingOptions;
use billable::reports::Month;
use clap::Parser;
use colored::Colorize;
use config::Config;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    months: usize,
    #[arg(short, long, default_value_t = false)]
    show_minutes: bool,
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

    if let Some(services) = config.services {
        for service in services.iter() {
            println!("{}", service.display_name().bold());

            let billable = service.billable();
            for month in Month::current().iter().rev().take(args.months) {
                billable.print_report(
                    month,
                    FormattingOptions {
                        show_minutes: args.show_minutes,
                    },
                    &config.clients,
                );
            }
            println!();
        }
    }
}
