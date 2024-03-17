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

///
/// testing against the great Filippo Valsorda https://github.com/FiloSottile/mlkem768
///

#[cfg(test)]
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
    74, 209, 84, 207, 169, 51, 219, 155, 118, 215, 8, 200, 103, 69, 144, 115, 244, 26, 158, 20,
    161, 8, 65, 170, 44, 237, 74, 199, 6, 36, 167, 221, 136, 94, 204, 246, 62, 202, 130, 13, 195,
    215, 145, 156, 231, 131, 108, 20, 46, 126, 99, 230, 222, 72, 123, 195, 62, 137, 108, 55, 229,
    220, 59, 170, 0, 53, 135, 103, 29, 15, 166, 186, 245, 210, 238, 159, 85, 149, 53, 226, 99, 216,
    37, 233, 123, 85, 150, 113, 193, 153, 206, 16, 249, 72, 92, 30, 59, 10, 54, 123, 151, 187, 47,
    0, 29, 156, 10, 64, 9, 142, 210, 31, 214, 66, 89, 179, 1, 176, 106, 162, 196, 80, 77, 132, 7,
    231, 243, 89, 145, 159, 214, 88, 55, 182, 110, 79, 143, 141, 167, 159, 180, 93, 104, 67, 197,
    30, 38, 205, 73, 198, 230, 7, 69, 163, 176, 210, 182, 163, 6, 52, 99, 233, 159, 114, 169, 116,
    197, 37, 1, 64, 70, 137, 88, 135, 227, 150, 221, 36, 106, 182, 187, 34, 219, 53, 118, 16, 242,
    196, 216, 178, 212, 92, 237, 162, 65, 211, 110, 250, 202, 131, 243, 41, 132, 178, 30, 118, 159,
    184, 140, 58, 113, 141, 192, 63, 135, 150, 182, 145, 253, 247, 91, 84, 32, 133, 122, 6, 104,
    206, 63, 63, 200, 116, 96, 0, 142, 31, 205, 221, 226, 50, 121, 164, 13, 127, 65, 35, 199, 105,
    186, 50, 20, 140, 56, 29, 145, 168, 247, 116, 13, 221, 30, 230, 27, 22, 186, 101, 110, 212, 33,
    67, 63, 182, 14, 147, 145, 113, 235, 152, 248, 206, 55, 199, 234, 88, 207, 196, 134, 251, 54,
    40, 19, 164, 228, 79, 245, 161, 159, 198, 120, 47, 139, 149, 162, 42, 232, 137, 192, 21, 164,
    30, 113, 153, 107, 79, 85, 68, 140, 3, 36, 217, 227, 115, 227, 219, 139, 129, 121, 36, 174, 82,
    3, 88, 209, 40, 21, 205, 103, 51, 99, 253, 27, 35, 113, 203, 204, 195, 47, 154, 114, 120, 243,
    76, 180, 166, 148, 91, 115, 25, 209, 246, 39, 60, 11, 125, 34, 7, 117, 199, 127, 81, 132, 152,
    219, 38, 50, 254, 92, 55, 250, 15, 22, 40, 220, 248, 80, 83, 75, 126, 252, 223, 184, 71, 127,
    113, 102, 250, 86, 82, 76, 19, 45, 138, 26, 91, 71, 142, 55, 159, 240, 77, 214, 190, 73, 124,
    148, 76, 222, 17, 57, 55, 232, 65, 168, 57, 103, 167, 105, 213, 218, 53, 184, 229, 212, 230,
    136, 249, 214, 105, 103, 223, 83, 17, 71, 77, 108, 222, 62, 184, 6, 59, 127, 106, 186, 189,
    111, 170, 171, 235, 57, 147, 79, 88, 107, 139, 16, 111, 96, 4, 2, 9, 127, 134, 73, 218, 185,
    27, 68, 103, 233, 26, 65, 74, 153, 104, 116, 5, 180, 12, 89, 183, 197, 105, 4, 114, 157, 201,
    176, 191, 23, 153, 225, 80, 73, 10, 239, 163, 98, 27, 42, 124, 118, 156, 176, 235, 155, 147,
    176, 30, 11, 178, 137, 152, 48, 144, 66, 232, 248, 101, 127, 130, 35, 166, 170, 219, 18, 173,
    164, 209, 145, 219, 55, 155, 27, 83, 66, 211, 47, 218, 222, 81, 146, 13, 34, 123, 92, 95, 1,
    243, 104, 67, 27, 46, 107, 83, 115, 239, 4, 101, 245, 222, 136, 106, 26, 238, 63, 156, 235,
    147, 72, 39, 243, 96, 83, 158, 194, 101, 215, 197, 12, 35, 42, 194, 91, 32, 166, 236, 32, 105,
    168, 201, 63, 106, 140, 165, 3, 255, 66, 244, 175, 128, 74, 149, 67, 222, 211, 37, 104, 120,
    16, 232, 83, 57, 19, 99, 190, 168, 114, 171, 155, 208, 25, 154, 74, 193, 21, 185, 26, 76, 84,
    57, 154, 41, 14, 104, 248, 179, 158, 246, 18, 215, 95, 73, 238, 96, 148, 114, 69, 1, 91, 5, 76,
    1, 220, 9, 151, 196, 163, 198, 92, 58, 226, 69, 62, 223, 231, 145, 197, 76, 148, 208, 43, 144,
    172, 68, 214, 205, 240, 68, 203, 243, 15, 185, 14, 79, 196, 7, 207, 35, 113, 9, 5, 33, 121,
    112, 219, 3, 54, 183, 59, 158, 47, 114, 142, 118, 35, 254, 232, 69, 22, 6, 95, 96, 133, 78,
    181, 231, 163, 231, 60, 22, 37, 238, 56, 124, 73, 27, 231, 13, 238, 193, 194, 188, 156, 4, 12,
    201, 184, 215, 40, 33, 81, 4, 120, 208, 71, 161, 72, 137, 86, 238, 121, 53, 167, 48, 103, 190,
    54, 213, 135, 46, 65, 80, 208, 77, 108, 152, 240, 96, 99, 230, 121, 224, 130, 25, 222, 156,
    163, 204, 118, 69, 76, 202, 157, 6, 65, 225, 227, 76, 224, 5, 189, 207, 91, 136, 181, 52, 19,
    52, 184, 181, 187, 210, 97, 167, 204, 121, 123, 173, 195, 210, 5, 185, 54, 250, 177, 99, 163,
    9, 131, 40, 156, 60, 184, 246, 105, 50, 51, 106, 249, 20, 26, 146, 202, 59, 131, 249, 20, 46,
    98, 225, 185, 64, 81, 175, 14, 174, 191, 238, 81, 246, 122, 16, 4, 3, 121, 170, 154, 151, 132,
    147, 11, 91, 173, 59, 104, 188, 90, 157, 68, 123, 34, 144, 214, 82, 60, 245, 42, 32, 197, 21,
    105, 69, 192, 68, 174, 203, 215, 199, 145, 49, 1, 182, 240, 138, 109, 249, 37, 69, 90, 71, 207,
    218, 140, 31, 80, 217, 176, 33, 191, 222, 78, 53, 10, 103, 216, 211, 18, 153, 93, 108, 180, 10,
    127, 122, 76, 240, 164, 218, 86, 103, 47, 146, 181, 225, 178, 244, 188, 166, 164, 252, 240,
    137, 133, 35, 108, 250, 204, 19, 72, 177, 242, 106, 80, 187, 41, 93, 37, 83, 87, 248, 40, 42,
    185, 235, 24, 248, 106, 253, 246, 94, 232, 255, 67, 245, 68, 129, 222, 119, 127, 18, 168, 185,
    110, 93, 60, 186, 125, 41, 45, 169, 252, 114, 215, 31, 149, 195, 215, 48, 233, 69, 42, 52, 234,
    57, 210, 95, 131, 35, 220, 147, 148, 119, 26, 154, 253, 73, 39, 179, 23, 3, 93, 134, 5, 66, 16,
    24, 154, 49, 234, 36, 195, 50, 6, 202, 139, 86, 234, 173, 59, 95, 79, 140, 225, 46, 98, 120,
    49, 20, 147, 137, 91, 28, 42, 173, 97, 46, 113, 140, 188, 124, 184, 73, 170, 178, 8, 109, 15,
    48, 30, 29, 12, 4, 61, 49, 228, 251, 141, 246,
];

