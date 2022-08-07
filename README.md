# Fan-out Links
### Context
Every week I visit the family and share funny reddit posts to non-redditors. <br/>
The process involves sending all the links via discord and manually clicking them all to open. <br/>
This cli tool allows the user to run a single command to open all the links for the week.
### Description
A small cli app that opens newline seperated links. <br/>
Opens links in your default browser from a url endpoint or from a cmdline argument.
### Usage
From url endpoint (in development):
```bash
fl config
# checks current url endpoint

fl config https://my-endpoint-with-links.com
fl open
# opens all links received from endpoint
```

From cmdline:
```bash
fl open "https://doc.rust-lang.org/book/
https://github.com/rust-lang/book"
# opens two tabs in my default browser
```

### Installation
