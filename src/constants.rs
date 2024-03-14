#[allow(dead_code)]
pub mod ml_kem_constants {
    pub const Q: u16 = 3329;
    pub const N: u16 = 256;
    pub const ENCODE_SIZE_12: u16 = N * 12 / 8;
}

// appendix table 2
#[allow(dead_code)]

pub mod parameter_sets {

    // Requireed random bit generator strength
    // 512 -> RBG(128)
    // 768 -> RBG(192)
    // 1024 -> RBG(256)

    pub trait ParameterSet {
        const K: u16;
        const ETA_ONE: u16;
        const ETA_TWO: u16;
        const Du: u16;
        const Dv: u16;
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct P512;
    impl ParameterSet for P512 {
        const K: u16 = 2;
        const ETA_ONE: u16 = 3;
        const ETA_TWO: u16 = 2;
        const Du: u16 = 10;
        const Dv: u16 = 4;
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct P768;
    impl ParameterSet for P768 {
        const K: u16 = 3;
        const ETA_ONE: u16 = 2;
        const ETA_TWO: u16 = 2;
        const Du: u16 = 10;
        const Dv: u16 = 4;
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct P1024;
    impl ParameterSet for P1024 {
        const K: u16 = 4;
        const ETA_ONE: u16 = 2;
        const ETA_TWO: u16 = 2;
        const Du: u16 = 11;
        const Dv: u16 = 5;
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
pub const K_INVERSE_NTT_ROOTS: [u16; 128] = [
    1, 1600, 40, 749, 2481, 1432, 2699, 687, 1583, 2760, 69, 543, 2532, 3136, 1410, 2267, 2508,
    1355, 450, 936, 447, 2794, 1235, 1903, 1996, 1089, 3273, 283, 1853, 1990, 882, 3033, 2419,
    2102, 219, 855, 2681, 1848, 712, 682, 927, 1795, 461, 1891, 2877, 2522, 1894, 1010, 1414, 2009,
    3296, 464, 2697, 816, 1352, 2679, 1274, 1052, 1025, 2132, 1573, 76, 2998, 3040, 1175, 2444,
    394, 1219, 2300, 1455, 2117, 1607, 2443, 554, 1179, 2186, 2303, 2926, 2237, 525, 735, 863,
    2768, 1230, 2572, 556, 3010, 2266, 1684, 1239, 780, 2954, 109, 1292, 1031, 1745, 2688, 3061,
    992, 2596, 941, 892, 1021, 2390, 642, 1868, 2377, 1482, 1540, 540, 1678, 1626, 279, 314, 1173,
    2573, 3096, 48, 667, 1920, 2229, 1041, 2606, 1692, 680, 2746, 568, 3312,
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
