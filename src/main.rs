use billable::reports::Billable;
use config::Config;

fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("Cannot build config.")
        .try_deserialize::<billable::Config>()
        .expect("Failed to read configuration.");

    let billable = billable::reports::toggl::Billable::new(config.api_token);
    println!("{}", billable.report().expect("Failed to prepare report."));
}
