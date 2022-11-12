use billable::reports::Billable;
use billable::reports::Month;
use config::Config;

fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("Cannot build config.")
        .try_deserialize::<billable::Config>()
        .expect("Failed to read configuration.");

    let billable = billable::reports::toggl::Billable::new(config.api_token);
    report(&billable, Month::current());
    report(&billable, Month::current().previous());
}

fn report(billable: &dyn Billable, month: Month) {
    println!("{}", month);
    println!(
        "{}",
        billable
            .report(month.into())
            .expect("Failed to prepare report.")
    );
}
