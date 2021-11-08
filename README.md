<!-- @format -->

# Shortman!

A link shortner written in Rust! Just make a simple post request and get back a shorterned url!
Shortman uses Redis as a key value store for the url redirects!

![rust logo](https://cloud-h7u3cr6bj-hack-club-bot.vercel.app/0image.png)

## API

Shortman can be hosted on any VPS, it requires a nightly version of RUST.

After hosting it, you can make a POST request to `/` path with a raw JSON Body containing url as a string!

example:
request(POST): http://127.0.0.1:8000/
body(JSON): {"https://link*that_you_want_to_short*.com"}

response => {"VaT23m2l8pDlEVn"}

It will return a token and and now request to http://127.0.0.1:8000/VaT23m2l8pDlEVn will redirect to https://link*that_you_want_to_short*.com

## Commands and Environemnts

### run command

`rustup override set nightly && cargo run`

### Environment Varialbe

It only expect a single environment variable, by the name of `CONNECTION_URL` {Redis Connection URL!}.

example usage:
`ROCKET_CONNECTION_URL=redis://connection_url:port_number cargo run `

In the above example it is assumed that the nightly verion of Rust is being used!
