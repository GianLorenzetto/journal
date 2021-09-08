# Journal

Simple, Rust based CLI application for entering daily journal notes. 

# Running

```bash
$ cargo [build|test|run]
```

# Usage

At the prompt enter a command in the format

```
add {date} {duration} {summary}
```

Where
- **duration** is in the format `10mins`
- **date** is in the format "yy-mm-dd" or one of `[today|tomorrow|yesterday]`
- **summary** any non-empty string

The duration is parsed using the [duration_parse crate](https://docs.rs/parse_duration/2.1.1/parse_duration/), although for now it only supports durations without spaces (eg, "1hour", but not "1 hour").

There is no loop or backend, this is just an experiment or get the input parsing working and to learn some Rust :)
