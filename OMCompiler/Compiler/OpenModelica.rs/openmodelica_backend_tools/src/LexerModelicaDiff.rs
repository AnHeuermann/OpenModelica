// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(
    unreachable_patterns,
    unreachable_code,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    unused_imports,
    unused_variables,
    non_upper_case_globals,
    unused_mut
)]

use arcstr::{ArcStr, format, literal};
use const_str;
use loop_unwrap::unwrap_break_err;
use metamodelica::Result;
use metamodelica::*; // Built-in types and functions
use std::sync::Arc;

use openmodelica_util::DiffAlgorithm;
use openmodelica_util::Error;
use openmodelica_util::StringUtil;
use openmodelica_util::System;

/*
 Template for Lexer Code
 replace keywords:
 %LexerCode
 %time
 %Token
 %Lexer
 %LexTable
 %constant
 %nameSpan
 %functions
 %caseAction
*/
pub(crate) const debug: bool = false;

pub mod LexTable {
    use super::*;
    pub(crate) const yy_limit: i32 = 395;

    pub(crate) const yy_finish: i32 = 453;

    pub(crate) static yy_acclist: std::sync::LazyLock<metamodelica::StaticArray<i32>> =
        std::sync::LazyLock::new(|| {
            metamodelica::StaticArray::new(
                list![
                    115, 114, 1, 114, 2, 114, 114, 101, 114, 114, 64, 114, 65, 114, 85, 114, 87, 114, 72, 114, 86, 114,
                    97, 114, 94, 114, 100, 114, 75, 114, 76, 114, 90, 114, 71, 114, 91, 114, 98, 114, 66, 114, 67, 114,
                    93, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114,
                    98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 68, 114, 69, 114, 109, 114, 110, 114, 109,
                    114, 113, 114, 112, 114, 105, 114, 106, 114, 104, 105, 114, 105, 114, 1, 2, 82, 80, 81, 83, 5, 84,
                    107, 111, 3, 100, 74, 73, 88, 89, 70, 92, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98,
                    98, 98, 98, 98, 98, 34, 98, 98, 36, 98, 98, 98, 98, 98, 46, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98,
                    98, 98, 98, 98, 108, 102, 103, 99, 3, 4, 98, 7, 98, 98, 98, 98, 98, 98, 98, 15, 98, 98, 98, 98, 98,
                    21, 98, 98, 98, 98, 98, 98, 98, 98, 32, 98, 98, 98, 98, 98, 98, 98, 98, 42, 98, 98, 98, 98, 98, 98,
                    98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 5, 3, 98, 98, 98, 98, 98, 98, 98,
                    98, 98, 17, 98, 18, 98, 98, 98, 98, 98, 98, 98, 98, 31, 98, 98, 98, 98, 98, 98, 98, 40, 98, 98, 98,
                    98, 98, 98, 98, 98, 98, 98, 98, 98, 77, 98, 98, 98, 98, 98, 98, 98, 56, 98, 57, 98, 58, 98, 59, 98,
                    98, 98, 98, 98, 9, 98, 63, 98, 10, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 29, 98, 30,
                    98, 98, 98, 98, 98, 38, 98, 39, 98, 41, 98, 98, 98, 43, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98,
                    98, 98, 98, 60, 98, 98, 98, 98, 98, 98, 98, 98, 98, 19, 98, 98, 98, 98, 98, 98, 98, 98, 98, 35, 98,
                    78, 98, 98, 98, 98, 47, 98, 98, 98, 98, 98, 98, 52, 98, 53, 98, 98, 98, 62, 98, 96, 98, 98, 61, 98,
                    98, 98, 11, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 26, 98, 98, 98, 37, 98, 98, 98, 98, 48, 98, 98,
                    50, 98, 98, 98, 98, 98, 98, 98, 98, 13, 98, 98, 98, 14, 98, 20, 98, 98, 98, 23, 98, 98, 28, 98, 33,
                    98, 44, 98, 98, 45, 98, 98, 98, 98, 98, 98, 6, 98, 98, 12, 98, 98, 98, 98, 98, 98, 98, 49, 98, 51,
                    98, 54, 98, 98, 95, 98, 8, 98, 98, 16, 98, 98, 98, 25, 98, 98, 98, 98, 98, 22, 98, 98, 55, 98, 98,
                    24, 98, 79, 98, 27, 98
                ]
                .into_iter()
                .cloned()
                .collect(),
            )
        });

