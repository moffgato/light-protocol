use groth16_solana::groth16::Groth16Verifyingkey;

pub const VERIFYINGKEY: Groth16Verifyingkey = Groth16Verifyingkey {
    nr_pubinputs: 3,

    vk_alpha_g1: [
        45, 77, 154, 167, 227, 2, 217, 223, 65, 116, 157, 85, 7, 148, 157, 5, 219, 234, 51, 251,
        177, 108, 100, 59, 34, 245, 153, 162, 190, 109, 242, 226, 20, 190, 221, 80, 60, 55, 206,
        176, 97, 216, 236, 96, 32, 159, 227, 69, 206, 137, 131, 10, 25, 35, 3, 1, 240, 118, 202,
        255, 0, 77, 25, 38,
    ],

    vk_beta_g2: [
        9, 103, 3, 47, 203, 247, 118, 209, 175, 201, 133, 248, 136, 119, 241, 130, 211, 132, 128,
        166, 83, 242, 222, 202, 169, 121, 76, 188, 59, 243, 6, 12, 14, 24, 120, 71, 173, 76, 121,
        131, 116, 208, 214, 115, 43, 245, 1, 132, 125, 214, 139, 192, 224, 113, 36, 30, 2, 19, 188,
        127, 193, 61, 183, 171, 48, 76, 251, 209, 224, 138, 112, 74, 153, 245, 232, 71, 217, 63,
        140, 60, 170, 253, 222, 196, 107, 122, 13, 55, 157, 166, 154, 77, 17, 35, 70, 167, 23, 57,
        193, 177, 164, 87, 168, 199, 49, 49, 35, 210, 77, 47, 145, 146, 248, 150, 183, 198, 62,
        234, 5, 169, 213, 127, 6, 84, 122, 208, 206, 200,
    ],

    vk_gamme_g2: [
        25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241, 170, 73, 51,
        53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18, 31, 30, 118,
        66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70, 222, 189, 92,
        217, 146, 246, 237, 9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149,
        188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91, 18, 200, 94, 165,
        219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143, 227, 209, 231, 105, 12, 67, 211,
        123, 76, 230, 204, 1, 102, 250, 125, 170,
    ],

    vk_delta_g2: [
        9, 112, 121, 8, 13, 41, 99, 52, 161, 125, 153, 212, 159, 69, 135, 252, 216, 149, 83, 187,
        190, 50, 80, 251, 92, 155, 252, 24, 34, 154, 149, 156, 12, 39, 163, 21, 24, 164, 28, 39, 1,
        49, 81, 221, 174, 174, 31, 9, 96, 209, 121, 84, 15, 98, 107, 24, 9, 180, 146, 101, 69, 140,
        19, 51, 29, 132, 210, 240, 238, 97, 1, 123, 110, 124, 123, 100, 83, 11, 244, 228, 7, 157,
        70, 158, 206, 146, 71, 96, 157, 215, 31, 176, 6, 174, 87, 54, 25, 9, 41, 6, 193, 123, 171,
        221, 187, 65, 17, 238, 133, 25, 161, 81, 251, 176, 189, 151, 76, 59, 55, 237, 108, 178, 60,
        235, 99, 68, 225, 189,
    ],

    vk_ic: &[
        [
            28, 5, 55, 182, 113, 41, 255, 21, 123, 19, 187, 98, 205, 92, 18, 94, 97, 201, 10, 28,
            86, 243, 57, 47, 41, 237, 102, 51, 167, 191, 117, 2, 38, 37, 133, 234, 253, 125, 60,
            72, 162, 176, 139, 136, 84, 78, 116, 209, 12, 199, 191, 27, 213, 45, 246, 207, 226, 5,
            184, 35, 54, 148, 128, 96,
        ],
        [
            10, 190, 52, 144, 47, 229, 119, 100, 218, 131, 253, 26, 0, 141, 231, 17, 106, 64, 56,
            90, 202, 233, 131, 25, 132, 47, 141, 52, 200, 238, 241, 19, 44, 206, 122, 67, 41, 63,
            196, 71, 62, 88, 193, 209, 108, 149, 60, 200, 141, 10, 208, 107, 106, 103, 160, 51,
            155, 54, 89, 136, 19, 104, 149, 197,
        ],
        [
            48, 57, 130, 24, 224, 78, 14, 10, 153, 159, 51, 247, 252, 28, 153, 87, 172, 77, 125,
            163, 212, 51, 45, 89, 78, 58, 174, 119, 215, 71, 179, 15, 27, 140, 108, 17, 156, 36,
            168, 2, 33, 116, 228, 193, 17, 225, 145, 224, 132, 6, 241, 223, 99, 209, 42, 246, 40,
            11, 95, 149, 229, 41, 37, 137,
        ],
    ],
};