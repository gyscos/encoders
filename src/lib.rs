//! This crate provides two traits to abstract over streaming compression formats.
#![deny(missing_docs)]

use std::io;

/// Trait for a streaming encoder.
///
/// This trait is generic over the wrapped writer.
pub trait Encoder<W>: io::Write {

    /// Parameters required to create a new encoder.
    ///
    /// This is typically the compression level.
    type Params;

    /// Wraps an existing writer, using the given parameters.
    fn with_params(inner: W, params: Self::Params) -> io::Result<Self> where Self: Sized;

    /// Finish the stream, writing the necessary footer.
    ///
    /// Returns the wrapped writer if everything went well.
    fn finish(self) -> io::Result<W> where Self: Sized;
}

/// Trait for a streaming decoder.
pub trait Decoder<R>: io::Read {

    /// Parameters required to create a new decoder.
    ///
    /// Many decoders may use `()`.
    type Params;

    /// Wraps an existing reader, using the given parameters.
    fn with_params(inner: R, params: Self::Params) -> io::Result<Self> where Self: Sized;

    /// Finish the stream, consuming the footer if necessary.
    ///
    /// Returns the wrapped reader if everything went well.
    fn finish(self) -> io::Result<R> where Self: Sized;
}
