#[allow(dead_code)]
pub mod ml_kem_constants {
    pub const q: u16 = 3329;
    pub const n: u16 = 256;
    pub const ENCODE_SIZE_12: u16 = n * 12 / 8;
}

// appendix table 2
#[allow(dead_code)]

pub mod parameter_sets {

    // Requireed random bit generator strength
    // 512 -> RBG(128)
    // 768 -> RBG(192)
    // 1024 -> RBG(256)

    pub trait ParameterSet {
        const k: u16;
        const ETA_ONE: u16;
        const ETA_TWO: u16;
        const du: u16;
        const dv: u16;
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct P512;
    impl ParameterSet for P512 {
        const k: u16 = 2;
        const ETA_ONE: u16 = 3;
        const ETA_TWO: u16 = 2;
        const du: u16 = 10;
        const dv: u16 = 4;
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct P768;
    impl ParameterSet for P768 {
        const k: u16 = 3;
        const ETA_ONE: u16 = 2;
        const ETA_TWO: u16 = 2;
        const du: u16 = 10;
        const dv: u16 = 4;
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct P1024;
    impl ParameterSet for P1024 {
        const k: u16 = 4;
        const ETA_ONE: u16 = 2;
        const ETA_TWO: u16 = 2;
        const du: u16 = 11;
        const dv: u16 = 5;
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

#[cfg(test)]
// testing against the great Filippo Valsorda https://github.com/FiloSottile/mlkem768
pub const sample_poly_cbd_result: [u16; 256] = [
    3328, 3328, 3327, 3328, 3328, 2, 0, 3328, 3328, 0, 0, 0, 0, 3328, 2, 1, 0, 0, 3328, 1, 0, 3328,
    1, 3328, 3328, 0, 0, 0, 1, 3328, 1, 3328, 1, 0, 2, 0, 1, 0, 3328, 0, 2, 3328, 3328, 2, 1, 3328,
    1, 0, 0, 0, 3327, 0, 1, 2, 3328, 1, 0, 0, 3328, 0, 0, 0, 0, 1, 1, 1, 1, 3328, 2, 3328, 3327,
    3328, 0, 1, 0, 0, 0, 3328, 1, 3328, 0, 0, 0, 1, 1, 0, 3328, 0, 1, 1, 0, 3328, 3328, 3328, 1, 1,
    0, 0, 3328, 0, 3327, 0, 2, 1, 0, 3328, 1, 0, 3328, 3328, 3328, 3328, 0, 1, 0, 0, 1, 2, 1, 0, 1,
    0, 3328, 3328, 0, 2, 3328, 3328, 3328, 0, 3328, 3328, 3328, 1, 3328, 1, 3328, 1, 0, 3328, 1,
    3328, 2, 3327, 3328, 0, 3328, 2, 1, 1, 0, 3328, 0, 3328, 0, 1, 3327, 0, 0, 1, 3328, 1, 3328, 1,
    1, 3328, 2, 3328, 0, 0, 3328, 1, 3327, 0, 3327, 1, 3328, 3327, 0, 0, 3328, 1, 0, 1, 1, 3328, 1,
    3328, 0, 0, 0, 2, 3328, 2, 3328, 0, 3328, 2, 3328, 0, 0, 1, 0, 3328, 2, 0, 0, 2, 3328, 3328, 0,
    3327, 3328, 1, 0, 0, 1, 0, 3328, 3328, 1, 3328, 3327, 0, 3327, 3328, 0, 0, 0, 0, 3328, 0, 1, 0,
    0, 0, 2, 3327, 1, 1, 0, 0, 1, 0, 0, 3327, 1, 3327, 0, 0, 0, 0, 1, 3327, 1, 1,
];

#[cfg(test)]
pub const sample_ntt_result: [u16; 256] = [
    2278, 18, 2449, 1376, 2453, 1346, 66, 738, 2100, 1008, 950, 2669, 2121, 3030, 880, 2569, 3146,
    1432, 1285, 2106, 1943, 895, 2326, 3255, 1301, 1752, 1281, 2500, 3149, 1061, 959, 687, 199,
    1817, 1651, 2069, 3091, 2864, 120, 2222, 3005, 1823, 2721, 3012, 665, 1426, 386, 1639, 1632,
    591, 1405, 756, 464, 1405, 2701, 3275, 76, 2137, 664, 2457, 2216, 2352, 1994, 1521, 1944, 1753,
    999, 2051, 3219, 2771, 1596, 2123, 527, 339, 2532, 2079, 2994, 576, 1876, 2698, 1708, 119, 537,
    2122, 3132, 285, 3198, 3131, 2761, 3187, 1, 3082, 2809, 3140, 895, 356, 1653, 2663, 2856, 2290,
    3166, 1245, 1876, 2355, 2746, 3213, 619, 551, 3216, 2092, 966, 479, 3079, 2557, 2706, 380,
    2388, 915, 4, 2336, 144, 3220, 1807, 95, 1109, 2105, 1441, 2379, 2890, 2985, 2129, 1040, 1472,
    1350, 1976, 927, 862, 1556, 2188, 447, 856, 1458, 2372, 1254, 2132, 2618, 200, 2880, 2834,
    1811, 505, 124, 621, 2574, 2546, 2974, 1875, 1646, 618, 1867, 1394, 1059, 486, 1232, 2574, 563,
    2509, 2805, 2674, 1594, 782, 1147, 12, 1853, 459, 2718, 1861, 913, 2538, 1986, 346, 2139, 1256,
    3148, 830, 615, 676, 2220, 2638, 893, 977, 474, 1096, 1307, 3285, 462, 3082, 2805, 1286, 2645,
    2733, 2695, 2082, 3216, 414, 1376, 2636, 971, 2671, 1721, 746, 516, 1620, 688, 1903, 2497,
    2869, 1587, 819, 256, 2326, 943, 1733, 117, 2941, 2933, 1852, 2753, 2057, 2585, 1042, 2572,
    220, 3049, 558, 2617, 1975, 45, 2593, 757, 3202, 1164, 1123, 1458, 1720, 2365, 148, 605, 2229,
    760, 90, 3212, 3015, 1643, 1962, 2954,
];

#[cfg(test)]
pub const kpke_encrypt_result: &[u8] = &[
    84, 35, 115, 11, 166, 106, 231, 167, 198, 201, 239, 182, 242, 112, 83, 239, 245, 55, 149, 178,
    77, 35, 185, 32, 75, 126, 62, 182, 80, 15, 215, 234, 107, 135, 208, 47, 63, 95, 190, 100, 62,
    109, 220, 50, 142, 4, 30, 149, 238, 247, 131, 73, 7, 94, 109, 136, 154, 5, 182, 232, 178, 211,
    105, 9, 43, 185, 123, 192, 33, 124, 158, 93, 105, 19, 129, 110, 213, 171, 86, 175, 79, 95, 208,
    232, 162, 2, 12, 44, 137, 11, 240, 189, 170, 5, 201, 196, 176, 73, 1, 52, 66, 136, 254, 155,
    126, 189, 190, 175, 185, 177, 145, 55, 235, 3, 230, 94, 49, 47, 134, 122, 241, 52, 229, 91, 34,
    127, 109, 227, 128, 163, 47, 195, 68, 157, 175, 128, 153, 245, 74, 33, 100, 191, 105, 14, 230,
    145, 99, 120, 233, 153, 227, 9, 150, 195, 72, 144, 248, 13, 134, 144, 21, 115, 19, 161, 9, 69,
    123, 185, 247, 30, 27, 100, 201, 139, 157, 123, 30, 113, 101, 141, 150, 232, 153, 127, 0, 131,
    142, 74, 159, 36, 155, 122, 54, 43, 127, 229, 50, 104, 160, 26, 111, 59, 169, 173, 56, 8, 216,
    116, 108, 126, 17, 166, 222, 89, 178, 16, 92, 17, 141, 244, 48, 135, 152, 189, 243, 151, 31,
    75, 55, 153, 186, 148, 38, 221, 64, 51, 15, 55, 179, 46, 51, 63, 3, 29, 71, 14, 10, 231, 192,
    165, 205, 0, 118, 144, 237, 220, 111, 214, 244, 30, 170, 42, 156, 95, 45, 24, 119, 121, 219, 3,
    62, 165, 136, 99, 29, 17, 38, 101, 179, 233, 70, 195, 59, 93, 76, 183, 31, 134, 133, 240, 132,
    238, 207, 146, 114, 0, 93, 198, 116, 171, 95, 145, 143, 46, 240, 80, 79, 54, 63, 255, 57, 138,
    5, 231, 36, 79, 54, 228, 17, 100, 200, 219, 115, 170, 113, 179, 251, 5, 205, 77, 175, 164, 175,
    122, 91, 101, 251, 102, 213, 49, 88, 58, 109, 86, 235, 48, 15, 226, 79, 83, 193, 105, 30, 229,
    82, 5, 209, 175, 198, 186, 55, 157, 221, 255, 98, 211, 3, 125, 58, 244, 142, 92, 205, 114, 143,
    120, 151, 240, 146, 117, 245, 15, 184, 41, 7, 76, 20, 47, 251, 232, 87, 16, 0, 86, 45, 173,
    222, 234, 252, 36, 64, 65, 92, 10, 102, 24, 158, 66, 173, 224, 112, 155, 192, 47, 192, 28, 217,
    191, 176, 228, 205, 132, 154, 2, 145, 188, 61, 111, 14, 128, 185, 43, 16, 25, 40, 17, 245, 159,
    242, 42, 4, 45, 73, 13, 153, 203, 149, 169, 16, 4, 64, 82, 37, 139, 105, 65, 170, 175, 176, 57,
    29, 140, 73, 111, 32, 190, 182, 123, 41, 71, 183, 28, 136, 175, 16, 52, 89, 202, 167, 170, 22,
    37, 225, 114, 46, 121, 247, 46, 89, 79, 113, 150, 137, 201, 155, 152, 176, 222, 250, 117, 106,
    147, 215, 177, 195, 88, 29, 179, 147, 231, 55, 88, 38, 102, 57, 83, 112, 244, 2, 66, 141, 99,
    72, 183, 241, 9, 7, 165, 115, 236, 144, 124, 108, 20, 80, 219, 55, 104, 68, 123, 32, 140, 202,
    205, 76, 175, 162, 105, 121, 139, 59, 72, 88, 155, 61, 6, 182, 52, 236, 232, 127, 182, 207,
    173, 31, 170, 34, 197, 101, 157, 147, 118, 66, 131, 2, 233, 48, 148, 93, 114, 12, 4, 37, 218,
    235, 212, 249, 81, 150, 207, 115, 140, 252, 55, 254, 188, 198, 230, 38, 6, 1, 40, 234, 147,
    120, 56, 52, 2, 183, 225, 38, 39, 87, 19, 249, 225, 108, 228, 3, 188, 8, 235, 136, 86, 39, 125,
    237, 150, 143, 16, 135, 189, 140, 112, 136, 86, 127, 190, 122, 36, 243, 69, 193, 161, 220, 137,
    164, 229, 22, 124, 34, 76, 92, 61, 130, 243, 51, 4, 103, 142, 41, 194, 237, 88, 70, 72, 241,
    205, 102, 189, 90, 30, 216, 21, 149, 190, 45, 215, 118, 131, 188, 233, 132, 158, 33, 12, 65,
    234, 130, 201, 6, 234, 182, 140, 3, 44, 41, 233, 240, 81, 99, 174, 190, 43, 243, 24, 180, 245,
    251, 205, 31, 135, 162, 42, 179, 243, 14, 57, 21, 220, 45, 229, 239, 169, 153, 54, 115, 156,
    80, 171, 4, 172, 151, 163, 9, 87, 247, 22, 87, 65, 179, 197, 52, 253, 177, 107, 144, 194, 229,
    58, 26, 102, 197, 155, 99, 112, 75, 12, 99, 29, 95, 101, 162, 184, 147, 21, 38, 212, 23, 129,
    63, 229, 88, 133, 175, 246, 71, 57, 15, 102, 46, 129, 133, 8, 217, 31, 69, 108, 92, 219, 162,
    147, 38, 114, 227, 173, 92, 194, 195, 172, 119, 172, 174, 222, 102, 182, 136, 4, 159, 10, 60,
    243, 59, 201, 10, 123, 209, 165, 61, 115, 198, 6, 190, 89, 78, 218, 100, 124, 75, 245, 114, 20,
    101, 137, 119, 8, 6, 141, 183, 177, 51, 179, 127, 116, 124, 164, 200, 121, 32, 194, 143, 136,
    4, 121, 65, 176, 172, 187, 54, 41, 146, 36, 167, 242, 207, 242, 251, 242, 146, 234, 12, 99, 25,
    151, 29, 31, 100, 125, 177, 175, 184, 80, 124, 236, 232, 0, 194, 30, 66, 252, 27, 99, 63, 67,
    23, 85, 68, 186, 27, 213, 65, 6, 173, 242, 161, 97, 177, 245, 107, 8, 126, 223, 144, 226, 198,
    24, 92, 67, 6, 124, 224, 218, 61, 42, 115, 109, 167, 134, 217, 0, 206, 213, 101, 92, 247, 34,
    192, 96, 36, 216, 194, 73, 226, 227, 125, 228, 198, 85, 68, 84, 232, 105, 141, 152, 69, 248,
    203, 50, 230, 152, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0,
    8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0,
    8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0,
    8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0,
    8, 0, 0, 0, 8, 0, 0,
];

pub const kpke_encrypd_result: &[u8] = &[
    84, 35, 115, 11, 166, 106, 231, 167, 198, 201, 239, 182, 242, 112, 83, 239, 245, 55, 149, 178,
    77, 35, 185, 32, 75, 126, 62, 182, 80, 15, 215, 234, 107, 135, 208, 47, 63, 95, 190, 100, 62,
    109, 220, 50, 142, 4, 30, 149, 238, 247, 131, 73, 7, 94, 109, 136, 154, 5, 182, 232, 178, 211,
    105, 9, 43, 185, 123, 192, 33, 124, 158, 93, 105, 19, 129, 110, 213, 171, 86, 175, 79, 95, 208,
    232, 162, 2, 12, 44, 137, 11, 240, 189, 170, 5, 201, 196, 176, 73, 1, 52, 66, 136, 254, 155,
    126, 189, 190, 175, 185, 177, 145, 55, 235, 3, 230, 94, 49, 47, 134, 122, 241, 52, 229, 91, 34,
    127, 109, 227, 128, 163, 47, 195, 68, 157, 175, 128, 153, 245, 74, 33, 100, 191, 105, 14, 230,
    145, 99, 120, 233, 153, 227, 9, 150, 195, 72, 144, 248, 13, 134, 144, 21, 115, 19, 161, 9, 69,
    123, 185, 247, 30, 27, 100, 201, 139, 157, 123, 30, 113, 101, 141, 150, 232, 153, 127, 0, 131,
    142, 74, 159, 36, 155, 122, 54, 43, 127, 229, 50, 104, 160, 26, 111, 59, 169, 173, 56, 8, 216,
    116, 108, 126, 17, 166, 222, 89, 178, 16, 92, 17, 141, 244, 48, 135, 152, 189, 243, 151, 31,
    75, 55, 153, 186, 148, 38, 221, 64, 51, 15, 55, 179, 46, 51, 63, 3, 29, 71, 14, 10, 231, 192,
    165, 205, 0, 118, 144, 237, 220, 111, 214, 244, 30, 170, 42, 156, 95, 45, 24, 119, 121, 219, 3,
    62, 165, 136, 99, 29, 17, 38, 101, 179, 233, 70, 195, 59, 93, 76, 183, 31, 134, 133, 240, 132,
    238, 207, 146, 114, 0, 93, 198, 116, 171, 95, 145, 143, 46, 240, 80, 79, 54, 63, 255, 57, 138,
    5, 231, 36, 79, 54, 228, 17, 100, 200, 219, 115, 170, 113, 179, 251, 5, 205, 77, 175, 164, 175,
    122, 91, 101, 251, 102, 213, 49, 88, 58, 109, 86, 235, 48, 15, 226, 79, 83, 193, 105, 30, 229,
    82, 5, 209, 175, 198, 186, 55, 157, 221, 255, 98, 211, 3, 125, 58, 244, 142, 92, 205, 114, 143,
    120, 151, 240, 146, 117, 245, 15, 184, 41, 7, 76, 20, 47, 251, 232, 87, 16, 0, 86, 45, 173,
    222, 234, 252, 36, 64, 65, 92, 10, 102, 24, 158, 66, 173, 224, 112, 155, 192, 47, 192, 28, 217,
    191, 176, 228, 205, 132, 154, 2, 145, 188, 61, 111, 14, 128, 185, 43, 16, 25, 40, 17, 245, 159,
    242, 42, 4, 45, 73, 13, 153, 203, 149, 169, 16, 4, 64, 82, 37, 139, 105, 65, 170, 175, 176, 57,
    29, 140, 73, 111, 32, 190, 182, 123, 41, 71, 183, 28, 136, 175, 16, 52, 89, 202, 167, 170, 22,
    37, 225, 114, 46, 121, 247, 46, 89, 79, 113, 150, 137, 201, 155, 152, 176, 222, 250, 117, 106,
    147, 215, 177, 195, 88, 29, 179, 147, 231, 55, 88, 38, 102, 57, 83, 112, 244, 2, 66, 141, 99,
    72, 183, 241, 9, 7, 165, 115, 236, 144, 124, 108, 20, 80, 219, 55, 104, 68, 123, 32, 140, 202,
    205, 76, 175, 162, 105, 121, 139, 59, 72, 88, 155, 61, 6, 182, 52, 236, 232, 127, 182, 207,
    173, 31, 170, 34, 197, 101, 157, 147, 118, 66, 131, 2, 233, 48, 148, 93, 114, 12, 4, 37, 218,
    235, 212, 249, 81, 150, 207, 115, 140, 252, 55, 254, 188, 198, 230, 38, 6, 1, 40, 234, 147,
    120, 56, 52, 2, 183, 225, 38, 39, 87, 19, 249, 225, 108, 228, 3, 188, 8, 235, 136, 86, 39, 125,
    237, 150, 143, 16, 135, 189, 140, 112, 136, 86, 127, 190, 122, 36, 243, 69, 193, 161, 220, 137,
    164, 229, 22, 124, 34, 76, 92, 61, 130, 243, 51, 4, 103, 142, 41, 194, 237, 88, 70, 72, 241,
    205, 102, 189, 90, 30, 216, 21, 149, 190, 45, 215, 118, 131, 188, 233, 132, 158, 33, 12, 65,
    234, 130, 201, 6, 234, 182, 140, 3, 44, 41, 233, 240, 81, 99, 174, 190, 43, 243, 24, 180, 245,
    251, 205, 31, 135, 162, 42, 179, 243, 14, 57, 21, 220, 45, 229, 239, 169, 153, 54, 115, 156,
    80, 171, 4, 172, 151, 163, 9, 87, 247, 22, 87, 65, 179, 197, 52, 253, 177, 107, 144, 194, 229,
    58, 26, 102, 197, 155, 99, 112, 75, 12, 99, 29, 95, 101, 162, 184, 147, 21, 38, 212, 23, 129,
    63, 229, 88, 133, 175, 246, 71, 57, 15, 102, 46, 129, 133, 8, 217, 31, 69, 108, 92, 219, 162,
    147, 38, 114, 227, 173, 92, 194, 195, 172, 119, 172, 174, 222, 102, 182, 136, 4, 159, 10, 60,
    243, 59, 201, 10, 123, 209, 165, 61, 115, 198, 6, 190, 89, 78, 218, 100, 124, 75, 245, 114, 20,
    101, 137, 119, 8, 6, 141, 183, 177, 51, 179, 127, 116, 124, 164, 200, 121, 32, 194, 143, 136,
    4, 121, 65, 176, 172, 187, 54, 41, 146, 36, 167, 242, 207, 242, 251, 242, 146, 234, 12, 99, 25,
    151, 29, 31, 100, 125, 177, 175, 184, 80, 124, 236, 232, 0, 194, 30, 66, 252, 27, 99, 63, 67,
    23, 85, 68, 186, 27, 213, 65, 6, 173, 242, 161, 97, 177, 245, 107, 8, 126, 223, 144, 226, 198,
    24, 92, 67, 6, 124, 224, 218, 61, 42, 115, 109, 167, 134, 217, 0, 206, 213, 101, 92, 247, 34,
    192, 96, 36, 216, 194, 73, 226, 227, 125, 228, 198, 85, 68, 84, 232, 105, 141, 152, 69, 248,
    203, 50, 230, 152, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0,
    8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0,
    8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0,
    8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0,
    8, 0, 0, 0, 8, 0, 0,
];