    pub(crate) static yy_accept: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 5, 7, 8, 10, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37,
                39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89,
                91, 93, 95, 97, 100, 102, 103, 104, 104, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 114,
                115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134,
                135, 136, 137, 138, 139, 141, 142, 144, 145, 146, 147, 148, 150, 151, 152, 153, 154, 155, 156, 157,
                158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 167, 168, 168, 168, 169, 170, 172, 173, 174, 175,
                176, 177, 178, 180, 181, 182, 183, 184, 186, 187, 188, 189, 190, 191, 192, 193, 195, 196, 197, 198,
                199, 200, 201, 202, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219,
                220, 221, 222, 223, 224, 225, 225, 226, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 238,
                240, 241, 242, 243, 244, 245, 246, 247, 249, 250, 251, 252, 253, 254, 255, 257, 258, 259, 260, 261,
                262, 263, 264, 265, 266, 267, 268, 270, 271, 272, 273, 274, 275, 276, 278, 280, 282, 284, 285, 286,
                287, 288, 290, 292, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 308, 310, 311,
                312, 313, 314, 316, 318, 320, 321, 322, 324, 325, 326, 327, 328, 329, 330, 331, 332, 333, 334, 335,
                336, 337, 339, 340, 341, 342, 343, 344, 345, 346, 347, 349, 350, 351, 352, 353, 354, 355, 356, 357,
                359, 361, 362, 363, 364, 366, 367, 368, 369, 370, 371, 373, 375, 376, 377, 379, 381, 382, 384, 385,
                386, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 399, 400, 401, 403, 404, 405, 406, 408, 409,
                411, 412, 413, 414, 415, 416, 417, 418, 420, 421, 422, 424, 426, 427, 428, 430, 431, 433, 435, 437,
                438, 440, 441, 442, 443, 444, 445, 447, 448, 450, 451, 452, 453, 454, 455, 456, 458, 460, 462, 463,
                465, 467, 468, 470, 471, 472, 474, 475, 476, 477, 478, 480, 481, 483, 484, 486, 488, 490, 490
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_ec: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 1, 1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 6, 7,
                6, 6, 6, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 18, 19, 20, 21,
                22, 6, 6, 23, 23, 23, 23, 24, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
                23, 23, 23, 25, 26, 27, 28, 23, 1, 29, 30, 31, 32, 33, 34, 35, 36, 37, 23, 38, 39, 40, 41, 42, 43, 44,
                45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 6, 55, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_meta: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                1, 1, 1, 1, 2, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 5, 2, 4, 4, 2, 5, 2, 2, 6, 6, 4, 4, 4, 6,
                4, 4, 4, 4, 4, 4, 6, 4, 4, 4, 6, 4, 4, 4, 6, 4, 4, 6, 4, 2, 2
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_base: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                0, 0, 53, 54, 449, 448, 55, 56, 450, 453, 64, 453, 446, 453, 422, 453, 453, 453, 453, 453, 453, 56, 60,
                62, 57, 453, 39, 426, 425, 0, 453, 453, 453, 44, 35, 48, 55, 60, 68, 62, 403, 402, 401, 69, 77, 409,
                46, 79, 72, 453, 453, 453, 453, 425, 453, 453, 453, 453, 453, 93, 118, 453, 113, 0, 453, 453, 453, 453,
                109, 453, 453, 453, 110, 113, 124, 453, 453, 453, 453, 453, 453, 0, 405, 103, 397, 405, 408, 395, 95,
                389, 403, 387, 116, 384, 102, 392, 389, 387, 383, 386, 0, 383, 113, 383, 392, 376, 118, 0, 375, 388,
                121, 378, 68, 126, 374, 388, 384, 368, 372, 122, 367, 453, 453, 146, 453, 162, 144, 166, 396, 395, 369,
                0, 368, 378, 379, 361, 121, 369, 0, 374, 368, 370, 373, 0, 361, 371, 370, 365, 351, 367, 345, 0, 363,
                133, 346, 359, 343, 347, 356, 0, 343, 350, 127, 341, 347, 142, 337, 344, 349, 339, 347, 340, 330, 344,
                329, 334, 341, 340, 331, 332, 334, 352, 351, 350, 349, 320, 317, 325, 324, 315, 327, 312, 317, 312, 0,
                145, 313, 322, 307, 312, 143, 319, 312, 0, 303, 304, 303, 310, 301, 298, 0, 305, 314, 302, 296, 292,
                300, 309, 297, 299, 302, 297, 0, 288, 301, 302, 285, 300, 276, 0, 0, 0, 0, 294, 289, 288, 295, 0, 0, 0,
                292, 156, 289, 288, 286, 283, 272, 272, 279, 283, 282, 272, 0, 0, 275, 264, 277, 280, 0, 0, 0, 261,
                270, 0, 259, 263, 269, 270, 273, 270, 269, 267, 259, 266, 255, 255, 251, 0, 252, 245, 244, 243, 248,
                259, 239, 239, 0, 252, 236, 254, 240, 252, 234, 250, 236, 0, 0, 238, 234, 222, 0, 245, 240, 225, 232,
                223, 0, 0, 240, 235, 0, 0, 234, 0, 230, 228, 222, 216, 225, 220, 227, 218, 219, 210, 215, 225, 0, 215,
                212, 0, 207, 222, 218, 0, 216, 0, 215, 202, 217, 203, 204, 201, 197, 0, 200, 203, 0, 0, 210, 201, 0,
                198, 0, 0, 0, 189, 0, 190, 202, 200, 202, 195, 0, 185, 0, 188, 153, 152, 156, 164, 159, 0, 0, 0, 155,
                0, 0, 161, 0, 159, 150, 0, 148, 154, 156, 131, 0, 91, 0, 39, 0, 0, 0, 453, 201, 207, 213, 218, 221,
                225
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_def: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                394, 1, 395, 395, 396, 396, 397, 397, 394, 394, 394, 394, 394, 394, 398, 394, 394, 394, 394, 394, 394,
                394, 394, 394, 394, 394, 394, 394, 394, 399, 394, 394, 394, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394,
                394, 398, 400, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                394, 394, 394, 394, 394, 394, 394, 394, 394, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                394, 394, 394, 394, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399,
                399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 0, 394, 394, 394, 394, 394, 394
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_nxt: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                10, 11, 12, 13, 11, 10, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 30, 31, 10,
                32, 33, 34, 35, 36, 37, 38, 39, 30, 30, 40, 30, 41, 42, 43, 44, 45, 30, 46, 47, 48, 30, 30, 49, 30, 30,
                30, 50, 51, 53, 53, 58, 58, 78, 79, 59, 59, 54, 54, 61, 65, 66, 61, 67, 71, 68, 69, 85, 76, 72, 73, 77,
                74, 86, 60, 60, 83, 70, 84, 75, 87, 89, 91, 88, 393, 90, 115, 116, 75, 101, 96, 168, 92, 123, 93, 102,
                103, 94, 97, 111, 98, 120, 121, 99, 95, 107, 169, 108, 117, 100, 109, 110, 124, 61, 125, 112, 61, 118,
                113, 69, 127, 73, 138, 74, 119, 392, 126, 128, 132, 129, 75, 129, 64, 139, 130, 126, 128, 133, 147, 75,
                143, 144, 148, 155, 161, 165, 123, 156, 179, 157, 170, 171, 180, 215, 127, 191, 391, 145, 162, 166,
                192, 128, 172, 216, 219, 124, 173, 182, 206, 182, 128, 184, 183, 184, 207, 245, 185, 251, 283, 390,
                389, 252, 220, 388, 387, 386, 385, 384, 246, 383, 382, 381, 380, 379, 284, 52, 52, 52, 52, 52, 52, 55,
                55, 55, 55, 55, 55, 57, 57, 57, 57, 57, 57, 63, 378, 63, 63, 63, 82, 377, 82, 63, 376, 63, 63, 375,
                374, 373, 372, 371, 370, 369, 368, 367, 366, 365, 364, 363, 362, 361, 360, 359, 358, 357, 356, 355,
                354, 353, 352, 351, 350, 349, 348, 347, 346, 345, 344, 343, 342, 341, 340, 339, 338, 337, 336, 335,
                334, 333, 332, 331, 330, 329, 328, 327, 326, 325, 324, 323, 322, 321, 320, 319, 318, 317, 316, 315,
                314, 313, 312, 311, 310, 309, 308, 307, 306, 305, 304, 303, 302, 301, 300, 299, 298, 297, 296, 295,
                294, 293, 292, 291, 290, 289, 288, 287, 286, 285, 282, 281, 280, 279, 278, 277, 276, 275, 274, 273,
                272, 271, 270, 269, 268, 267, 266, 265, 264, 263, 262, 261, 260, 259, 258, 257, 256, 255, 254, 253,
                250, 249, 248, 247, 244, 243, 242, 241, 240, 239, 238, 237, 236, 185, 185, 183, 183, 235, 234, 233,
                232, 231, 230, 229, 228, 227, 226, 225, 224, 223, 222, 221, 218, 217, 214, 213, 212, 211, 210, 209,
                208, 205, 204, 203, 202, 201, 200, 199, 198, 197, 196, 195, 194, 193, 190, 189, 188, 187, 186, 130,
                130, 181, 178, 177, 176, 175, 174, 167, 164, 163, 160, 159, 158, 154, 153, 152, 151, 150, 149, 146,
                142, 141, 140, 137, 136, 135, 134, 131, 122, 114, 106, 105, 104, 81, 80, 64, 62, 394, 56, 56, 9, 394,
                394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394,
                394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394,
                394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });

    pub(crate) static yy_chk: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| {
        metamodelica::StaticArray::new(
            list![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 4, 7, 8, 27, 27, 7, 8, 3, 4, 11, 22,
                22, 11, 22, 23, 22, 22, 35, 25, 23, 24, 25, 24, 35, 7, 8, 34, 22, 34, 24, 36, 37, 38, 36, 390, 37, 47,
                47, 24, 40, 39, 113, 38, 60, 38, 40, 40, 38, 39, 45, 39, 49, 49, 39, 38, 44, 113, 44, 48, 39, 44, 44,
                60, 61, 63, 45, 61, 48, 45, 69, 73, 74, 89, 74, 48, 388, 69, 73, 84, 75, 74, 75, 63, 89, 75, 69, 73,
                84, 95, 74, 93, 93, 95, 103, 107, 111, 124, 103, 120, 103, 114, 114, 120, 163, 127, 137, 386, 93, 107,
                111, 137, 127, 114, 163, 166, 124, 114, 126, 154, 126, 127, 128, 126, 128, 154, 196, 128, 201, 242,
                385, 384, 201, 166, 383, 381, 380, 378, 375, 196, 371, 370, 369, 368, 367, 242, 395, 395, 395, 395,
                395, 395, 396, 396, 396, 396, 396, 396, 397, 397, 397, 397, 397, 397, 398, 366, 398, 398, 398, 399,
                364, 399, 400, 362, 400, 400, 361, 360, 359, 358, 356, 352, 350, 349, 346, 345, 343, 342, 341, 340,
                339, 338, 337, 335, 333, 332, 331, 329, 328, 326, 325, 324, 323, 322, 321, 320, 319, 318, 317, 316,
                315, 313, 310, 309, 306, 305, 304, 303, 302, 300, 299, 298, 295, 294, 293, 292, 291, 290, 289, 288,
                286, 285, 284, 283, 282, 281, 280, 279, 277, 276, 275, 274, 273, 272, 271, 270, 269, 268, 267, 266,
                265, 263, 262, 258, 257, 256, 255, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 241, 237, 236,
                235, 234, 229, 228, 227, 226, 225, 224, 222, 221, 220, 219, 218, 217, 216, 215, 214, 213, 212, 210,
                209, 208, 207, 206, 205, 203, 202, 200, 199, 198, 197, 194, 193, 192, 191, 190, 189, 188, 187, 186,
                185, 184, 183, 182, 181, 180, 179, 178, 177, 176, 175, 174, 173, 172, 171, 170, 169, 168, 167, 165,
                164, 162, 161, 159, 158, 157, 156, 155, 153, 151, 150, 149, 148, 147, 146, 145, 143, 142, 141, 140,
                138, 136, 135, 134, 133, 131, 130, 129, 121, 119, 118, 117, 116, 115, 112, 110, 109, 106, 105, 104,
                102, 100, 99, 98, 97, 96, 94, 92, 91, 90, 88, 87, 86, 85, 83, 54, 46, 43, 42, 41, 29, 28, 15, 13, 9, 6,
                5, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394,
                394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394,
                394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394
            ]
            .into_iter()
            .cloned()
            .collect(),
        )
    });
}

