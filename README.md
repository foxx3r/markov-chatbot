# markov-chatbot
The source code of the chatbot @veriskobot on Telegram using markov chain.

## Installation

If you don't have the (rust)[https://www.rust-lang.org] compiler, just install it via (rustup toolchain)[https://rustup.rs]

> warning: I have not tested on stable rust yet
>> 2nd warning: this project may not run on Windows

## How to run

To run, just enter in the directory and run:

`$ cargo run`

Or if you wanna run with the release mode, execute:

`$ cargo run --release`

## Token

If you trynna run the bot, it will give you an error, to fix that, just run:

`$ export TELEGRAM_BOT_TOKEN=<your_token>`

Or put it into your `~/.zshrc` or `~/.bashrc`

## Run tests

### TODO

To run the tests, just execute:

`$ cargo test`

Or in release mode:

`$ cargo test --release`

## Notes

It will create a file named `chat.txt`, it will contain the whole conversation that you'll get. It's needed because it will be necessary to pass the messages to the markov chain network.
