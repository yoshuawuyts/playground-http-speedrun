# playground-http-speedrun
Timing how long it takes to build HTTP services in Rust.

## Timings
```txt
01  Hello World    17m
```

## Notes
### 01 - Hello World
- Spent about 10 minutes trying to get `clap_flags` to be picked up. Turns out
  for some reason it must also be specified as a build dependency. The help
  message wasn't helpful at all.
- Most of the HTTP code was copied from an existing example, and not written
  from scratch. That probably sped up things a lot.
- We didn't add a router. We should probably add _some_ routing next time.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)
