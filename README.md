
# FastCpy

The Rust Compiler calls `memcpy` for slices of unknown length. There's considerable overhead for that in some cases.
If you know most of you copy operations are not too big you can use `fastcpy` to speed up your program.

`fastcpy` provides a faster implementation of `memcpy` for slices up to 32bytes (64bytes with `avx`).

It is designed to contain not too much assembly, so the overhead is low.

As fall back the standard `memcpy` is called

## Double Copy Trick
`fastcpy` employs a double copy trick to copy slices of length 4-32bytes (64bytes with `avx`).
E.g. Slice of length 6 can be copied with two uncoditional copy operations.
```
/// [1, 2, 3, 4, 5, 6]
/// [1, 2, 3, 4]
///       [3, 4, 5, 6]
```



