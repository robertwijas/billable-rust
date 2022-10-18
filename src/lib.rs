pub mod toggl;
use std::fmt::Display;

pub trait Billable {
    fn report(&self) -> Result<Report, BillableError>;
}

#[derive(Debug)]
pub enum BillableError {
    Default,
}

#[derive(Debug)]
pub struct Report {
    total: Vec<(Client, i32)>,
}

#[derive(Debug)]
pub struct Client {
    name: String,
    // rate: u8, // TODO: implement currencies
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.total.iter().for_each(|c| {
            writeln!(f, "{}: {}h", c.0.name, c.1).unwrap();
        });
        Ok(())
    }
}

pub struct TogglBillable {
    service: toggl::Service,
}

impl TogglBillable {
    pub fn new(api_token: String) -> Self {
        TogglBillable {
            service: toggl::Service::new(api_token),
        }
    }
}

impl Billable for TogglBillable {
    fn report(&self) -> Result<Report, BillableError> {
        self.service
            .get(toggl::Endpoint::me())
            .and_then(|user| {
                self.service.get(toggl::Endpoint::client_summary_report(
                    user.default_workspace_id.to_string(),
                ))
            })
            .map(|summary| Report {
                // TODO: implement From conversion?
                total: summary
                    .data
                    .iter()
                    .map(|x| {
                        let client = Client {
                            name: x.title.client.clone(),
                        };
                        (client, x.time / (60 * 60 * 1000))
                    })
                    .collect(),
            })
            .map_err(|e| {
                println!("{:?}", e);
                BillableError::Default
            })
    }
}
