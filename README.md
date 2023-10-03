# matching-engine-rs
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-v1.0.0-blue.svg)](https://semver.org/)
[![GitHub Stars](https://img.shields.io/github/stars/amankrx/matching-engine-rs?logo=github&label=Stars&color=yellow)](https://github.com/amankrx/matching-engine-rs)

This is an attempt to implement a matching engine with Rust. Currently, I have created an implementation of a Limit Order Book.

## Table of Contents
- [Project Structure](#project-structure)
- [Build, Run, and Test](#build-run-and-test)
- [Device Specifications](#device-specifications)
- [Performance](#performance)
- [ITCH Specifications](#itch-specifications)
- [Contributing](#contributing)
- [Credits](#credits)
- [License](#license)

## Project Structure
These project consists of two libraries:
- **[itch-parser](itch-parser)**: This library is responsible for managing the processing of *NASDAQ ITCH 5.0* protocol data. It parses the useful fields that will be required for the Limit Order Book. The remaining fields are skipped using placeholders. Check out the folder's [README](itch-parser/README.md) for more information.
- **[optimized-lob](optimized-lob)**: This library contains a streamlined and efficient implementation of a Limit Order Book (LOB). It is worth noting that the LOB simply stores a few useful fields that will be required for creating a LOB. It just keeps an aggregate quantities at each level. Check out the folder's [README](optimized-lob/README.md) for more information.

Apart from that, there is a testing suite for both libraries that can be found in the "[tests](tests)" directory. 

## Build, Run, and Test
Make sure you have Rust installed. Also, you must download the NASDAQ ITCH 5.0 data whose instructions are available in the [ITCH Specifications](#ITCH-Specifications). 
All of these operations are performed in the `tests` directory. 
```bash
cd tests
```
### Build
```bash
cargo build
```
or
```bash
cargo build --release
```

### Running the LOB
```bash
ITCH_DATA=PATH_TO_ITCH_DATA_FILE cargo run
```
or
```bash
ITCH_DATA=PATH_TO_ITCH_DATA_FILE cargo run --release
```

### Running the ITCH parser
```bash
ITCH_DATA=PATH_TO_ITCH_DATA_FILE cargo run -- --itch-parser
```
or
```bash
ITCH_DATA=PATH_TO_ITCH_DATA_FILE cargo run --release -- --itch-parser
```

### Testing
```bash
cargo test
```


## Device Specifications
At the time of testing:
```text
Device: MacBook Air M2
CPU architecture: Apple M2
CPU logical cores: 8
CPU physical cores: 8
RAM total: 16 GB
RAM free: 11.5 GB
OS bits: 64-bit
```
## Performance

### ITCH Processing

```text
ITCH Parser Processing...

Success...

ITCH Parsing Statistics:
Total Messages: 268744780
Total Time: 6.082 seconds
Speed: 44189583 msg/second
Latency: 22 ns
```

### LOB Performance

```text
LOB Processing...

Success...

Performance Metrics:
Total Messages: 268744780
ITCH Latency: 88 ns
Total Time: 23.660 seconds
Speed: 11358746 msg/second

Orderbook Statistics:
Total Add Orders: 118631456
Total Execute Orders: 5822741
Total Cancel Orders: 2787676
Total Delete Orders: 114360997
Total Replace Orders: 21639067
```
## ITCH Specifications
<!-- itch-specs -->
The project follows the `Nasdaq TotalView-ITCH 5.0` standard for the processing of data.

- [Protocol Specifications](http://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHSpecification.pdf)
- [Binary Specification File](http://www.nasdaqtrader.com/content/technicalSupport/specifications/dataproducts/binaryfile.pdf)
- ITCH data can be downloaded from their website: https://emi.nasdaq.com/ITCH/Nasdaq%20ITCH/

I have specifically used their `12302019.NASDAQ_ITCH50` data whose compressed file can be downloaded from [here](https://emi.nasdaq.com/ITCH/Nasdaq%20ITCH/12302019.NASDAQ_ITCH50.gz).
## Contributing

Contributions to matching-engine-rs are welcome! If you encounter any issues, have suggestions, or would like to add new features, please feel free to open an issue or submit a pull request. Note that I'm still learning my way around Rust and trading systems, so any feedback is appreciated!

## Credits

These are a few useful resources that helped me and will be useful to understand the LOB as well. Most of them are primarily written in C/C++.
- [CppTrader](https://github.com/chronoxor/CppTrader) matching engine implementation
- A [StackOverflow answer](https://quant.stackexchange.com/questions/3783/what-is-an-efficient-data-structure-to-model-order-book/32482#32482) along with his implementation of an [optimized LOB](https://github.com/charles-cooper/itch-order-book/)
- This [blog post](https://web.archive.org/web/20110219163448/http://howtohft.wordpress.com/2011/02/15/how-to-build-a-fast-limit-order-book/) gives a good idea for the low-level design of the orderbook.

Apart from that, the implementation in the [itchy-rust](https://github.com/adwhit/itchy-rust) library was helpful to create the ITCH Parser. The `nom` library used in the library was pretty old, so I created a parser using the updated libraries and as suitable to my project.

It's important to note that the parsing logic employed within my ITCH parser is optimized for a subset of functions relevant to the Limit Order Book implementation. For broader parsing requirements, it is recommended to utilize the comprehensive capabilities offered by the `itchy-rust` library itself.
## License

This project is licensed under the [MIT License](LICENSE).
