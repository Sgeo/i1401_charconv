// Each character set is defined as an array mapping from internal BCD to external representation

use std::{collections::HashMap, sync::LazyLock};

const UNICODE_CARD_FROM_BCD: LazyLock<[char; 64]> = LazyLock::new(|| {
    // Taken from https://bitsavers.org/pdf/ibm/1401/A24-1403-5_1401_Reference_Apr62.pdf page 170
    // Using the defined chain described there
    let mut array = ['�'; 64];
    //    0bBA8421. C is ignored as it's a parity bit
    array[0b000000] = ' ';
    array[0b111011] = '.';
    array[0b111100] = '\u{2311}'; // Square lozenge (⌑), not present in a lot of fonts
    array[0b111101] = '(';
    array[0b111110] = '<';
    array[0b111111] = '\u{2BD2}'; // Group mark (⯒), not present in a lot of fonts
    array[0b110000] = '&'; 
    array[0b101011] = '$';
    array[0b101100] = '*';
    array[0b101101] = ')';
    array[0b101110] = ';';
    array[0b101111] = 'Δ';
    array[0b100000] = '-';
    array[0b010001] = '/';
    array[0b011011] = ',';
    array[0b011100] = '%';
    array[0b011101] = '='; // called word separator
    array[0b011110] = '\'';
    array[0b011111] = '"';
    array[0b010000] = '¢'; // Not readable from card??
    array[0b001011] = '#';
    array[0b001100] = '@';
    array[0b001101] = ':';
    array[0b001110] = '>';
    array[0b001111] = '√'; // Tape mark (square root?)
    array[0b111010] = '?'; // Also called plus 0. Printed differently
    array[0b110001] = 'A';
    array[0b110010] = 'B';
    array[0b110011] = 'C';
    array[0b110100] = 'D';
    array[0b110101] = 'E';
    array[0b110110] = 'F';
    array[0b110111] = 'G';
    array[0b111000] = 'H';
    array[0b111001] = 'I';
    array[0b101010] = '!'; // Also called minus 0
    array[0b100001] = 'J';
    array[0b100010] = 'K';
    array[0b100011] = 'L';
    array[0b100100] = 'M';
    array[0b100101] = 'N';
    array[0b100110] = 'O';
    array[0b100111] = 'P';
    array[0b101000] = 'Q';
    array[0b101001] = 'R';
    array[0b011010] = '\u{29E7}'; // Thermodynamic (⧧), not sure if appropriate but it looks like record mark
    array[0b010010] = 'S';
    array[0b010011] = 'T';
    array[0b010100] = 'U';
    array[0b010101] = 'V';
    array[0b010110] = 'W';
    array[0b010111] = 'X';
    array[0b011000] = 'Y';
    array[0b011001] = 'Z';
    array[0b001010] = '0';
    array[0b000001] = '1';
    array[0b000010] = '2';
    array[0b000011] = '3';
    array[0b000100] = '4';
    array[0b000101] = '5';
    array[0b000110] = '6';
    array[0b000111] = '7';
    array[0b001000] = '8';
    array[0b001001] = '9';

    array
});

const BCD_FROM_UNICODE_CARD: LazyLock<HashMap<char, u8>> = LazyLock::new(|| {
    let mut hashmap: HashMap<char, u8> = HashMap::with_capacity(64);
    for (bcd, char) in UNICODE_CARD_FROM_BCD.iter().enumerate() {
        hashmap.insert(*char, bcd as u8);
    }
    hashmap.insert(' ', 0); // Not relevant for unicode-card, but relevant for similar

    hashmap
});

const UNICODE_PRINT_A_FROM_BCD: LazyLock<[char; 64]> = LazyLock::new(|| {
    let mut array = *UNICODE_CARD_FROM_BCD;

    array[0b111101] = ' ';
    array[0b111110] = ' ';
    array[0b111111] = ' ';
    array[0b101101] = ' ';
    array[0b101110] = ' ';
    array[0b101111] = ' ';
    array[0b011101] = ' ';
    array[0b011110] = ' ';
    array[0b011111] = ' ';
    array[0b010000] = '\u{29E7}'; // Thermodynamic (⧧), not sure if appropriate but it looks like record mark
    array[0b001101] = ' ';
    array[0b001110] = ' ';
    array[0b001111] = ' '; // Tape mark (square root?)
    array[0b111010] = '&';
    array[0b101010] = '-';


    array
});