pub const pke_keygen_ek_pke: &[u8] = &[
    142, 251, 196, 138, 24, 24, 231, 176, 90, 243, 179, 204, 241, 132, 78, 54, 1, 24, 69, 22, 187,
    36, 69, 111, 204, 9, 1, 145, 69, 57, 28, 89, 107, 106, 151, 177, 221, 208, 194, 66, 113, 54, 0,
    225, 62, 7, 24, 197, 169, 234, 182, 22, 193, 151, 74, 121, 1, 82, 80, 58, 108, 187, 108, 18,
    171, 37, 70, 250, 20, 16, 241, 11, 65, 252, 19, 64, 149, 3, 65, 162, 90, 61, 242, 43, 49, 134,
    29, 81, 18, 9, 214, 81, 83, 96, 242, 165, 226, 167, 48, 21, 4, 69, 147, 1, 87, 91, 212, 47, 14,
    38, 173, 162, 226, 173, 7, 139, 206, 157, 20, 106, 92, 67, 61, 219, 49, 29, 17, 3, 61, 248,
    138, 22, 57, 123, 144, 157, 248, 12, 144, 118, 167, 114, 201, 184, 141, 172, 24, 106, 1, 86,
    113, 11, 184, 161, 106, 206, 180, 9, 127, 109, 160, 207, 198, 64, 117, 27, 89, 141, 133, 177,
    131, 54, 99, 139, 73, 145, 27, 106, 215, 192, 23, 232, 126, 60, 193, 131, 141, 1, 6, 89, 179,
    111, 15, 215, 187, 81, 147, 36, 226, 6, 185, 227, 218, 82, 167, 227, 106, 197, 113, 122, 131,
    16, 144, 173, 100, 2, 142, 90, 34, 62, 148, 129, 0, 228, 121, 37, 32, 37, 216, 106, 96, 202,
    100, 207, 21, 244, 99, 90, 17, 166, 105, 251, 130, 222, 84, 137, 34, 82, 123, 105, 162, 98,
    111, 246, 114, 198, 24, 122, 166, 161, 83, 42, 19, 89, 150, 115, 92, 81, 96, 204, 208, 194,
    203, 112, 10, 116, 204, 243, 33, 178, 187, 25, 227, 132, 138, 172, 208, 121, 145, 83, 66, 168,
    27, 96, 145, 124, 22, 44, 4, 204, 69, 171, 34, 101, 247, 75, 170, 52, 88, 70, 66, 30, 110, 140,
    1, 139, 198, 36, 13, 42, 118, 20, 42, 36, 187, 41, 12, 132, 84, 187, 250, 136, 113, 165, 164,
    188, 249, 184, 177, 76, 117, 196, 246, 6, 167, 118, 84, 67, 210, 200, 35, 251, 11, 161, 141,
    102, 199, 192, 137, 164, 126, 84, 37, 72, 64, 139, 238, 37, 203, 115, 193, 142, 210, 101, 133,
    24, 197, 157, 195, 135, 5, 43, 43, 88, 247, 218, 57, 21, 81, 180, 19, 231, 116, 104, 20, 103,
    21, 137, 139, 134, 17, 3, 86, 192, 117, 9, 156, 179, 61, 139, 146, 99, 182, 115, 69, 119, 207,
    135, 37, 129, 73, 0, 176, 152, 33, 56, 104, 49, 141, 133, 172, 133, 251, 84, 97, 19, 199, 56,
    67, 234, 148, 132, 248, 146, 10, 149, 159, 118, 85, 53, 69, 105, 56, 30, 36, 17, 150, 16, 93,
    15, 181, 155, 183, 138, 131, 139, 11, 168, 79, 44, 181, 214, 230, 15, 12, 70, 83, 149, 242, 67,
    132, 118, 159, 8, 21, 94, 146, 164, 10, 73, 233, 130, 178, 50, 82, 164, 116, 154, 225, 152,
    119, 59, 209, 28, 213, 123, 162, 197, 32, 203, 65, 214, 164, 107, 226, 26, 228, 151, 4, 179,
    23, 134, 209, 52, 193, 242, 211, 15, 70, 48, 26, 234, 232, 125, 17, 181, 110, 222, 212, 186,
    123, 121, 29, 46, 232, 87, 81, 38, 36, 137, 105, 147, 26, 177, 100, 219, 83, 133, 145, 185,
    132, 214, 87, 178, 181, 49, 141, 186, 86, 4, 187, 114, 80, 65, 23, 140, 39, 90, 126, 149, 116,
    105, 60, 131, 67, 17, 90, 165, 104, 24, 70, 210, 67, 202, 227, 172, 128, 203, 186, 193, 181,
    34, 102, 90, 217, 148, 236, 147, 0, 160, 65, 6, 69, 192, 16, 240, 132, 136, 36, 186, 16, 149,
    116, 114, 233, 55, 55, 207, 153, 2, 13, 181, 181, 72, 248, 113, 57, 114, 14, 90, 119, 184, 160,
    148, 109, 238, 84, 84, 161, 166, 169, 229, 17, 188, 254, 168, 2, 219, 213, 20, 220, 199, 61,
    206, 183, 5, 49, 5, 91, 210, 87, 15, 55, 49, 124, 216, 4, 53, 234, 138, 165, 177, 54, 168, 182,
    192, 201, 11, 145, 147, 186, 23, 124, 191, 214, 96, 226, 9, 113, 47, 148, 179, 21, 165, 178, 6,
    155, 17, 25, 10, 184, 225, 133, 20, 62, 124, 135, 201, 220, 62, 254, 6, 117, 24, 87, 116, 99,
    151, 200, 167, 91, 35, 22, 118, 149, 175, 105, 15, 183, 245, 17, 197, 161, 189, 239, 197, 119,
    229, 229, 77, 108, 72, 205, 80, 227, 86, 48, 153, 115, 85, 10, 140, 49, 220, 201, 140, 177,
    160, 4, 214, 138, 68, 180, 71, 223, 108, 98, 72, 248, 133, 236, 231, 65, 207, 251, 154, 3, 227,
    15, 3, 73, 183, 56, 208, 98, 7, 97, 35, 26, 103, 98, 128, 247, 171, 218, 145, 176, 112, 230,
    175, 33, 132, 178, 241, 208, 195, 182, 145, 61, 254, 251, 80, 111, 107, 0, 130, 156, 88, 238,
    1, 125, 127, 48, 80, 7, 128, 79, 99, 144, 110, 203, 58, 18, 60, 12, 182, 190, 25, 194, 83, 98,
    160, 208, 25, 48, 199, 19, 204, 99, 84, 167, 127, 149, 153, 244, 154, 38, 136, 7, 26, 192, 27,
    199, 212, 18, 26, 0, 253, 107, 172, 121, 86, 117, 102, 152, 86, 66, 46, 111, 72, 49, 167, 220,
    186, 83, 70, 9, 60, 86, 83, 114, 80, 60, 180, 4, 10, 118, 24, 105, 106, 146, 39, 148, 209, 55,
    40, 148, 155, 135, 33, 163, 27, 40, 125, 108, 34, 101, 253, 225, 67, 216, 232, 201, 96, 234,
    74, 83, 179, 33, 161, 34, 33, 32, 58, 40, 224, 166, 126, 18, 169, 131, 53, 131, 66, 137, 101,
    63, 225, 203, 153, 151, 16, 113, 128, 36, 159, 175, 170, 127, 170, 215, 115, 132, 92, 157, 239,
    129, 148, 198, 155, 191, 195, 90, 88, 91, 232, 113, 182, 187, 142, 126, 106, 32, 206, 119, 74,
    154, 247, 79, 65, 184, 199, 153, 20, 116, 235, 225, 73, 8, 214, 44, 9, 64, 120, 160, 16, 45,
    49, 57, 107, 23, 0, 100, 172, 107, 83, 252, 81, 73, 11, 43, 95, 14, 154, 68, 203, 227, 21, 18,
    176, 73, 243, 72, 96, 55, 11, 129, 45, 124, 61, 171, 17, 5, 87, 50, 145, 188, 39, 74, 66, 75,
    131, 142, 165, 135, 119, 138, 72, 90, 219, 54, 193, 33, 41, 166, 5, 61, 183, 21, 25, 137, 138,
    51, 114, 19, 6, 241, 128, 9, 105, 153, 127, 33, 128, 132, 243, 92, 47, 209, 18, 74, 249, 241,
    92, 120, 163, 177, 103, 193, 187, 213, 35, 140, 37, 72, 7, 51, 19, 50, 122, 3, 106, 92, 73, 39,
    129, 195, 61, 249, 112, 120, 243, 98, 133, 143, 122, 44, 116, 156, 29, 39, 228, 11, 145, 89,
    206, 124, 212, 53, 208, 112, 163, 3, 128, 97, 97, 180, 80, 203, 46, 28, 54, 133, 11, 187, 170,
    169, 37, 32, 38, 20, 64, 147, 5, 77, 173, 119, 73, 29, 139, 237, 236, 109, 194,
];

