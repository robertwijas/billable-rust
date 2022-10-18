pub mod toggl;
use std::{fmt::Display, time::Duration};

pub trait Billable {
    fn report(&self) -> Result<Report, BillableError>;
}

#[derive(Debug)]
pub enum BillableError {
    Default,
}

#[derive(Debug)]
pub struct Report {
    duration: Duration,
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Total time: {:?}", self.duration)
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
            .map_err(|_| BillableError::Default)
        // self.service
        //     .get(toggl::Endpoint::time_entries())
        //     .map(|l| {
        //         let seconds: i32 = l.iter().map(|x| x.duration).sum::<i32>().into();
        //         Report {
        //             duration: Duration::from_secs(seconds as u64),
        //         }
        //     })
        //     .map_err(|_| BillableError::Default)
    }
}
