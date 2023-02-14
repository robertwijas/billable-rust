# Billable

![status](https://github.com/robertwijas/billable-rust/actions/workflows/rust.yml/badge.svg)

## Configuration

Configuration file `config.toml` is loaded from:
* current directory
* or `~/Library/Application Support/com.robertwijas.billable`

Make sure the `config.toml` file is available in either one of these two locations. You can copy and modify the example configuation:
https://github.com/robertwijas/billable-rust/blob/d96c71af64358c2ff8deb365473f0783401bc59f/config.toml.example#L1-L8

## Usage

```bash
billable --help
Usage: billable [OPTIONS]

Options:
  -m, --months <MONTHS>  [default: 1]
  -h, --help             Print help
```

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
- [x] add option to show time with minutes
- [x] calculate optimal weekly and daily pace to hit the goal
- [x] support show minutes flag in goal hints
- [ ] adjust first column width to the longest client name
- [ ] add option to display _Total_ row for all clients
- [ ] add option to show weekly reports
- [ ] support holidays (?) or allow to somehow override working days
- [ ] configure automatic releases with github actions
- [ ] reports/configuration per project
- [ ] write better README
- [ ] use fixed hours in DEBUG mode 
- [ ] display hours in PLN AKA support hourly rates :)
- [ ] auto currency conversion
- [ ] implement async http requests

## Notes 

### How to store configuration?

This looks good: https://docs.rs/config/latest/config/.
