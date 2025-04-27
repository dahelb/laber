# Laber

A simple TCP based client-server based chat protocol and its server implementation in Rust.

Run the server:

```
$ cargo run
```

This will start the chat server on socket 127.0.0.1:9999.


Then, connect one or multiple instances of ncat or another netcat derivate of your choice in CRLF mode:


```
$ netcat -C localhost 9999
SYSTEM Welcome to the chat server!
```

Start sending and receiving messages to all connections using `MSG <user> <message>`.

```
MSG dave hello, world!
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

