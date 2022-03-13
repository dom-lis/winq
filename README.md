# winq

Winq is a graphical utility that opens a window and draws text to it

## Why?

Winq is intended to build applications with text-based user interface.

This project is an application that would communicate with another process and draw it's interfaces, communicated with
Winq using special protocol in either simple, `sscanf`-friendly format, or as new-line separated json objects.

Initially Winq (then known as Tulip) was a terminal application. I since reconsidered my approach and now the
tool is, to some extent, it's own terminal emulator. This decision was made largely because of poor terminals'
support for localization, specifically difficulties with different (non en-us) layouts, as well as archaic, sequential,
stateful ways of communication.

### Examples

You can run examples with a command like

```
python examples/(desired example).py
```

[GNU GPLv3](LICENSE)
