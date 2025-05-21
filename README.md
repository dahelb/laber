# Laber

A simple TCP based client-server based chat protocol and its server implementation in Rust.

Run the server:

```
$ cargo run --bin server
```

This will start the chat server on socket 127.0.0.1:9999.

Then, start one or multiple clients and start chatting:

```
$ cargo run --bin client
```

## Protocol

The current ABNF:

```
message  = "MSG" SP user SP text CRLF
error    = "ERROR" SP text CRLF 
system   = "SYSTEM" SP text CRLF 

user     = 1*18VCHAR

text     = *OCTET   ; UTF-8 encoded text

SP       = %x20
CRLF     = %x0D.0A
```

