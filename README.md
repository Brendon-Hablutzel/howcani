# howcani

A generative AI-powered CLI for getting step-by-step instructions for how to perform various tasks using the command line.

This project has been implemented in pure Rust using the clap [crate](https://crates.io/crates/clap).

## Use

To install, ensure you have `cargo` on your system and run `cargo install howcani`.

The CLI requires a generative AI model as a backend to use for producing the instructions. As of now, there is only one available: [Cohere](https://cohere.com/).

## Credentials

Before using the CLI, you must set the credentials required for the chosen model. These should be specified in a toml config file called `howto-cli-creds.toml` in the user's configuration directory (for example, on most Linux systems, the config file would be located at `/home/user/.config/howto-cli-creds.toml`).

Example:

```toml
cohere_api_key = "key"
```