pub const pke_keygen_dk_pke: &[u8] = &[
    78, 184, 44, 170, 153, 67, 8, 11, 163, 12, 219, 106, 231, 71, 194, 111, 32, 179, 163, 232, 112,
    163, 90, 56, 84, 67, 188, 176, 11, 159, 223, 96, 83, 40, 106, 132, 74, 135, 42, 211, 76, 151,
    96, 146, 48, 53, 106, 166, 206, 68, 201, 3, 131, 117, 57, 144, 20, 111, 65, 108, 250, 169, 146,
    58, 240, 24, 7, 203, 78, 135, 172, 101, 41, 172, 114, 134, 16, 203, 194, 218, 144, 118, 97, 78,
    229, 194, 189, 92, 169, 174, 124, 25, 90, 14, 144, 95, 217, 119, 165, 73, 20, 107, 22, 198,
    117, 49, 225, 95, 242, 56, 33, 64, 176, 26, 121, 133, 45, 153, 251, 44, 130, 117, 154, 107,
    148, 159, 220, 131, 132, 226, 224, 117, 96, 96, 73, 1, 136, 173, 66, 96, 56, 208, 144, 102,
    231, 18, 204, 168, 226, 127, 129, 198, 116, 221, 118, 6, 13, 7, 135, 18, 0, 67, 241, 51, 193,
    224, 166, 149, 46, 75, 123, 250, 85, 67, 70, 90, 155, 164, 144, 178, 237, 84, 38, 231, 197, 92,
    23, 228, 198, 68, 145, 34, 44, 163, 25, 17, 161, 38, 26, 245, 158, 40, 104, 207, 167, 129, 63,
    63, 51, 82, 142, 251, 109, 240, 155, 107, 45, 139, 56, 95, 119, 14, 45, 218, 101, 79, 26, 35,
    41, 166, 98, 208, 243, 143, 101, 66, 50, 193, 160, 74, 214, 151, 147, 111, 210, 19, 121, 240,
    159, 162, 6, 7, 144, 131, 120, 30, 83, 42, 124, 54, 103, 162, 122, 7, 200, 16, 169, 7, 199,
    138, 22, 200, 169, 237, 119, 182, 125, 4, 17, 61, 44, 33, 191, 203, 4, 229, 50, 93, 0, 228, 64,
    41, 97, 5, 160, 194, 87, 142, 169, 154, 122, 37, 175, 79, 140, 88, 0, 161, 114, 30, 124, 134,
    20, 245, 74, 196, 34, 108, 238, 215, 40, 229, 165, 130, 184, 185, 167, 48, 38, 73, 20, 42, 49,
    35, 236, 141, 205, 229, 173, 37, 227, 172, 187, 183, 146, 176, 85, 152, 157, 240, 9, 22, 214,
    130, 139, 208, 22, 231, 138, 55, 212, 34, 160, 233, 188, 160, 131, 162, 198, 141, 23, 74, 182,
    8, 198, 60, 99, 171, 87, 224, 50, 249, 245, 174, 58, 149, 24, 237, 184, 58, 224, 172, 141, 165,
    100, 66, 30, 149, 1, 176, 187, 131, 87, 86, 17, 68, 203, 73, 85, 134, 3, 253, 219, 164, 243,
    226, 96, 8, 204, 99, 99, 21, 34, 160, 24, 119, 38, 39, 99, 128, 200, 79, 141, 9, 146, 154, 225,
    65, 145, 39, 50, 84, 218, 85, 145, 203, 97, 127, 128, 79, 222, 213, 3, 149, 103, 65, 39, 1,
    192, 176, 138, 86, 171, 195, 6, 210, 192, 88, 121, 208, 52, 177, 0, 135, 249, 74, 19, 33, 97,
    196, 81, 213, 52, 19, 147, 148, 100, 10, 21, 120, 202, 95, 104, 171, 207, 13, 187, 184, 230,
    229, 133, 220, 182, 174, 86, 227, 130, 216, 9, 164, 197, 17, 59, 145, 160, 76, 76, 146, 109,
    160, 183, 145, 132, 122, 48, 119, 210, 84, 26, 179, 130, 253, 156, 0, 143, 199, 181, 209, 106,
    8, 160, 138, 166, 221, 195, 84, 21, 9, 192, 7, 167, 153, 184, 89, 196, 206, 166, 181, 139, 75,
    81, 32, 150, 12, 144, 21, 93, 175, 16, 90, 138, 247, 75, 70, 119, 12, 110, 84, 184, 112, 192,
    190, 10, 66, 207, 166, 20, 96, 87, 201, 176, 134, 145, 100, 81, 240, 71, 117, 140, 0, 164, 170,
    73, 88, 98, 84, 14, 43, 22, 125, 230, 27, 228, 182, 84, 157, 169, 115, 173, 40, 205, 15, 81,
    189, 99, 225, 42, 141, 25, 0, 237, 137, 123, 199, 64, 24, 133, 131, 110, 237, 16, 206, 191,
    181, 116, 105, 136, 0, 68, 115, 66, 123, 211, 84, 96, 214, 154, 106, 68, 120, 74, 32, 49, 182,
    198, 46, 64, 75, 24, 215, 65, 118, 13, 246, 157, 55, 163, 21, 23, 18, 109, 24, 108, 115, 145,
    64, 107, 242, 243, 168, 142, 137, 56, 220, 139, 126, 192, 250, 44, 192, 134, 15, 198, 0, 74,
    93, 73, 39, 78, 252, 7, 149, 220, 159, 238, 151, 101, 180, 156, 131, 117, 187, 99, 216, 100,
    181, 180, 160, 48, 100, 128, 115, 62, 32, 160, 77, 91, 125, 22, 245, 16, 184, 33, 79, 36, 75,
    139, 101, 211, 178, 48, 80, 206, 68, 119, 152, 250, 3, 65, 48, 104, 97, 89, 180, 131, 65, 180,
    44, 142, 234, 33, 159, 60, 65, 76, 41, 67, 227, 106, 144, 134, 5, 56, 183, 201, 40, 63, 148, 0,
    35, 83, 35, 163, 12, 198, 170, 122, 161, 119, 39, 133, 116, 203, 99, 109, 123, 15, 184, 76,
    160, 82, 36, 150, 125, 4, 205, 127, 88, 106, 25, 64, 127, 169, 131, 205, 18, 169, 149, 199,
    232, 153, 179, 227, 126, 168, 88, 137, 180, 130, 121, 67, 115, 11, 195, 202, 203, 152, 21, 15,
    242, 10, 192, 173, 224, 168, 194, 212, 180, 131, 212, 166, 64, 215, 160, 26, 186, 198, 98, 89,
    77, 15, 156, 19, 191, 2, 19, 118, 236, 150, 156, 35, 181, 80, 247, 196, 247, 113, 8, 213, 183,
    95, 158, 137, 21, 66, 74, 23, 234, 240, 146, 110, 122, 162, 93, 135, 122, 152, 212, 92, 115,
    67, 13, 153, 247, 115, 193, 139, 0, 56, 102, 102, 15, 176, 6, 37, 7, 40, 180, 100, 133, 125,
    24, 5, 185, 163, 105, 62, 49, 186, 41, 169, 110, 229, 216, 11, 182, 59, 47, 137, 102, 157, 136,
    147, 185, 190, 106, 192, 120, 229, 190, 40, 69, 15, 60, 17, 202, 177, 122, 66, 185, 182, 155,
    242, 176, 9, 192, 33, 190, 10, 242, 52, 5, 250, 135, 72, 245, 182, 231, 115, 33, 93, 123, 61,
    33, 36, 10, 150, 106, 132, 184, 0, 83, 0, 6, 205, 140, 55, 107, 57, 137, 108, 149, 32, 107,
    205, 170, 89, 108, 196, 145, 205, 88, 114, 249, 244, 61, 29, 105, 170, 134, 84, 95, 48, 244,
    171, 111, 71, 24, 126, 140, 46, 209, 66, 168, 93, 22, 120, 81, 252, 104, 42, 183, 169, 195, 81,
    24, 162, 243, 23, 145, 81, 48, 46, 123, 5, 238, 82, 105, 14, 106, 1, 55, 187, 105, 11, 105,
    173, 5, 34, 9, 151, 168, 192, 92, 105, 173, 253, 212, 182, 90, 107, 65, 153, 67, 78, 195, 252,
    85, 171, 182, 196, 48, 230, 80, 128, 250, 107, 247, 97, 173, 250, 117, 160, 85, 7, 58, 76, 34,
    0, 167, 185, 8, 250, 18, 184, 87, 244, 170, 151, 211, 37, 202, 134, 45, 173, 101, 135, 87, 118,
    113, 41, 199, 40, 73, 5, 24, 42, 35, 115, 224, 121, 129,
];
