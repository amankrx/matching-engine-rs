# ITCH Parser
A nom-based Rust parser for parsing NASDAQ ITCH Protocol 5.0 based data. 
I initially intended to use the [itchy-rust](https://github.com/adwhit/itchy-rust) library directly which is in itself a robust way to handle the ITCH data, but much of the libraries that are used in it are outdated, and maybe rejected in future versions. 
For eg: it still uses `v4.x` of nom, when `v7.x` are available now. 
But still though much of the logic 

For a robust parser

Note: The parser is incomplete since I just included a few useful operations for my optimized-lob. However adding the remaining data won't be much difficult since you'll have to 