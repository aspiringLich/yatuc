# yauc

Yet another utilities crate. This crate provides re-exports of many crates and functions I commonly find myself using and also provides some of its own utility as well (see below).

## Features
### logging / style

Ever felt like the many, many, many other logging crates just weren't sufficient? Did you want yet another ANSI helper crate? No? Well here's my addition to the pile anyway.

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

### unwrap

Have you every written a function and were too lazy to have it return `Result<T, E>` but still wanted to use the `?` operator? I spent an hour figuring out `proc-macro` to make this and you will use it and like it.

```rs
use yauc::macros::unwrap;

#[unwrap]
fn normal_fn() -> i32 {
  let s = "does it detect this question mark? (no)";
  println!("{}", s);
  let x: Result<i32, ()> = Ok(23);
  return x?; // gets replaced with x.unwrap();
}

assert_eq!(normal_fn(), 23);
```
