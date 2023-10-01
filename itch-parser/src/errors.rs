// errors.rs

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Nom(::nom::Err<u8>);
    }
}
