## Junk

> Filter out OS junk files like .DS_Store and Thumbs.db. A Rust implementation of https://github.com/sindresorhus/junk

## API

```rust
use junk;
```

### junk::is(&path);

Returns `true` if `filename` matches a junk file.

### junk::not(&path);

Returns `true` if `filename` doesn't match a junk file.

### junk::REGEX

Regex used for matching junk files.