pub(crate) fn scan(mut fileName: ArcStr) -> Result<(metamodelica::List<Token>, metamodelica::List<Token>)> {
    let mut tokens: metamodelica::List<Token>;
    let mut errorTokens: metamodelica::List<Token>;
    let mut contents: ArcStr;
    contents = System::readFile(fileName.clone())?;
    (tokens, errorTokens) = lex(fileName, contents)?;
    Ok((tokens, errorTokens))
}

pub fn scanString(
    mut fileSource: ArcStr,
    mut fileName: ArcStr,
) -> Result<(metamodelica::List<Token>, metamodelica::List<Token>)> {
    let mut tokens: metamodelica::List<Token>;
    let mut errorTokens: metamodelica::List<Token>;
    (tokens, errorTokens) = lex(fileName, fileSource)?;
    Ok((tokens, errorTokens))
}

pub(crate) fn action(
    mut act: i32,
    mut startSt: i32,
    mut mm_currSt: i32,
    mut mm_pos: i32,
    mut mm_sPos: i32,
    mut mm_ePos: i32,
    mut mm_linenr: i32,
    mut lineNrStart: i32,
    mut buffer: i32,
    mut fileNm: ArcStr,
    mut fileContents: ArcStr,
    mut inErrorTokens: metamodelica::List<Token>,
) -> Result<(Token, i32, i32, metamodelica::List<Token>)> {
    let mut token: Token;
    let mut mm_startSt: i32;
    let mut bufferRet: i32;
    let mut errorTokens: metamodelica::List<Token> = inErrorTokens;
    mm_startSt = startSt;
    bufferRet = 0;
    token = (match act {
        1 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::WHITESPACE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        2 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::NEWLINE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        3 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::UNSIGNED_REAL.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        4 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::UNSIGNED_REAL.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        5 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::UNSIGNED_REAL.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        6 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::ALGORITHM.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        7 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::AND.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        8 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::ANNOTATION.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        9 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::BLOCK.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        10 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::CLASS.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        11 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::CONNECT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        12 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::CONNECTOR.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        13 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::CONSTANT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        14 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::DISCRETE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        15 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::DER.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        16 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::DEFINEUNIT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        17 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::EACH.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        18 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::ELSE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        19 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::ELSEIF.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        20 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::ELSEWHEN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        21 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::END.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        22 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::ENUMERATION.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        23 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::EQUATION.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        24 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::ENCAPSULATED.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        25 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::EXPANDABLE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        26 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::EXTENDS.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        27 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::CONSTRAINEDBY.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        28 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::EXTERNAL.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        29 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::FALSE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        30 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::FINAL.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        31 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::FLOW.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        32 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::FOR.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        33 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::FUNCTION.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        34 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::IF.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        35 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::IMPORT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        36 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::IN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        37 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::INITIAL.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        38 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::INNER.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        39 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::INPUT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        40 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::LOOP.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        41 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::MODEL.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        42 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::NOT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        43 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::OUTER.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        44 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::OPERATOR.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        45 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::OVERLOAD.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        46 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::OR.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        47 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::OUTPUT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        48 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::PACKAGE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        49 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::PARAMETER.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        50 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::PARTIAL.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        51 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::PROTECTED.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        52 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::PUBLIC.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        53 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::RECORD.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        54 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::REDECLARE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        55 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::REPLACEABLE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        56 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::THEN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        57 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::TRUE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        58 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::TYPE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        59 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::WHEN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        60 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::WHILE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        61 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::WITHIN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        62 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::RETURN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        63 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::BREAK.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        64 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::LPAR.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        65 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::RPAR.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        66 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::LBRACK.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        67 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::RBRACK.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        68 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::LBRACE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        69 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::RBRACE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        70 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::EQEQ.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        71 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::EQUALS.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        72 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::COMMA.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        73 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::ASSIGN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        74 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::COLONCOLON.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        75 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::COLON.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        76 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::SEMICOLON.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        77 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::PURE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        78 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::IMPURE.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        79 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::OPTIMIZATION.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        80 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::PLUS_EW.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        81 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::MINUS_EW.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        82 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::STAR_EW.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        83 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::SLASH_EW.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        84 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::POWER_EW.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        85 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::STAR.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        86 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::MINUS.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        87 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::PLUS.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        88 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::LESSEQ.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        89 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::LESSGT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        90 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::LESS.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        91 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::GREATER.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        92 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::GREATEREQ.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        93 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::POWER.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        94 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::SLASH.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        95 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::SUBTYPEOF.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        96 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::STREAM.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        97 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::DOT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        98 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::IDENT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        99 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::IDENT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        100 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::UNSIGNED_INTEGER.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        101 => {
            mm_startSt = 7;
            bufferRet = buffer;
            noToken.clone()
        }
        102 => {
            bufferRet = buffer;
            noToken.clone()
        }
        103 => {
            bufferRet = buffer;
            noToken.clone()
        }
        104 => {
            let mut tok: Token;
            mm_startSt = 1;
            tok = Token {
                fileName: fileNm,
                id: TokenId::STRING.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        105 => {
            bufferRet = buffer;
            noToken.clone()
        }
        106 => {
            bufferRet = buffer;
            noToken.clone()
        }
        107 => {
            mm_startSt = 3;
            bufferRet = buffer;
            noToken.clone()
        }
        108 => {
            let mut tok: Token;
            mm_startSt = 1;
            tok = Token {
                fileName: fileNm,
                id: TokenId::BLOCK_COMMENT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        109 => {
            bufferRet = buffer;
            noToken.clone()
        }
        110 => {
            bufferRet = buffer;
            noToken.clone()
        }
        111 => {
            mm_startSt = 5;
            bufferRet = buffer;
            noToken.clone()
        }
        112 => {
            let mut tok: Token;
            mm_startSt = 1;
            tok = Token {
                fileName: fileNm,
                id: TokenId::LINE_COMMENT.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            tok
        }
        113 => {
            bufferRet = buffer;
            noToken.clone()
        }
        114 => {
            let mut tok: Token;
            tok = Token {
                fileName: fileNm,
                id: TokenId::_NO_TOKEN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            errorTokens = metamodelica::cons(tok, errorTokens);
            noToken.clone()
        }
        _ => {
            let mut tok: Token;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nLexer unknown rule, action="));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", act)));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            tok = Token {
                fileName: fileNm,
                id: TokenId::_NO_TOKEN.clone(),
                fileContents: fileContents,
                byteOffset: mm_pos - buffer,
                length: buffer,
                lineNumberStart: lineNrStart,
                columnNumberStart: mm_ePos + 1,
                lineNumberEnd: mm_linenr,
                columnNumberEnd: mm_sPos + 1,
            };
            metamodelica::print(printToken(tok)?);
            return Err("fail");
        }
    });
    Ok((token, mm_startSt, bufferRet, errorTokens))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum TokenId {
    _NO_TOKEN = 1,
    ALGORITHM = 2,
    AND = 3,
    ANNOTATION = 4,
    ASSIGN = 5,
    BLOCK = 6,
    BLOCK_COMMENT = 7,
    BREAK = 8,
    CLASS = 9,
    COLON = 10,
    COLONCOLON = 11,
    COMMA = 12,
    CONNECT = 13,
    CONNECTOR = 14,
    CONSTANT = 15,
    CONSTRAINEDBY = 16,
    DEFINEUNIT = 17,
    DER = 18,
    DISCRETE = 19,
    DOT = 20,
    EACH = 21,
    ELSE = 22,
    ELSEIF = 23,
    ELSEWHEN = 24,
    ENCAPSULATED = 25,
    END = 26,
    ENUMERATION = 27,
    EQEQ = 28,
    EQUALS = 29,
    EQUATION = 30,
    EXPANDABLE = 31,
    EXTENDS = 32,
    EXTERNAL = 33,
    FALSE = 34,
    FINAL = 35,
    FLOW = 36,
    FOR = 37,
    FUNCTION = 38,
    GREATER = 39,
    GREATEREQ = 40,
    IDENT = 41,
    IF = 42,
    IMPORT = 43,
    IMPURE = 44,
    IN = 45,
    INITIAL = 46,
    INNER = 47,
    INPUT = 48,
    LBRACE = 49,
    LBRACK = 50,
    LESS = 51,
    LESSEQ = 52,
    LESSGT = 53,
    LINE_COMMENT = 54,
    LOOP = 55,
    LPAR = 56,
    MINUS = 57,
    MINUS_EW = 58,
    MODEL = 59,
    MODELICA = 60,
    NEWLINE = 61,
    NOT = 62,
    OPERATOR = 63,
    OPTIMIZATION = 64,
    OR = 65,
    OUTER = 66,
    OUTPUT = 67,
    OVERLOAD = 68,
    PACKAGE = 69,
    PARAMETER = 70,
    PARTIAL = 71,
    PLUS = 72,
    PLUS_EW = 73,
    POWER = 74,
    POWER_EW = 75,
    PROTECTED = 76,
    PUBLIC = 77,
    PURE = 78,
    RBRACE = 79,
    RBRACK = 80,
    RECORD = 81,
    REDECLARE = 82,
    REPLACEABLE = 83,
    RETURN = 84,
    RPAR = 85,
    SEMICOLON = 86,
    SLASH = 87,
    SLASH_EW = 88,
    STAR = 89,
    STAR_EW = 90,
    STREAM = 91,
    STRING = 92,
    SUBTYPEOF = 93,
    THEN = 94,
    TRUE = 95,
    TYPE = 96,
    UNSIGNED_INTEGER = 97,
    UNSIGNED_REAL = 98,
    WHEN = 99,
    WHILE = 100,
    WHITESPACE = 101,
    WITHIN = 102,
}
impl PartialOrd for TokenId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TokenId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for TokenId {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for TokenId {
    fn default() -> Self {
        Self::_NO_TOKEN
    }
}

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Token {
    pub fileName: ArcStr,
    pub id: TokenId,
    pub fileContents: ArcStr,
    pub byteOffset: i32,
    pub length: i32,
    pub lineNumberStart: i32,
    pub columnNumberStart: i32,
    pub lineNumberEnd: i32,
    pub columnNumberEnd: i32,
}

impl metamodelica::gc::MMTrace for Token {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fileName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.id, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fileContents, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.byteOffset, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.length, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lineNumberStart, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.columnNumberStart, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lineNumberEnd, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.columnNumberEnd, __mmv)?;
        Ok(())
    }
}
impl Default for Token {
    fn default() -> Self {
        Self {
            fileName: Default::default(),
            id: Default::default(),
            fileContents: Default::default(),
            byteOffset: Default::default(),
            length: Default::default(),
            lineNumberStart: Default::default(),
            columnNumberStart: Default::default(),
            lineNumberEnd: Default::default(),
            columnNumberEnd: Default::default(),
        }
    }
}

pub type TOKEN = Token;

pub(crate) static noToken: std::sync::LazyLock<Token> = std::sync::LazyLock::new(|| Token {
    fileName: literal!("<NoFile>"),
    id: TokenId::_NO_TOKEN.clone(),
    fileContents: literal!(""),
    byteOffset: 0,
    length: 0,
    lineNumberStart: 0,
    columnNumberStart: 0,
    lineNumberEnd: 0,
    columnNumberEnd: 0,
});

pub(crate) fn printToken(mut token: Token) -> Result<ArcStr> {
    let mut strTk: ArcStr;
    let mut id: TokenId;
    let mut contents: ArcStr;
    let mut byteOffset: i32;
    let mut length: i32;
    let Token {
        id: __pa0,
        fileContents: __pa1,
        byteOffset: __pa2,
        length: __pa3,
        ..
    } = &token;
    id = metamodelica::Own::own(__pa0);
    contents = metamodelica::Own::own(__pa1);
    byteOffset = metamodelica::Own::own(__pa2);
    length = metamodelica::Own::own(__pa3);
    contents = if (length > 0) {
        substring(contents, byteOffset, byteOffset + length - 1)?
    } else {
        literal!("")
    };
    strTk = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("[TOKEN:"));
        __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", id)));
        __mm_s.push_str(&*literal!(" '"));
        __mm_s.push_str(&*contents);
        __mm_s.push_str(&*literal!("' ("));
        __mm_s.push_str(&*intString(token.lineNumberStart.clone()));
        __mm_s.push_str(&*literal!(":"));
        __mm_s.push_str(&*intString(token.columnNumberStart.clone()));
        __mm_s.push_str(&*literal!("-"));
        __mm_s.push_str(&*intString(token.lineNumberEnd.clone()));
        __mm_s.push_str(&*literal!(":"));
        __mm_s.push_str(&*intString(token.columnNumberEnd.clone()));
        __mm_s.push_str(&*literal!(")]"));
        ArcStr::from(__mm_s)
    };
    Ok(strTk)
}

pub fn tokenContent(mut token: Token) -> Result<ArcStr> {
    let mut contents: ArcStr;
    let mut byteOffset: i32;
    let mut length: i32;
    let Token {
        fileContents: __pa0,
        byteOffset: __pa1,
        length: __pa2,
        ..
    } = token;
    contents = metamodelica::Own::own(__pa0);
    byteOffset = metamodelica::Own::own(__pa1);
    length = metamodelica::Own::own(__pa2);
    contents = if (length > 0) {
        substring(contents, byteOffset, byteOffset + length - 1)?
    } else {
        literal!("")
    };
    Ok(contents)
}

pub(crate) fn tokenContentEq(mut token1: Token, mut token2: Token) -> bool {
    let mut b: bool;
    let mut contents1: ArcStr;
    let mut contents2: ArcStr;
    let mut offset1: i32;
    let mut length1: i32;
    let mut offset2: i32;
    let mut length2: i32;
    let Token {
        fileContents: __pa0,
        byteOffset: __pa1,
        length: __pa2,
        ..
    } = token1;
    contents1 = metamodelica::Own::own(__pa0);
    offset1 = metamodelica::Own::own(__pa1);
    length1 = metamodelica::Own::own(__pa2);
    let Token {
        fileContents: __pa3,
        byteOffset: __pa4,
        length: __pa5,
        ..
    } = token2;
    contents2 = metamodelica::Own::own(__pa3);
    offset2 = metamodelica::Own::own(__pa4);
    length2 = metamodelica::Own::own(__pa5);
    b = if (length1 != length2) {
        false
    } else {
        0 == System::strcmp_offset(contents1, offset1, length1, contents2, offset2, length2)
    };
    b
}

pub(crate) fn tokenSourceInfo(mut token: Token) -> SourceInfo {
    let mut info: SourceInfo;
    info = {
        let mut t = token;
        (match t.clone() {
            Token { .. } => SourceInfo {
                fileName: t.fileName.clone(),
                isReadOnly: false,
                lineNumberStart: t.lineNumberStart.clone(),
                columnNumberStart: t.columnNumberStart.clone(),
                lineNumberEnd: t.lineNumberEnd.clone(),
                columnNumberEnd: t.columnNumberEnd.clone(),
                lastModification: metamodelica::OrderedFloat(0.0_f64),
            },
        })
    };
    info
}

fn lex(mut fileName: ArcStr, mut contents: ArcStr) -> Result<(metamodelica::List<Token>, metamodelica::List<Token>)> {
    let mut tokens: metamodelica::List<Token>;
    let mut errorTokens: metamodelica::List<Token> = metamodelica::nil();
    let mut startSt: i32;
    let mut i: i32;
    let mut cTok: i32;
    let mut currSt: i32;
    let mut pos: i32;
    let mut sPos: i32;
    let mut ePos: i32;
    let mut linenr: i32;
    let mut contentLen: i32;
    let mut numBacktrack: i32;
    let mut buffer: i32;
    let mut lineNrStart: i32;
    let mut states: metamodelica::List<i32>;
    startSt = 1;
    currSt = 1;
    pos = 1;
    sPos = 0;
    ePos = 0;
    linenr = 1;
    lineNrStart = 1;
    buffer = 0;
    states = metamodelica::nil();
    if debug.clone() == true {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nLexer analyzer LexerCode..."));
            __mm_s.push_str(&*fileName);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    tokens = metamodelica::nil();
    if debug.clone() {
        metamodelica::print(literal!("\n TOTAL Chars:"));
        metamodelica::print(intString(((contents).len() as i32)));
    }
    contentLen = ((contents).len() as i32);
    i = 1;
    while i <= contentLen {
        cTok = stringGet(&contents, i)?;
        (
            tokens,
            numBacktrack,
            startSt,
            currSt,
            pos,
            sPos,
            ePos,
            linenr,
            lineNrStart,
            buffer,
            states,
            errorTokens,
        ) = consume(
            cTok,
            tokens,
            contents.clone(),
            startSt,
            currSt,
            pos,
            sPos,
            ePos,
            linenr,
            lineNrStart,
            buffer,
            states,
            fileName.clone(),
            errorTokens,
        )?;
        i = i - numBacktrack + 1;
    }
    tokens = metamodelica::Dangerous::listReverseInPlace(tokens);
    errorTokens = metamodelica::Dangerous::listReverseInPlace(errorTokens);
    Ok((tokens, errorTokens))
}

fn consume(
    mut cp: i32,
    mut tokens: metamodelica::List<Token>,
    mut fileContents: ArcStr,
    mut startSt: i32,
    mut currSt: i32,
    mut pos: i32,
    mut sPos: i32,
    mut ePos: i32,
    mut linenr: i32,
    mut inLineNrStart: i32,
    mut inBuffer: i32,
    mut inStates: metamodelica::List<i32>,
    mut fileName: ArcStr,
    mut inErrorTokens: metamodelica::List<Token>,
) -> Result<(
    metamodelica::List<Token>,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    metamodelica::List<i32>,
    metamodelica::List<Token>,
)> {
    let mut resToken: metamodelica::List<Token>;
    let mut bkBuffer: i32 = 0;
    let mut mm_startSt: i32;
    let mut mm_currSt: i32;
    let mut mm_pos: i32;
    let mut mm_sPos: i32;
    let mut mm_ePos: i32;
    let mut mm_linenr: i32;
    let mut lineNrStart: i32;
    let mut buffer: i32;
    let mut states: metamodelica::List<i32>;
    let mut errorTokens: metamodelica::List<Token> = inErrorTokens;
    let mut tok: Token;
    let mut act: i32;
    let mut buffer2: i32;
    let mut c: i32;
    let mut baseCond: i32;
    mm_startSt = startSt;
    mm_currSt = currSt;
    mm_pos = pos;
    mm_sPos = sPos;
    mm_ePos = ePos;
    mm_linenr = linenr;
    lineNrStart = inLineNrStart;
    buffer = inBuffer;
    states = inStates;
    baseCond = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_base.borrow(), mm_currSt)?).clone();
        __elt
    });
    if debug.clone() == true {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nPROGRAM:{"));
            __mm_s.push_str(&*intString(cp));
            __mm_s.push_str(&*literal!("} "));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nBUFFER:{"));
            __mm_s.push_str(&*intString(buffer));
            __mm_s.push_str(&*literal!("} "));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("base:"));
            __mm_s.push_str(&*intString(baseCond));
            __mm_s.push_str(&*literal!(" st:"));
            __mm_s.push_str(&*intString(mm_currSt));
            __mm_s.push_str(&*literal!(" "));
            ArcStr::from(__mm_s)
        });
    }
    buffer = buffer + 1;
    mm_pos = mm_pos + 1;
    if cp == 10 {
        mm_linenr = mm_linenr + 1;
        mm_sPos = 0;
    } else {
        mm_sPos = mm_sPos + 1;
    }
    if debug.clone() == true {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n[Reading:'"));
            __mm_s.push_str(&*intStringChar(cp));
            __mm_s.push_str(&*literal!("' at p:"));
            __mm_s.push_str(&*intString(mm_pos - 1));
            __mm_s.push_str(&*literal!(" line:"));
            __mm_s.push_str(&*intString(mm_linenr));
            __mm_s.push_str(&*literal!(" rPos:"));
            __mm_s.push_str(&*intString(mm_sPos));
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        });
    }
    c = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_ec.borrow(), cp)?).clone();
        __elt
    });
    if debug.clone() == true {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" evalState Before[c"));
            __mm_s.push_str(&*intString(c));
            __mm_s.push_str(&*literal!(",s"));
            __mm_s.push_str(&*intString(mm_currSt));
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        });
    }
    (mm_currSt, c) = evalState(mm_currSt, c)?;
    if debug.clone() == true {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" After[c"));
            __mm_s.push_str(&*intString(c));
            __mm_s.push_str(&*literal!(",s"));
            __mm_s.push_str(&*intString(mm_currSt));
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        });
    }
    if mm_currSt > 0 {
        mm_currSt = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_base.borrow(), mm_currSt)?).clone();
            __elt
        });
        mm_currSt = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_nxt.borrow(), mm_currSt + c)?).clone();
            __elt
        });
    } else {
        mm_currSt = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_nxt.borrow(), c)?).clone();
            __elt
        });
    }
    states = metamodelica::cons(mm_currSt, states);
    baseCond = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_base.borrow(), mm_currSt)?).clone();
        __elt
    });
    if baseCond == LexTable::yy_finish.clone() {
        if debug.clone() == true {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n[RESTORE="));
                __mm_s.push_str(&*intString(
                    ({
                        let __elt = (*metamodelica::index_checked(&LexTable::yy_accept.borrow(), mm_currSt)?).clone();
                        __elt
                    }),
                ));
                __mm_s.push_str(&*literal!("]"));
                ArcStr::from(__mm_s)
            });
        }
        (act, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states) = findRule(
            &fileContents,
            mm_currSt,
            mm_pos,
            mm_sPos,
            mm_ePos,
            mm_linenr,
            buffer,
            bkBuffer,
            states,
        )?;
        if debug.clone() == true {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nFound rule: "));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", act)));
                ArcStr::from(__mm_s)
            });
        }
        (tok, mm_startSt, buffer2, errorTokens) = action(
            act,
            mm_startSt,
            mm_currSt,
            mm_pos,
            mm_sPos,
            mm_ePos,
            mm_linenr,
            lineNrStart,
            buffer,
            fileName,
            fileContents,
            errorTokens,
        )?;
        if debug.clone() == true {
            metamodelica::print(literal!("\nDid action"));
        }
        mm_currSt = mm_startSt;
        states = metamodelica::nil();
        if buffer != buffer2 {
            mm_ePos = mm_sPos;
            lineNrStart = linenr;
        }
        buffer = buffer2;
        resToken = (match tok.clone() {
            Token {
                id: TokenId::_NO_TOKEN, ..
            } => tokens,
            _ => metamodelica::cons(tok, tokens),
        });
        if debug.clone() {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n CountTokens:"));
                __mm_s.push_str(&*intString(((resToken).len() as i32)));
                ArcStr::from(__mm_s)
            });
        }
    } else {
        bkBuffer = 0;
        resToken = tokens;
    }
    Ok((
        resToken,
        bkBuffer,
        mm_startSt,
        mm_currSt,
        mm_pos,
        mm_sPos,
        mm_ePos,
        mm_linenr,
        lineNrStart,
        buffer,
        states,
        errorTokens,
    ))
}

