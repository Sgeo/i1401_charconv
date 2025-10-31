function roundtrip_checker(encoding) {
    for(let i=0; i < 64; i++) {
        let char = encoding.encode[i];
        let newbcd = encoding.decode[char];
        if(newbcd != i) {
            console.log(`Failed round-trip! 0o${i.toString(8)} -> '${char}' -> 0o${newbcd?.toString(8)}`);
        }
    }
}

function ucm_charmap_to_mapping(charmap) {
    let result = {
        "encode": {},
        "decode": {}
    }
    let lines = charmap.split("\n");
    for(let line of lines) {
        const PARSER = /^\<U([0-9A-F]{4})\> \\x([0-9A-F]{2}) \|0$/g;
        for(let match of line.matchAll(PARSER)) {
            let unichar = String.fromCodePoint(parseInt(match[1], 16));
            let byte = parseInt(match[2], 16);
            result.encode[unichar] = byte;
            result.decode[byte] = unichar;
        }
    }
    return result;

}

for(let i = 0; i < 256; i++) { char437 = cp437_map.decode[i]; let bcd = newcomer_ascii_bcd[i]; if(bcd != -1) { newcomer_decode[char437] = bcd; } }