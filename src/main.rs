use billable::Billable;
use config::Config;

fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    let api_token = config
        .try_deserialize::<std::collections::HashMap<String, String>>()
        .expect("Failed to read configuration.")
        .get("api_token")
        .expect("Failed to read API token from configuration.")
        .clone();

    let billable = billable::TogglBillable::new(api_token);
    println!("{:?}", billable.report())
}
