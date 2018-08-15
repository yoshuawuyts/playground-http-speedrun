# playground-http-speedrun
Timing how long it takes to build HTTP services in Rust.

## Timings
```txt
01  Hello World       17m
02  Signup Login   02h50m
03  Hello World       14m
```

## Notes
### [01 - Hello World](./http_01)
- Spent about 10 minutes trying to get `clap_flags` to be picked up. Turns out
  for some reason it must also be specified as a build dependency. The help
  message wasn't helpful at all.
- Most of the HTTP code was copied from an existing example, and not written
  from scratch. That probably sped up things a lot.
- We didn't add a router. We should probably add _some_ routing next time.

### [02 - Signup Login](./http_02)
- Used an `Arc<Mutex<HashMap>>` as the database.
- This is a pretty great pattern to emulate any more difficult databases. Maybe
  we should make that a crate? Memdb? Testdb? Something like that.
- I'm not sure how to handle errors in Warp yet. Maybe we need to specify a
  return type to handle it correctly.
- similarly I don't know how to return a filter from a function. That took a
  while to figure out, and eventually had to abandon it.
- Found out that Argon2 stores the Salt in the hash itself, so there's only one
  value that needs to be stored. That's really nice tbh!
- We should have a standard logger thing for info. Couldn't get the built-in
  warp logger to work, but didn't spend too much time on this either haha.

### [03 - Hello World](./http_03)
- It's not great that we can't call `server.local_addr()`; apparently something
  is wrong with traits that we need to figure out there.
- I would like us to be able to get away from having to import
    `futures::prelude::*` ideally. But I guess there's not really escaping it.
- It's a bit annoying that `clap_flags`'s `verbosity` is set to `Error` by
  default. `Info` seems like a better default.
- There's a lot of imports required to get a basic hyper server working. It'd be
  nice if we would require fewer.
- Very much looking forward to having async/await streams at some point in the
  future in Rust. Having to use combinators still feels a bit awkward to me.
- I'm not sure how to close a server if an error limit is exceeded in a time
  frame. All we can do for now is log a bunch of things.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)
