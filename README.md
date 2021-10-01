# Ping-pong client-daemon example

This makes the _client_ expose an interface on the bus, and after it has
communicated with the daemon that listens on a well known name, the daemon
calls a method on the client before returning a reply.

In my use-case, sometimes the daemon wants a bit of bulky "more data" from a
client, but rarely enough that it's not a good idea to have the client always
supply that data.

I was looking at things like, "return a hint that the server could use some
more data",  when I realized that it might just work both ways at the same
time.

And it works.
