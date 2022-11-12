# Billable

![status](https://github.com/robertwijas/billable-rust/actions/workflows/rust.yml/badge.svg)

## Todos 

- [x] show hours for current month per client
- [x] remove hardcoded month
- [x] show report for last 2 months
- [ ] better time precision (or round up hours)
- [ ] use fixed hours in DEBUG mode 
- [ ] display hours in PLN :)
- [ ] read config file from $HOME/.billable.toml or similar
- [ ] implement async http requests
- [ ] support arg for controlling the number of recent months to display
- [ ] show estimated hours for current month per client
- [ ] configure monthly revenue goal
- [ ] indicate goal progress and likelihood of hitting the target

## Notes 

### How to store configuration?

This looks good: https://docs.rs/config/latest/config/.
