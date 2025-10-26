// Each character set is defined as an array mapping from internal BCD to external representation

use std::sync::LazyLock;

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
    array[0b010000] = '¢';
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
