# tulip

Tulip is a command-line utility intended to be used as front-end for tui
applications.

Tulip runs executable as a child process and communicates with it through
anonymous pipe.

## Why?

Every modern language can do json and asynchronous i/o, but working with
terminals &mdash; not really. There are some libraries but I am never satisfied
with those.

Hence this project &mdash; a terminal user interface proxy.

Tulip sends terminal input events into a client application via stdin. It also
accepts client application's output and displays it on the screen.

### Plans for future

- [ ] Graceful shutdown (especially when receiving signals)
- [x] Colors
- [x] Screen buffering/diffing (done by tui-rs)
- [ ] Key codes (for clients to be able to recognise same key in different layouts)

## License

[MIT](LICENSE)