fn findRule<'__b>(
    mut fileContents: &'__b ArcStr,
    mut currSt: i32,
    mut pos: i32,
    mut sPos: i32,
    mut mm_ePos: i32,
    mut linenr: i32,
    mut inBuffer: i32,
    mut inBkBuffer: i32,
    mut inStates: metamodelica::List<i32>,
) -> Result<(i32, i32, i32, i32, i32, i32, i32, metamodelica::List<i32>)> {
    let mut action: i32;
    let mut mm_currSt: i32;
    let mut mm_pos: i32;
    let mut mm_sPos: i32;
    let mut mm_linenr: i32;
    let mut buffer: i32;
    let mut bkBuffer: i32;
    let mut states: metamodelica::List<i32>;
    let mut lp: i32;
    let mut lp1: i32;
    let mut stCmp: i32;
    let mut cp: i32;
    let mut st: bool;
    mm_currSt = currSt;
    mm_pos = pos;
    mm_sPos = sPos;
    mm_linenr = linenr;
    buffer = inBuffer;
    bkBuffer = inBkBuffer;
    states = inStates;
    stCmp = (states).get(1)?;
    lp = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_accept.borrow(), stCmp)?).clone();
        __elt
    });
    lp1 = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_accept.borrow(), stCmp + 1)?).clone();
        __elt
    });
    st = intGt(lp, 0) && intLt(lp, lp1);
    if st {
        if debug.clone() {
            checkArrayModelica(
                LexTable::yy_accept.clone(),
                stCmp,
                metamodelica::sourceInfo!("Lexers/LexerModelicaDiff.mo"),
            )?;
            checkArrayModelica(
                LexTable::yy_acclist.clone(),
                lp,
                metamodelica::sourceInfo!("Lexers/LexerModelicaDiff.mo"),
            )?;
        }
        lp = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_accept.borrow(), stCmp)?).clone();
            __elt
        });
        action = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_acclist.borrow(), lp)?).clone();
            __elt
        });
    } else {
        cp = stringGet(&fileContents, mm_pos - 1)?;
        buffer = buffer - 1;
        bkBuffer = bkBuffer + 1;
        mm_pos = mm_pos - 1;
        mm_sPos = mm_sPos - 1;
        if cp == 10 {
            mm_sPos = mm_ePos;
            mm_linenr = mm_linenr - 1;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(states) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        mm_currSt = metamodelica::Own::own(__pa0);
        states = metamodelica::Own::own(__pa1);
        (action, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states) = findRule(
            fileContents,
            mm_currSt,
            mm_pos,
            mm_sPos,
            mm_ePos,
            mm_linenr,
            buffer,
            bkBuffer,
            states,
        )?;
    }
    Ok((action, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states))
}

