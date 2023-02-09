# Billable

![status](https://github.com/robertwijas/billable-rust/actions/workflows/rust.yml/badge.svg)

## Todos 

- [x] show hours for current month per client
- [x] remove hardcoded month
- [x] show report for last 2 months
- [x] read user config file from $HOME/.billable.toml or similar
- [x] support arg for controlling the number of recent months to display
- [x] better time precision (or round up hours)
- [x] configure monthly goal
- [x] use basic text formatting when displaying results
- [x] show estimated hours for current month per client
- [x] indicate goal progress
- [ ] add option to show weekly reports
- [ ] configure automatic releases with github actions
- [ ] write better README
- [ ] use fixed hours in DEBUG mode 
- [ ] display hours in PLN AKA support hourly rates :)
- [ ] auto currency conversion
- [ ] implement async http requests

## Notes 

### How to store configuration?

This looks good: https://docs.rs/config/latest/config/.