const VDC_FROM_BCD: LazyLock<[char; 64]> = LazyLock::new(|| {
    let mut array = *UNICODE_CARD_FROM_BCD;

    array[0b010000] = 'c';
    array[0b001111] = 't';
    array[0b011010] = 'r';
    array[0b011101] = 'v';
    array[0b011111] = 's';
    array[0b100000] = '!';
    array[0b101111] = '^';
    array[0b101010] = '!'; // The VDC <-> SIMH page wrongly lists this as B. Other documentation including VDC says B82
    array[0b101111] = '^';
    array[0b111010] = '?';
    array[0b111100] = 'q'; 
    array[0b111111] = 'g';
    array[30] = '\\'; // Weird: Both VDC and SIMH have this as backslash, the reference calls it apostrophe
    // Using the chart on https://rolffson.de/m_doc.html more directly is a good idea
    // Only doing it partially, I think the rest of the chart matches, but not certain
    array[62] = '<';
    array[45] = ']';
    array[46] = ';';
    array[14] = '>';

    array

});

const BCD_FROM_VDC: LazyLock<HashMap<char, u8>> = LazyLock::new(|| {
    let mut hashmap: HashMap<char, u8> = HashMap::with_capacity(64);
    for (bcd, char) in VDC_FROM_BCD.iter().enumerate() {
        hashmap.insert(*char, bcd as u8);
    }
    hashmap.insert(' ', 0); // Not relevant for unicode-card, but relevant for similar

    hashmap
});

const SIMH_NEW_FROM_BCD: LazyLock<[char; 64]> = LazyLock::new(|| {
    let mut array = ['�'; 64];

    // Copy/pasted from SIMH documentation
    array[0o00] = ' ';
    array[0o01] = '1';
    array[0o02] = '2';
    array[0o03] = '3';
    array[0o04] = '4';
    array[0o05] = '5';
    array[0o06] = '6';
    array[0o07] = '7';
    array[0o10] = '8';
    array[0o11] = '9';
    array[0o12] = '0';
    array[0o13] = '#'; // Or =
    array[0o14] = '@'; // Or '
    array[0o15] = ':';
    array[0o16] = '>';
    array[0o17] = '{';
    array[0o20] = '^';
    array[0o21] = '/';
    array[0o22] = 'S';
    array[0o23] = 'T';
    array[0o24] = 'U';
    array[0o25] = 'V';
    array[0o26] = 'W';
    array[0o27] = 'X';
    array[0o30] = 'Y';
    array[0o31] = 'Z';
    array[0o32] = '|';
    array[0o33] = ',';
    array[0o34] = '%'; // Or (
    array[0o35] = '~';
    array[0o36] = '\\'; // This is weird. Everything else has it as '
    array[0o37] = '"';
    array[0o40] = '-';
    array[0o41] = 'J';
    array[0o42] = 'K';
    array[0o43] = 'L';
    array[0o44] = 'M';
    array[0o45] = 'N';
    array[0o46] = 'O';
    array[0o47] = 'P';
    array[0o50] = 'Q';
    array[0o51] = 'R';
    array[0o52] = '!';
    array[0o53] = '$';
    array[0o54] = '*';
    array[0o55] = ']';
    array[0o56] = ';';
    array[0o57] = '_';
    array[0o60] = '&';
    array[0o61] = 'A';
    array[0o62] = 'B';
    array[0o63] = 'C';
    array[0o64] = 'D';
    array[0o65] = 'E';
    array[0o66] = 'F';
    array[0o67] = 'G';
    array[0o70] = 'H';
    array[0o71] = 'I';
    array[0o72] = '?';
    array[0o73] = '.';
    array[0o74] = ')';
    array[0o75] = '[';
    array[0o76] = '<';
    array[0o77] = '}';

    array
});


const BCD_FROM_SIMH_NEW: LazyLock<HashMap<char, u8>> = LazyLock::new(|| {
    let mut hashmap: HashMap<char, u8> = HashMap::with_capacity(64);
    for (bcd, char) in SIMH_NEW_FROM_BCD.iter().enumerate() {
        hashmap.insert(*char, bcd as u8);
    }
    hashmap.insert(' ', 0); // Not relevant for unicode-card, but relevant for similar
    hashmap.insert('=', 0o13);
    hashmap.insert('\'', 0o14);
    hashmap.insert('(', 0o34);
    hashmap.insert('+', 0o60); // Documented in the simh-old area, not in the main charset.

    hashmap
});

const SIMH_OLD_FROM_BCD: LazyLock<[char; 64]> = LazyLock::new(|| {
    let mut array = *SIMH_NEW_FROM_BCD;
    array[0o17] = '(';
    array[0o32] = '\'';
    array[0o35] = '=';
    array[0o37] = '+';
    array[0o60] = '&';
    array[0o77] = '"';

    array

});

