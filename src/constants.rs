pub mod ml_kem_constants {
    pub const q: u16 = 3329;
    pub const n: usize = 256;

    pub const ENCODE_10: usize = n * 10 / 8;
    pub const ENCODE_12: usize = n * 12 / 8;
    pub const E_PKE_KEYSIZE: usize = k * ENCODE_12 + 32;
    pub const D_PKE_KEYSIZE: usize = k * ENCODE_12;
    // 32(d_u * k + d_v) - 32(d_u * k)
    pub const C2_SIZE: usize = (32 * (k * du + dv)) - 32 * (du * k);
    pub const MASK_12: u32 = 0b1111_1111_1111;

    pub const k: usize = 3;
    pub const du: usize = 10;
    pub const dv: usize = 4;
}

#[allow(non_camel_case_types)]
// appendix table 2
pub mod parameter_sets {
    use crate::math::encoding::EncodingSize;
    use typenum::{Unsigned, U10, U11, U12, U16, U2, U3, U4, U5, U9};

    pub trait ParameterSet {
        // Define each parameter as an associated type.
        type K: Unsigned;
        type KSquared: Unsigned;
        type EtaOne: Unsigned;
        type EtaTwo: Unsigned;
        type Du: EncodingSize;
        type Dv: EncodingSize;
        type Encode12: EncodingSize;
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct KEM_512;
    impl ParameterSet for KEM_512 {
        type K = U2;
        type KSquared = U4;
        type EtaOne = U3;
        type EtaTwo = U2;
        type Du = U10;
        type Dv = U4;
        type Encode12 = U12;
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct KEM_768;
    impl ParameterSet for KEM_768 {
        type K = U3;
        type KSquared = U9;
        type EtaOne = U2;
        type EtaTwo = U2;
        type Du = U10;
        type Dv = U4;
        type Encode12 = U12;
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct KEM_1024;
    impl ParameterSet for KEM_1024 {
        type K = U4;
        type KSquared = U16;
        type EtaOne = U2;
        type EtaTwo = U2;
        type Du = U11;
        type Dv = U5;
        type Encode12 = U12;
    }
}

/// Parameters for Barrett reduction
pub mod barrett_constants {
    pub const MULTIPLIER: u16 = 5039; // 4¹² / q,
    pub const SHIFT: u16 = 24; // log₂(4¹²)
}

/// obtained externally and verified in utils.rs
pub const K_NTT_ROOTS: [u16; 128] = [
    1, 1729, 2580, 3289, 2642, 630, 1897, 848, 1062, 1919, 193, 797, 2786, 3260, 569, 1746, 296,
    2447, 1339, 1476, 3046, 56, 2240, 1333, 1426, 2094, 535, 2882, 2393, 2879, 1974, 821, 289, 331,
    3253, 1756, 1197, 2304, 2277, 2055, 650, 1977, 2513, 632, 2865, 33, 1320, 1915, 2319, 1435,
    807, 452, 1438, 2868, 1534, 2402, 2647, 2617, 1481, 648, 2474, 3110, 1227, 910, 17, 2761, 583,
    2649, 1637, 723, 2288, 1100, 1409, 2662, 3281, 233, 756, 2156, 3015, 3050, 1703, 1651, 2789,
    1789, 1847, 952, 1461, 2687, 939, 2308, 2437, 2388, 733, 2337, 268, 641, 1584, 2298, 2037,
    3220, 375, 2549, 2090, 1645, 1063, 319, 2773, 757, 2099, 561, 2466, 2594, 2804, 1092, 403,
    1026, 1143, 2150, 2775, 886, 1722, 1212, 1874, 1029, 2110, 2935, 885, 2154,
];

/// obtained externally and verified in utils.rs
pub const K_MOD_ROOTS: [u16; 128] = [
    17, 3312, 2761, 568, 583, 2746, 2649, 680, 1637, 1692, 723, 2606, 2288, 1041, 1100, 2229, 1409,
    1920, 2662, 667, 3281, 48, 233, 3096, 756, 2573, 2156, 1173, 3015, 314, 3050, 279, 1703, 1626,
    1651, 1678, 2789, 540, 1789, 1540, 1847, 1482, 952, 2377, 1461, 1868, 2687, 642, 939, 2390,
    2308, 1021, 2437, 892, 2388, 941, 733, 2596, 2337, 992, 268, 3061, 641, 2688, 1584, 1745, 2298,
    1031, 2037, 1292, 3220, 109, 375, 2954, 2549, 780, 2090, 1239, 1645, 1684, 1063, 2266, 319,
    3010, 2773, 556, 757, 2572, 2099, 1230, 561, 2768, 2466, 863, 2594, 735, 2804, 525, 1092, 2237,
    403, 2926, 1026, 2303, 1143, 2186, 2150, 1179, 2775, 554, 886, 2443, 1722, 1607, 1212, 2117,
    1874, 1455, 1029, 2300, 2110, 1219, 2935, 394, 885, 2444, 2154, 1175,
];
