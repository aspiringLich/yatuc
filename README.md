# yauc

Yet another utilities crate. This crate provides re-exports of many crates and functions I commonly find myself using and also provides some of its own utility as well (see below).

## Features
### logging

Ever felt like the many, many, many other logging crates just weren't sufficient? No? Well here's my addition to the pile anyway.

These are literally the first actual macros ive ever written so they're probably absolutely terrible.

- [x] `style_print!();`
- [ ] time zones (macro will auto-adjust based on OS's time zone configuration if applicable)
- [ ] provide logging to a file

```rs
// shrimply:
info!(("i have {} fingers", 10), blue);
warn!(
  ("sounds like its almost time to... "), 
  ("panic!", red),
);
error!("AAAAAAAA");
/* to get (imagine if it was also colored correctly):
[22/11/17 16:53:09.927]INFO: i have 10 fingers
[22/11/17 16:53:09.928]WARN: sounds like its almost time to... panic!
[22/11/17 16:53:09.930]ERROR: AAAAAAAA
*/
```