# Billable

![status](https://github.com/robertwijas/billable-rust/actions/workflows/rust.yml/badge.svg)

A simple command line utility that displays monthly reports for billable hours.
It integrates with [Toggl](http://track.toggl.com/) and [Harvest](https://www.getharvest.com).

The key feature is the ability to set monthly goal for each client.
Billable provides monthly estimates, daily and weekly targets based on the definied goal.

## Configuration

Configuration file `config.toml` is loaded from:
* current directory
* or `~/Library/Application Support/com.robertwijas.billable`

Make sure the `config.toml` file is available in either one of these two locations. You can copy and modify the example configuation:
https://github.com/robertwijas/billable-rust/blob/712609831f2790c9fa3ad6bdc71198b7ec4c3bd1/config.toml.example#L1-L11

```bash

```


## Usage

```bash
billable --help
Usage: billable [OPTIONS]

Options:
  -m, --months <MONTHS>  [default: 1]
  -s, --show-minutes
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
- [x] add harvest
- [x] add demo reports provider
- [x] automatically create configuration if missing, based on the `config.toml.example`
- [ ] replace colored with console
- [ ] extract styling code using Style from console
- [ ] add example output to README (preferably based on the demo report)
- [ ] adjust first column width to the longest client name
- [ ] write better README
- [ ] create CHANGELOG

# Ideas

- [ ] support holidays (?) or allow to somehow override working days
- [ ] add option to display _Total_ row for all clients
- [ ] add option to show weekly reports
- [ ] configure automatic releases with github actions
- [ ] reports/configuration per project
- [ ] display hours in PLN AKA support hourly rates :)
- [ ] auto currency conversion
- [ ] implement async http requests

## Notes 

### How to store configuration?

This looks good: https://docs.rs/config/latest/config/.
