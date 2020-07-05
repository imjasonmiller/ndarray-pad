# ndarray-pad

[![Coverage](https://img.shields.io/codecov/c/github/imjasonmiller/ndarray-pad?style=social)](https://codecov.io/gh/imjasonmiller/ndarray-pad)

This crate provides padding for
[`ndarray`](https://github.com/rust-ndarray/ndarray)'s `ArrayBase` type. It
aims to achieve feature parity with Python's
[`numpy.pad`](https://numpy.org/doc/stable/reference/generated/numpy.pad.html).

Current padding modes include:
- Constant
- Edge, also known as extend or clamp

## Usage with Cargo

```toml
[dependencies]
ndarray = "0.13"
ndarray-pad = "0.1"
```

## Contributing 

Please feel free to create issues and submit pull requests.

## License 

Licensed under the [Apache License, Version 2.0](LICENSE-APACHE), or the [MIT
license](LICENSE-MIT), at your option. You may not use this project except in
compliance with those terms.

