# Billable

![status](https://github.com/robertwijas/billable-rust/actions/workflows/rust.yml/badge.svg)

A simple command line utility that displays monthly reports for billable hours.
It integrates with [Toggl](http://track.toggl.com/) and [Harvest](https://www.getharvest.com).

The key feature is the ability to set monthly goal for each client.
Billable provides monthly estimates, daily and weekly targets based on the definied goal.

## Output example

```bash
# billable -s -m 2
[Toggl]
February 2023
My Second Client        19:00 🟢 23:28/20:00 🎯 0:15 a day, 1:15 a week
My First Client         23:00 🔴 28:24/30:00 🎯 1:45 a day, 8:45 a week
January 2023
My Second Client        30:00 🟢 30:00/20:00
My First Client         17:00 🔴 17:00/30:00

[Harvest]
February 2023
Harvest Client          27:00 🟢 33:21/33:00 🎯 1:30 a day, 7:30 a week
January 2023
Harvest Client          35:00 🟢 35:00/33:00
```

## Installation

You can install `billable` with [cargo](https://github.com/rust-lang/cargo):
```
cargo install billable
```

## Configuration

Configuration file `config.toml` is loaded from:
* current directory
* or `~/Library/Application Support/com.robertwijas.billable`

Running `billable` presents and option to create a sample config.

## Usage

```bash
# billable --help
Usage: billable [OPTIONS]

Options:
  -m, --months <MONTHS>            [default: 1]
  -s, --show-minutes
  -c, --config-name <CONFIG_NAME>  [default: config]
  -h, --help                       Print help
```

## Todos/Features

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
- [x] add example output to README (preferably based on the demo report)
- [ ] replace colored with console
- [ ] extract styling code using Style from console
- [ ] adjust first column width to the longest client name
- [ ] create CHANGELOG
- [ ] support holidays (?) or allow to somehow override working days
- [ ] add option to display _Total_ row for all clients
- [ ] add option to show weekly reports
- [ ] configure automatic releases with github actions
- [ ] reports/configuration per project
- [ ] display hours in PLN AKA support hourly rates :)
- [ ] auto currency conversion
- [ ] implement async http requests