const BCD_FROM_SIMH_OLD: LazyLock<HashMap<char, u8>> = LazyLock::new(|| {
    let mut hashmap: HashMap<char, u8> = HashMap::with_capacity(64);
    for (bcd, char) in SIMH_OLD_FROM_BCD.iter().enumerate() {
        hashmap.insert(*char, bcd as u8);
    }
    hashmap.insert(' ', 0); // Not relevant for unicode-card, but relevant for similar

    hashmap
});

const SIM1401_FROM_BCD: LazyLock<[char; 64]> = LazyLock::new(|| {
    // This encoding is a composition of SIM1401's BCD -> EBCDIC encoding
    // and Hercules's EBCDIC-1047 -> ISO-8859-1 encoding
    let mut array: [char; 64] = ['�'; 64];
    let TRIE: [u8; 64] = [0x40, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xF0, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F, 0xF0, 0x61, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xE0, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x60, 0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xD0, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F, 0x50, 0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xC0, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F];
    let RACINGMARS_ASCII_FROM_1047: [u8; 256] = [
                /* 0x */ 0x00, 0x01, 0x02, 0x03, 0x9C, 0x09, 0x86, 0x7F, 0x97, 0x8D, 0x8E, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                /* 1x */ 0x10, 0x11, 0x12, 0x13, 0x9D, 0x85, 0x08, 0x87, 0x18, 0x19, 0x92, 0x8F, 0x1C, 0x1D, 0x1E, 0x1F,
                /* 2x */ 0x80, 0x81, 0x82, 0x83, 0x84, 0x0A, 0x17, 0x1B, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x05, 0x06, 0x07,
                /* 3x */ 0x90, 0x91, 0x16, 0x93, 0x94, 0x95, 0x96, 0x04, 0x98, 0x99, 0x9A, 0x9B, 0x14, 0x15, 0x9E, 0x1A,
                /* 4x */ 0x20, 0xA0, 0xE2, 0xE4, 0xE0, 0xE1, 0xE3, 0xE5, 0xE7, 0xF1, 0xA2, 0x2E, 0x3C, 0x28, 0x2B, 0x7C,
                /* 5x */ 0x26, 0xE9, 0xEA, 0xEB, 0xE8, 0xED, 0xEE, 0xEF, 0xEC, 0xDF, 0x21, 0x24, 0x2A, 0x29, 0x3B, 0x5E,
                /* 6x */ 0x2D, 0x2F, 0xC2, 0xC4, 0xC0, 0xC1, 0xC3, 0xC5, 0xC7, 0xD1, 0xA6, 0x2C, 0x25, 0x5F, 0x3E, 0x3F,
                /* 7x */ 0xF8, 0xC9, 0xCA, 0xCB, 0xC8, 0xCD, 0xCE, 0xCF, 0xCC, 0x60, 0x3A, 0x23, 0x40, 0x27, 0x3D, 0x22,
                /* 8x */ 0xD8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xAB, 0xBB, 0xF0, 0xFD, 0xFE, 0xB1,
                /* 9x */ 0xB0, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0xAA, 0xBA, 0xE6, 0xB8, 0xC6, 0xA4,
                /* Ax */ 0xB5, 0x7E, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0xA1, 0xBF, 0xD0, 0x5B, 0xDE, 0xAE,
                /* Bx */ 0xAC, 0xA3, 0xA5, 0xB7, 0xA9, 0xA7, 0xB6, 0xBC, 0xBD, 0xBE, 0xDD, 0xA8, 0xAF, 0x5D, 0xB4, 0xD7,
                /* Cx */ 0x7B, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0xAD, 0xF4, 0xF6, 0xF2, 0xF3, 0xF5,
                /* Dx */ 0x7D, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52, 0xB9, 0xFB, 0xFC, 0xF9, 0xFA, 0xFF,
                /* Ex */ 0x5C, 0xF7, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0xB2, 0xD4, 0xD6, 0xD2, 0xD3, 0xD5,
                /* Fx */ 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0xB3, 0xDB, 0xDC, 0xD9, 0xDA, 0x9F,
            ];
    for i in 0..64u8 {
        let code = RACINGMARS_ASCII_FROM_1047[TRIE[i as usize] as usize];
        // Hercules doesn't attempt to emit Unicode, but all codepoints in question are ASCII anyway
        let char = char::from_u32(code as u32).expect("Invalid character?");
        array[i as usize] = char;

    }

    array

});