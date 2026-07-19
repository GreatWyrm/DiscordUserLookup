# Discord User Lookup

A simple program written in Rust to fetch user details from the Discord API.

## Requirements

This can be built using cargo, though to properly contact the Discord's API you will need a bot token. One can be obtained through creating an application on the [Discord Developer Website](https://discord.com/developers/applications/). One obtained, you will need to provide it as an environment variable or in a file called BOT_TOKEN.txt.

## Running

There are 3 ways to run this program:

- Without command line arguments, the program will launch a window that you can interact with.
- With the --lookup parameter, the next argument will be converted to a user id and looked up, with the details being printed out on standard out.
- With the --file parameter, the next argment will be a file to read in and split up to find user ids, and then those user ids will be looked up and printed out.