function roundtrip_checker(encoding) {
    for(let i=0; i < 64; i++) {
        let char = encoding.encode[i];
        let newbcd = encoding.decode[char];
        if(newbcd != i) {
            console.log(`Failed round-trip! 0o${i.toString(8)} -> '${char}' -> 0o${newbcd.toString(8)}`);
        }
    }
}