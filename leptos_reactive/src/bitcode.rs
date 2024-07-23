use crate::{create_rw_signal, RwSignal, SignalWith};
use bitcode::{
    Decode, Encode,
    __private::{Encoding, Read, Write},
};

/* Serialization for signal types */

impl<T: Encode> Encode for RwSignal<T> {
    const ENCODE_MIN: usize = T::ENCODE_MIN;
    const ENCODE_MAX: usize = T::ENCODE_MAX;

    fn encode(
        &self,
        encoding: impl Encoding,
        writer: &mut impl Write,
    ) -> Result<(), bitcode::Error> {
        self.with(|value| value.encode(encoding, writer))
    }
}

/* Deserialization for signal types */

impl<T: Decode> Decode for RwSignal<T> {
    const DECODE_MIN: usize = T::DECODE_MIN;
    const DECODE_MAX: usize = T::DECODE_MAX;

    fn decode(
        encoding: impl Encoding,
        reader: &mut impl Read,
    ) -> Result<Self, bitcode::Error> {
        T::decode(encoding, reader).map(create_rw_signal)
    }
}
