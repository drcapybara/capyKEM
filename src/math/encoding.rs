use hybrid_array::typenum::Unsigned;

use hybrid_array::typenum::{
    operator_aliases::{Gcf, Prod, Quot},
    type_operators::Gcd,
    U32, U8,
};

use crate::math::util::Truncate;
use core::{
    fmt::Debug,
    ops::{Div, Mul},
};

use super::ring_element::RingElement;
/// An array length with other useful properties
pub trait ArraySize: hybrid_array::ArraySize + PartialEq + Debug {}

impl<T> ArraySize for T where T: hybrid_array::ArraySize + PartialEq + Debug {}
/// An integer that can be used as a length for encoded values.
pub trait EncodingSize: ArraySize {
    type EncodedPolynomialSize: ArraySize;
    type ValueStep: ArraySize;
    type ByteStep: ArraySize;
}

type EncodingUnit<D> = Quot<Prod<D, U8>, Gcf<D, U8>>;

impl<D> EncodingSize for D
where
    D: ArraySize + Mul<U8> + Gcd<U8> + Mul<U32>,
    Prod<D, U32>: ArraySize,
    Prod<D, U8>: Div<Gcf<D, U8>>,
    EncodingUnit<D>: Div<D> + Div<U8>,
    Quot<EncodingUnit<D>, D>: ArraySize,
    Quot<EncodingUnit<D>, U8>: ArraySize,
{
    type EncodedPolynomialSize = Prod<D, U32>;
    type ValueStep = Quot<EncodingUnit<D>, D>;
    type ByteStep = Quot<EncodingUnit<D>, U8>;
}
pub type Integer = u16;
fn byte_decode<D: EncodingSize>(bytes: &[u8]) -> RingElement {
    let val_step = D::ValueStep::USIZE;
    let byte_step = D::ByteStep::USIZE;
    let mask = (1 << D::USIZE) - 1;

    let mut vals = RingElement::zero();

    let vc = vals.coefs.chunks_mut(val_step);

    dbg!(&vc);

    let bc = bytes.chunks(byte_step);
    for (v, b) in vc.zip(bc) {
        let mut xb = [0u8; 16];
        xb[..byte_step].copy_from_slice(b);

        let x = u128::from_le_bytes(xb);
        for (j, vj) in v.iter_mut().enumerate() {
            let val: Integer = (x >> (D::USIZE * j)).truncate();
            vj.set(val & mask);

            if D::USIZE == 12 {
                vj.reduce_once();
            }
        }
    }
    vals
}

// Algorithm 4 ByteEncode_d(F)
//
// Note: This algorithm performs compression as well as encoding.
fn byte_encode<D: EncodingSize>(vals: &RingElement) -> Vec<u8> {
    let val_step = D::ValueStep::USIZE;
    let byte_step = D::ByteStep::USIZE;

    let mut bytes = vec![];

    let vc = vals.coefs.chunks(val_step);
    let bc = bytes.chunks_mut(byte_step);
    for (v, b) in vc.zip(bc) {
        let mut x = 0u128;
        for (j, vj) in v.iter().enumerate() {
            x |= u128::from(vj.val()) << (D::USIZE * j);
        }

        let xb = x.to_le_bytes();
        b.copy_from_slice(&xb[..byte_step]);
    }

    bytes
}

pub trait Encode<D: EncodingSize> {
    type EncodedSize: ArraySize;
    fn encode(&self) -> Vec<u8>;
    fn decode(enc: &[u8]) -> Self;
}

impl<D: EncodingSize> Encode<D> for RingElement {
    type EncodedSize = D::EncodedPolynomialSize;

    fn encode(&self) -> Vec<u8> {
        byte_encode::<D>(self)
    }

    fn decode(enc: &[u8]) -> Self {
        byte_decode::<D>(enc)
    }
}

// A convenience trait to allow us to associate some constants with a typenum
pub trait CompressionFactor: EncodingSize {
    const POW2_HALF: u32;
    const MASK: Integer;
}

impl<T> CompressionFactor for T
where
    T: EncodingSize,
{
    const POW2_HALF: u32 = 1 << (T::USIZE - 1);
    const MASK: Integer = ((1 as Integer) << T::USIZE) - 1;
}

// Traits for objects that allow compression / decompression
pub trait Compress {
    fn compress<D: CompressionFactor>(&mut self) -> &Self;
    fn decompress<D: CompressionFactor>(&mut self) -> &Self;
}

impl Compress for RingElement {
    fn compress<D: CompressionFactor>(&mut self) -> &Self {
        for x in &mut self.coefs {
            x.compress::<D>();
        }

        self
    }

    fn decompress<D: CompressionFactor>(&mut self) -> &Self {
        for x in &mut self.coefs {
            x.decompress::<D>();
        }

        self
    }
}
