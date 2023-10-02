# matching-engine-rs

[WIP]: Some of the implementations are buggy and missing as well.

This is an attempt to implement a matching engine with Rust. At present, the project is organized into two main libraries: `itch-parser` and `optimized-lob`. The `itch-parser` is responsible for managing the processing of *NASDAQ ITCH 5.0* protocol data, while the `optimized-lob` library offers a streamlined and efficient implementation of a Limit Order Book (LOB). It's worth mentioning that I've made some specific design choices and adaptations for the Limit Order Book. Please note that the `optimized-lob` library calculates only the aggregate quantities at each price level and does not track the queue depth for each individual order.

## Performance

### ITCH Processing

```text
ITCH Message Processing

Total Messages: 268744780
Total Time: 8.266 seconds
Speed: 33593097 messages per second
Latency: 30 ns
```

### LOB Performance

```text
ITCH Message Processing

Total Messages: 268744780
ITCH Latency: 94 ns
Total Time: 25.326 seconds
Speed: 10749791 msg/second
Total Add Orders: 118631456
Total Execute Orders: 5822741
Total Cancel Orders: 2787676

```
## ITCH Specifications

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
- A [StackOverflow answer](https://quant.stackexchange.com/questions/3783/what-is-an-efficient-data-structure-to-model-order-book/32482#32482) along with his implementation of a [optimized LOB](https://github.com/charles-cooper/itch-order-book/)
- This [blog post](https://web.archive.org/web/20110219163448/http://howtohft.wordpress.com/2011/02/15/how-to-build-a-fast-limit-order-book/) gives a good idea for the low-level design of the orderbook.

Apart from that, the implementation in the [itchy-rust](https://github.com/adwhit/itchy-rust) library was helpful to create the ITCH Parser. The `nom` library used in the library was pretty old, so I created a parser using the updated libraries and as suitable to my project.

It's important to note that the parsing logic employed within my ITCH parser is optimized for a subset of functions relevant to the Limit Order Book implementation. For broader parsing requirements, it is recommended to utilize the comprehensive capabilities offered by the `itchy-rust` library itself.
## License

This project is licensed under the [MIT License](LICENSE).
