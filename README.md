# Billable

![status](https://github.com/robertwijas/billable-rust/actions/workflows/rust.yml/badge.svg)

## Todos 

- [x] show hours for current month per client
- [x] remove hardcoded month
- [x] show report for last 2 months
- [x] read user config file from $HOME/.billable.toml or similar
- [x] support arg for controlling the number of recent months to display
- [x] better time precision (or round up hours)
- [ ] use fixed hours in DEBUG mode 
- [ ] display hours in PLN AKA support hourly rates :)
- [ ] use basic text formatting when displaying results
- [ ] implement async http requests
- [ ] show estimated hours for current month per client
- [ ] configure monthly revenue goal
- [ ] indicate goal progress and likelihood of hitting the target
- [ ] auto currency conversion

## Notes 

### How to store configuration?

This looks good: https://docs.rs/config/latest/config/.
