use billable::reports::display::FormattingOptions;
use billable::reports::display::Printer;
use billable::reports::Month;
use clap::Parser;
use colored::Colorize;
use config::Config;
use dialoguer::Confirm;

const CONFIG_FILE_EXAMPLE: &str = include_str!("../config.toml.example");

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    months: usize,
    #[arg(short, long, default_value_t = false)]
    show_minutes: bool,
    #[arg[short, long, default_value_t = String::from("config")]]
    config_name: String,
}

fn main() {
    let args = Args::parse();

    let project_dirs = directories::ProjectDirs::from("com", "robertwijas", "billable")
        .expect("failed to find project directory");
    let user_config_path = project_dirs.config_dir().join(&args.config_name);

    let config = Config::builder()
        .add_source(config::File::with_name(user_config_path.to_str().unwrap()).required(false))
        .add_source(config::File::with_name(&args.config_name).required(false))
        .build()
        .expect("cannot build configuration")
        .try_deserialize::<billable::Config>()
        .expect("failed to read configuration");

    if let Some(services) = config.services {
        let printer = Printer {
            formatting_options: FormattingOptions {
                show_minutes: args.show_minutes,
            },
        };
        for service in services.iter() {
            println!("[{}]", service.display_name().bold());

            let billable = service.billable();
            for month in Month::current().iter().rev().take(args.months) {
                printer.print(billable.monthly_report(month, &config.clients).unwrap());
            }
            println!();
        }
    } else {
        println!("There are no services configured.");
        if Confirm::new()
            .with_prompt("Would you like to create an example configuration file?")
            .interact()
            .unwrap_or(false)
        {
            let path = user_config_path.with_extension("toml");
            std::fs::write(&path, CONFIG_FILE_EXAMPLE).expect("failed to write configuration");

            println!(
                "An example configuration has been saved at:\n{}",
                path.to_str().unwrap()
            );
            println!();
            println!(
                "You can run billable again to see demo reports \
                     or edit the configuration file to use your Toggl/Harvest account."
            )
        }
    }
}