fn evalState(mut cState: i32, mut c: i32) -> Result<(i32, i32)> {
    let mut new_state: i32;
    let mut new_c: i32;
    let mut cState1: i32 = cState;
    let mut c1: i32 = c;
    let mut val: i32;
    let mut val2: i32;
    let mut chk: i32;
    chk = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_base.borrow(), cState1)?).clone();
        __elt
    });
    chk = chk + c1;
    val = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_chk.borrow(), chk)?).clone();
        __elt
    });
    val2 = ({
        let __elt = (*metamodelica::index_checked(&LexTable::yy_base.borrow(), cState1)?).clone();
        __elt
    }) + c1;
    if cState1 != val {
        cState1 = ({
            let __elt = (*metamodelica::index_checked(&LexTable::yy_def.borrow(), cState1)?).clone();
            __elt
        });
        if cState1 >= LexTable::yy_limit.clone() {
            c1 = ({
                let __elt = (*metamodelica::index_checked(&LexTable::yy_meta.borrow(), c1)?).clone();
                __elt
            });
        }
        if cState1 > 0 {
            (cState1, c1) = evalState(cState1, c1)?;
        }
    }
    new_state = cState1;
    new_c = c1;
    Ok((new_state, new_c))
}

fn checkArray<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut arr: metamodelica::Array<T>,
    mut index: i32,
    mut info: SourceInfo,
) -> Result<()> {
    let mut filename: ArcStr;
    let mut lineStart: i32;
    if index < 1 || index > metamodelica::arrayLength(arr.clone()) {
        let SourceInfo {
            fileName: __pa0,
            lineNumberStart: __pa1,
            ..
        } = (info)
        else {
            return Err("pattern mismatch");
        };
        filename = metamodelica::Own::own(__pa0);
        lineStart = metamodelica::Own::own(__pa1);
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n["));
            __mm_s.push_str(&*filename);
            __mm_s.push_str(&*literal!(":"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", lineStart)));
            __mm_s.push_str(&*literal!("]: checkArray failed: arrayLength="));
            __mm_s.push_str(&*ArcStr::from(::std::format!(
                "{}",
                metamodelica::arrayLength(arr.clone())
            )));
            __mm_s.push_str(&*literal!(" index="));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        return Err("fail");
    }
    Ok(())
}

fn checkArrayModelica(mut arr: metamodelica::Array<i32>, mut index: i32, mut info: SourceInfo) -> Result<()> {
    let mut filename: ArcStr;
    let mut lineStart: i32;
    if index < 1 || index > metamodelica::arrayLength(arr.clone()) {
        let SourceInfo {
            fileName: __pa0,
            lineNumberStart: __pa1,
            ..
        } = (info)
        else {
            return Err("pattern mismatch");
        };
        filename = metamodelica::Own::own(__pa0);
        lineStart = metamodelica::Own::own(__pa1);
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n["));
            __mm_s.push_str(&*filename);
            __mm_s.push_str(&*literal!(":"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", lineStart)));
            __mm_s.push_str(&*literal!("]: checkArray failed: arrayLength="));
            __mm_s.push_str(&*ArcStr::from(::std::format!(
                "{}",
                metamodelica::arrayLength(arr.clone())
            )));
            __mm_s.push_str(&*literal!(" index="));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        return Err("fail");
    }
    Ok(())
}

pub fn modelicaDiffTokenEq(mut ta: Token, mut tb: Token) -> Result<bool> {
    let mut b: bool;
    let mut ida: TokenId;
    let mut idb: TokenId;
    let TOKEN { id: __pa0, .. } = &ta;
    ida = metamodelica::Own::own(__pa0);
    let TOKEN { id: __pa1, .. } = &tb;
    idb = metamodelica::Own::own(__pa1);
    if ida != idb {
        b = false;
        return Ok(b);
    }
    b = (match ida {
        TokenId::IDENT { .. } => tokenContentEq(ta, tb),
        TokenId::UNSIGNED_INTEGER => tokenContentEq(ta, tb),
        TokenId::UNSIGNED_REAL => stringReal(tokenContent(ta)?)? == stringReal(tokenContent(tb)?)?,
        TokenId::BLOCK_COMMENT => blockCommentCanonical(ta)? == blockCommentCanonical(tb)?,
        TokenId::LINE_COMMENT => tokenContentEq(ta, tb),
        TokenId::STRING { .. } => {
            b = tokenContentEq(ta.clone(), tb.clone());
            if !(b) {
                b = if (0 != StringUtil::findChar(tokenContent(ta.clone())?, stringCharInt(literal!("\n"))?, 1, 0)) {
                    blockCommentCanonical(ta)? == blockCommentCanonical(tb)?
                } else {
                    false
                };
            }
            b
        }
        TokenId::WHITESPACE => true,
        _ => true,
    });
    Ok(b)
}

pub fn modelicaDiffTokenWhitespace(mut t: Token) -> bool {
    let mut b: bool;
    let mut id: TokenId;
    let TOKEN { id: __pa0, .. } = t;
    id = metamodelica::Own::own(__pa0);
    b = id == TokenId::BLOCK_COMMENT.clone()
        || id == TokenId::LINE_COMMENT.clone()
        || id == TokenId::WHITESPACE.clone()
        || id == TokenId::NEWLINE.clone();
    b
}

pub fn filterModelicaDiff(
    mut diffs: metamodelica::List<(DiffAlgorithm::Diff, metamodelica::List<Token>)>,
    mut removeWhitespace: bool,
) -> Result<metamodelica::List<(DiffAlgorithm::Diff, metamodelica::List<Token>)>> {
    use openmodelica_util::DiffAlgorithm::Diff;
    let mut odiffs: metamodelica::List<(Diff, metamodelica::List<Token>)>;
    let mut addedLineComments: metamodelica::List<ArcStr>;
    let mut removedLineComments: metamodelica::List<ArcStr>;
    let mut addedBlockComments: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut removedBlockComments: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut simpleDiff: metamodelica::List<(Diff, Token)>;
    let mut tmp: metamodelica::List<(Diff, Token)>;
    let mut rest: metamodelica::List<(Diff, Token)>;
    let mut lastIsNewline: bool;
    let mut depth: i32;
    let () = (::match_deref::match_deref! { match &(diffs.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::ListNode::Nil } => {
            odiffs = diffs;
            return Ok(odiffs);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    odiffs = ({
        let mut __acc: metamodelica::List<(Diff, metamodelica::List<Token>)> = metamodelica::nil();
        for mut e in (diffs).into_iter().cloned() {
            if !(::match_deref::match_deref! { match &(e.clone()) {
                (DiffAlgorithm::Diff::Add, Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::WHITESPACE, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => !(removeWhitespace),
                (DiffAlgorithm::Diff::Add, Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => !(removeWhitespace),
                (_, Deref @ metamodelica::ListNode::Nil) => false,
                _ => true,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }) {
                continue;
            }
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
                (DiffAlgorithm::Diff::Delete, ts @ Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::WHITESPACE, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    (Diff::Equal.clone(), ts.clone())
                },
                (DiffAlgorithm::Diff::Delete, ts @ Deref @ metamodelica::ListNode::Cons { head: Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    (Diff::Equal.clone(), ts.clone())
                },
                _ => {
                    e.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc
    });
    simpleDiff = ({
        let mut __acc: metamodelica::List<(Diff, Token)> = metamodelica::nil();
        for mut e in (odiffs).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
                (DiffAlgorithm::Diff::Add, ts) => {
                    ({
                let mut __acc: metamodelica::List<(Diff, Token)> = metamodelica::nil();
                for mut t in (ts.clone()).into_iter().cloned() {
                    let __x = (Diff::Add.clone(), t.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                },
                (DiffAlgorithm::Diff::Equal, ts) => {
                    ({
                let mut __acc: metamodelica::List<(Diff, Token)> = metamodelica::nil();
                for mut t in (ts.clone()).into_iter().cloned() {
                    let __x = (Diff::Equal.clone(), t.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                },
                (DiffAlgorithm::Diff::Delete, ts) => {
                    ({
                let mut __acc: metamodelica::List<(Diff, Token)> = metamodelica::nil();
                for mut t in (ts.clone()).into_iter().cloned() {
                    let __x = (Diff::Delete.clone(), t.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                },
                _ => return Err("match: no arm matched"),
            } });
            __acc = __x.append(&__acc);
        }
        __acc
    });
    tmp = metamodelica::nil();
    lastIsNewline = false;
    depth = 2;
    while !((simpleDiff).is_empty()) {
        (lastIsNewline, simpleDiff, tmp) = (::match_deref::match_deref! { match &(simpleDiff) {
            Deref @ metamodelica::ListNode::Cons { head: e1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Delete, t1 @ Token { id: TokenId::NEWLINE, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Delete, t2 @ Token { id: TokenId::WHITESPACE, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: e2 @ (DiffAlgorithm::Diff::Equal, _), tail: __esc_rest } } } } => {
                rest = (*__esc_rest).clone();
                (false, metamodelica::cons(e1.clone(), metamodelica::cons((Diff::Equal.clone(), t1.clone()), metamodelica::cons((Diff::Equal.clone(), t2.clone()), metamodelica::cons(e2.clone(), rest.clone())))), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: e1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Delete, t @ Token { id: TokenId::WHITESPACE, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: e2 @ (DiffAlgorithm::Diff::Equal, _), tail: __esc_rest } } } => {
                rest = (*__esc_rest).clone();
                (false, metamodelica::cons(e1.clone(), metamodelica::cons((Diff::Equal.clone(), t.clone()), metamodelica::cons(e2.clone(), rest.clone()))), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: e1 @ (DiffAlgorithm::Diff::Equal, Token { id: t3, .. }), tail: rest } if (t3.clone() != TokenId::WHITESPACE.clone() && t3.clone() != TokenId::NEWLINE.clone() && (deleteWhitespaceFollowedByEqualNonWhitespace(rest.clone())?).0) => {
                let mut rest = (*rest).clone();
                (_, rest) = deleteWhitespaceFollowedByEqualNonWhitespace(rest.clone())?;
                (false, metamodelica::cons(e1.clone(), rest.clone()), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: (d1, t1), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t3, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t4, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t5, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (d2, t2), tail: __esc_rest } } } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone()) && (t4.clone() == TokenId::NEWLINE.clone() || t4.clone() == TokenId::WHITESPACE.clone()) && (t5.clone() == TokenId::NEWLINE.clone() || t5.clone() == TokenId::WHITESPACE.clone())) => {
                rest = (*__esc_rest).clone();
                (false, metamodelica::cons((Diff::Equal.clone(), t1.clone()), rest.clone()), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: (d1, t1), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t3, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t4, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (d2, t2), tail: __esc_rest } } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone()) && (t4.clone() == TokenId::NEWLINE.clone() || t4.clone() == TokenId::WHITESPACE.clone())) => {
                rest = (*__esc_rest).clone();
                (false, metamodelica::cons((Diff::Equal.clone(), t1.clone()), rest.clone()), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: (d1, t1), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t3, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (d2, t2), tail: __esc_rest } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone())) => {
                rest = (*__esc_rest).clone();
                (false, metamodelica::cons((Diff::Equal.clone(), t1.clone()), rest.clone()), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: (d1, t1), tail: Deref @ metamodelica::ListNode::Cons { head: (d3, tk3 @ Token { id: t3, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (d4, tk4 @ Token { id: t4, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (d5, tk5 @ Token { id: t5, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (d2, t2), tail: __esc_rest } } } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (d3.clone() == Diff::Equal.clone() || d3.clone() == Diff::Delete.clone()) && (d4.clone() == Diff::Equal.clone() || d4.clone() == Diff::Delete.clone()) && (d5.clone() == Diff::Equal.clone() || d5.clone() == Diff::Delete.clone()) && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone()) && (t4.clone() == TokenId::NEWLINE.clone() || t4.clone() == TokenId::WHITESPACE.clone()) && (t5.clone() == TokenId::NEWLINE.clone() || t5.clone() == TokenId::WHITESPACE.clone())) => {
                rest = (*__esc_rest).clone();
                (false, metamodelica::cons((Diff::Equal.clone(), t1.clone()), metamodelica::cons((Diff::Equal.clone(), tk3.clone()), metamodelica::cons((Diff::Equal.clone(), tk4.clone()), metamodelica::cons((Diff::Equal.clone(), tk5.clone()), rest.clone())))), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: (d1, t1), tail: Deref @ metamodelica::ListNode::Cons { head: (d3, tk3 @ Token { id: t3, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (d4, tk4 @ Token { id: t4, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (d2, t2), tail: __esc_rest } } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (d3.clone() == Diff::Equal.clone() || d3.clone() == Diff::Delete.clone()) && (d4.clone() == Diff::Equal.clone() || d4.clone() == Diff::Delete.clone()) && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone()) && (t4.clone() == TokenId::NEWLINE.clone() || t4.clone() == TokenId::WHITESPACE.clone())) => {
                rest = (*__esc_rest).clone();
                (false, metamodelica::cons((Diff::Equal.clone(), t1.clone()), metamodelica::cons((Diff::Equal.clone(), tk3.clone()), metamodelica::cons((Diff::Equal.clone(), tk4.clone()), rest.clone()))), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: (d1, t1), tail: Deref @ metamodelica::ListNode::Cons { head: (d3, tk3 @ Token { id: t3, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (d2, t2), tail: __esc_rest } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (d3.clone() == Diff::Equal.clone() || d3.clone() == Diff::Delete.clone()) && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone())) => {
                rest = (*__esc_rest).clone();
                (false, metamodelica::cons((Diff::Equal.clone(), t1.clone()), metamodelica::cons((Diff::Equal.clone(), tk3.clone()), rest.clone())), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::NEWLINE, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::WHITESPACE, .. }), tail: __esc_rest @ Deref @ metamodelica::ListNode::Cons { head: (_, Token { id: TokenId::NEWLINE, .. }), tail: _ } } } => {
                rest = (*__esc_rest).clone();
                (false, rest.clone(), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::NEWLINE, .. }), tail: __esc_rest @ Deref @ metamodelica::ListNode::Cons { head: (_, Token { id: TokenId::NEWLINE, .. }), tail: _ } } => {
                rest = (*__esc_rest).clone();
                (false, rest.clone(), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: e @ (_, Token { id: TokenId::NEWLINE, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::NEWLINE, .. }), tail: __esc_rest } } => {
                rest = (*__esc_rest).clone();
                (false, metamodelica::cons(e.clone(), rest.clone()), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: e @ (_, Token { id: TokenId::NEWLINE, .. }), tail: __esc_rest } => {
                rest = (*__esc_rest).clone();
                (true, rest.clone(), metamodelica::cons(e.clone(), tmp))
            },
            Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::WHITESPACE, .. }), tail: Deref @ metamodelica::ListNode::Cons { head: e @ (DiffAlgorithm::Diff::Add, _), tail: __esc_rest } } if (lastIsNewline) => {
                rest = (*__esc_rest).clone();
                (false, rest.clone(), metamodelica::cons(e.clone(), metamodelica::cons((Diff::Add.clone(), Token { fileName: literal!("WHITESPACE"), id: TokenId::WHITESPACE.clone(), fileContents: (({
            let mut __acc = String::new();
            for mut i in (1..=depth).into_iter() {
                let __x = literal!(" ");
                __acc.push_str(&__x);
            }
            ArcStr::from(__acc)
        })).clone(), byteOffset: 1, length: depth, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0 }), tmp)))
            },
            Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::WHITESPACE, .. }), tail: __esc_rest @ Deref @ metamodelica::ListNode::Cons { head: (_, Token { id: TokenId::NEWLINE, .. }), tail: _ } } if (lastIsNewline) => {
                rest = (*__esc_rest).clone();
                (true, rest.clone(), tmp)
            },
            Deref @ metamodelica::ListNode::Cons { head: e @ (_, t @ Token { id: TokenId::WHITESPACE, .. }), tail: __esc_rest } if (lastIsNewline) => {
                rest = (*__esc_rest).clone();
                let Token { length: __pa0, .. } = &t;
                depth = metamodelica::Own::own(__pa0);
                (false, rest.clone(), metamodelica::cons(e.clone(), tmp))
            },
            Deref @ metamodelica::ListNode::Cons { head: e, tail: __esc_rest } => {
                rest = (*__esc_rest).clone();
                (false, rest.clone(), metamodelica::cons(e.clone(), tmp))
            },
            _ => return Err("match: no arm matched"),
        } });
    }
    simpleDiff = tmp.reverse();
    addedLineComments = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut e in (simpleDiff.clone()).into_iter().cloned() {
            if !(Diff::Add.clone() == tuple21(e.clone()) && isLineComment(&(tuple22(e.clone())))) {
                continue;
            }
            let __x = tokenContent(tuple22(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    removedLineComments = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut e in (simpleDiff.clone()).into_iter().cloned() {
            if !(Diff::Delete.clone() == tuple21(e.clone()) && isLineComment(&(tuple22(e.clone())))) {
                continue;
            }
            let __x = tokenContent(tuple22(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    addedBlockComments = ({
        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (simpleDiff.clone()).into_iter().cloned() {
            if !(Diff::Add.clone() == tuple21(e.clone()) && isBlockComment(&(tuple22(e.clone())))) {
                continue;
            }
            let __x = blockCommentCanonical(tuple22(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    removedBlockComments = ({
        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (simpleDiff.clone()).into_iter().cloned() {
            if !(Diff::Delete.clone() == tuple21(e.clone()) && isBlockComment(&(tuple22(e.clone())))) {
                continue;
            }
            let __x = blockCommentCanonical(tuple22(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    simpleDiff = ({
        let mut __acc: metamodelica::List<(Diff, Token)> = metamodelica::nil();
        for mut e in (simpleDiff).into_iter().cloned() {
            if !(match e.clone() {
                (
                    DiffAlgorithm::Diff::Add,
                    mut t @ Token {
                        id: TokenId::LINE_COMMENT,
                        ..
                    },
                ) => !(listMember(tokenContent(t.clone())?, removedLineComments.clone())),
                (
                    DiffAlgorithm::Diff::Add,
                    mut t @ Token {
                        id: TokenId::BLOCK_COMMENT,
                        ..
                    },
                ) => !(listMember(blockCommentCanonical(t.clone())?, removedBlockComments.clone())),
                _ => true,
            }) {
                continue;
            }
            let __x = (match e.clone() {
                (
                    DiffAlgorithm::Diff::Delete,
                    mut t @ Token {
                        id: TokenId::LINE_COMMENT,
                        ..
                    },
                ) => {
                    if (listMember(tokenContent(t.clone())?, addedLineComments.clone())) {
                        (Diff::Equal.clone(), t.clone())
                    } else {
                        e.clone()
                    }
                }
                (
                    DiffAlgorithm::Diff::Delete,
                    mut t @ Token {
                        id: TokenId::BLOCK_COMMENT,
                        ..
                    },
                ) => {
                    if (listMember(blockCommentCanonical(t.clone())?, addedBlockComments.clone())) {
                        (Diff::Equal.clone(), t.clone())
                    } else {
                        e.clone()
                    }
                }
                _ => e.clone(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    odiffs = ({
        let mut __acc: metamodelica::List<(Diff, metamodelica::List<Token>)> = metamodelica::nil();
        for mut e in (simpleDiff).into_iter().cloned() {
            let __x = (match e.clone() {
                (mut d, mut t) => (d, list![t.clone()]),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(odiffs)
}

pub fn isBlockComment(mut t: &Token) -> bool {
    let mut b: bool;
    b = (match t.clone() {
        Token {
            id: TokenId::BLOCK_COMMENT,
            ..
        } => true,
        _ => false,
    });
    b
}

pub fn isLineComment(mut t: &Token) -> bool {
    let mut b: bool;
    b = (match t.clone() {
        Token {
            id: TokenId::LINE_COMMENT,
            ..
        } => true,
        _ => false,
    });
    b
}

pub(crate) fn tuple21<
    A: Clone + 'static + metamodelica::gc::MMTrace,
    B: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut t: (A, B),
) -> A {
    let mut a: A;
    (a, _) = t;
    a
}

pub(crate) fn tuple22<
    A: Clone + 'static + metamodelica::gc::MMTrace,
    B: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut t: (A, B),
) -> B {
    let mut b: B;
    (_, b) = t;
    b
}

pub fn blockCommentCanonical(mut t: Token) -> Result<metamodelica::List<ArcStr>> {
    let mut lines: metamodelica::List<ArcStr>;
    lines = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut s in (System::strtok(tokenContent(t)?, literal!("\n"))).into_iter().cloned() {
            let __x = System::trim(s.clone(), literal!(" \u{c}\n\r\t\u{b}"));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(lines)
}

pub(crate) fn deleteWhitespaceFollowedByEqualNonWhitespace(
    mut inRest: metamodelica::List<(DiffAlgorithm::Diff, Token)>,
) -> Result<(bool, metamodelica::List<(DiffAlgorithm::Diff, Token)>)> {
    use openmodelica_util::DiffAlgorithm::Diff;
    let mut b: bool;
    let mut result: metamodelica::List<(Diff, Token)>;
    let mut head: (Diff, Token);
    let mut diff: Diff;
    let mut t: Token;
    let mut id: TokenId;
    let mut rest: metamodelica::List<(Diff, Token)>;
    let mut foundWS: bool = false;
    let mut foundNL: bool = false;
    rest = inRest;
    result = metamodelica::nil();
    while !((rest).is_empty()) {
        let ref __pa3 @ (ref __pa0, ref __pa2 @ Token { id: ref __pa1, .. }) = (rest).head().cloned()?;
        diff = metamodelica::Own::own(__pa0);
        id = metamodelica::Own::own(__pa1);
        t = metamodelica::Own::own(__pa2);
        head = metamodelica::Own::own(__pa3);
        if diff != Diff::Delete.clone() {
            break;
        }
        rest = (rest).rest()?;
        if id == TokenId::WHITESPACE.clone() && !(foundWS) {
            foundWS = true;
            result = metamodelica::cons((Diff::Equal.clone(), t.clone()), result);
        } else if id == TokenId::NEWLINE.clone() {
            foundNL = true;
            break;
        } else {
            result = metamodelica::cons(head, result);
        }
    }
    if !(foundWS) || foundNL {
        b = false;
        result = metamodelica::nil();
        return Ok((b, result));
    }
    let () = (::match_deref::match_deref! { match &(&*rest) {
        Deref @ metamodelica::ListNode::Cons { head: (DiffAlgorithm::Diff::Equal, __esc_t), tail: _ } => {
            t = (*__esc_t).clone();
            ()
        },
        _ => {
            b = false;
            result = metamodelica::nil();
            return Ok((b, result));
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b = true;
    for mut i in &*result {
        rest = metamodelica::cons(i.clone(), rest);
    }
    result = rest;
    Ok((b, result))
}

pub fn reportErrors(mut tokens: &metamodelica::List<Token>) -> Result<()> {
    let mut i: i32 = 0;
    let mut content: ArcStr;
    for mut t in &**tokens {
        i = i + 1;
        if i > 10 {
            Error::addMessage(Error::SCANNER_ERROR_LIMIT.clone(), metamodelica::nil())?;
        }
        content = tokenContent(t.clone())?;
        Error::addSourceMessage(
            &(Error::SCANNER_ERROR.clone()),
            list![StringUtil::convertCharNonAsciiToHex(content)?],
            &(tokenSourceInfo(t.clone())),
        )?;
    }
    if !((tokens).is_empty()) {
        return Err("fail");
    }
    Ok(())
}
