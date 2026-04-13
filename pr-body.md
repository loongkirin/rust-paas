Implement feature issue #3: add login and logout command to paas-cli

## Changes
- Add login subcommand with --username and --password flags
- Add logout subcommand
- Both commands print status messages (actual logic to be implemented)

## Testing
cargo run -p paas-cli -- login --username user --password pass
cargo run -p paas-cli -- logout
