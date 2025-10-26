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