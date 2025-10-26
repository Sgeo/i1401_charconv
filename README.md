# i1401-charconv

The IBM 1401 uses characters that are not present in ASCII. Different emulators have different ways of handling such special characters. **i1401-charconv** converts between these encodings.

## Usage

`i1401-charconv --from encoding --to encoding < source.txt > destination.txt`

## Supported encodings

* `unicode-card`, an encoding that displays each character as shown in the [IBM 1401 reference from April 1962](https://bitsavers.org/pdf/ibm/1401/A24-1403-5_1401_Reference_Apr62.pdf). No known emulator uses this, but I consider this to be the most "correct" conversion.
* `vdc`, the encoding used by [Michael Schuetz's Virtual Data Center](https://rolffson.de/), a 3D virtual datacenter with an IBM 1401.
* `simh-new` and `simh-old`, the encodings supported by [SIMH](https://simh.trailing-edge.com/). Note that many files on https://ibm-1401.info appear to use `simh-old`
* `sim1401-1047`, the encoding used by IBM's 1401 emulator for the System/360. As this is inherently BCD <-> EBCDIC, `i1401-charconv` then converts that EBCDIC assuming that it's EBCDIC-1047. For use with Hercules, it is suggested to run `codepage 819/1047` at the console.

## Todo

* `dos` encoding: There exists a DOS based IBM 1401 emulator, by Joe Newcomer and Jay Jaeger. It uses a CP-437 encoding, rather than ASCII. This project will probably convert to a Unicode equivalent, so using cards with that emulator will require a second step to convert UTF-8 to CP-437.
* `unicode-print-a` and `unicode-print-h` encodings: The IBM 1401 doesn't necessarily print the same characters it receives.
* simh fixes: SIMH accepts lowercase characters, which this code currently doesn't. Should probably copy from SIMH code instead of documentation.