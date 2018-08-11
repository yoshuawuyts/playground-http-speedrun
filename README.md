# playground-http-speedrun
Timing how long it takes to build HTTP services in Rust.

## Timings
```txt
01  Hello World       17m
02  Signup Login   02h50m
```

## Notes
### 01 - Hello World
- Spent about 10 minutes trying to get `clap_flags` to be picked up. Turns out
  for some reason it must also be specified as a build dependency. The help
  message wasn't helpful at all.
- Most of the HTTP code was copied from an existing example, and not written
  from scratch. That probably sped up things a lot.
- We didn't add a router. We should probably add _some_ routing next time.

### 02 - Signup Login
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

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)
