fn main() {
    // Byte i of a[0] is 0x0i, byte i of a[1] is 0x1i, ... (little-endian)
    let arr: [u128; 4] = [
        0x0F0E_0D0C_0B0A_0908_0706_0504_0302_0100,
        0x1F1E_1D1C_1B1A_1918_1716_1514_1312_1110,
        0x2F2E_2D2C_2B2A_2928_2726_2524_2322_2120,
        0x3F3E_3D3C_3B3A_3938_3736_3534_3332_3130,
    ];

    // A reference can't do arithmetic, so use a raw pointer.
    let p: *const u128 = &arr[0];

    unsafe {
        let next_elem = p.add(1); // +1 element  = +size_of::<u128>() bytes
        let next_byte = p.byte_add(1); // +1 byte

        println!("size_of::<u128>() = {}", std::mem::size_of::<u128>());
        println!("p                 = {:p}", p);
        println!("p.add(1)          = {:p}  (+{} bytes)", next_elem, next_elem as usize - p as usize);
        println!("p.byte_add(1)     = {:p}  (+{} bytes)", next_byte, next_byte as usize - p as usize);
        println!();

        // 1) Step by one element: lands on a[1]
        println!("*p.add(1)             = {:#034x}   <- a[1]", *next_elem);

        // 2) Step by one byte: still inside a[0]. Read it as a single byte.
        let b = *(next_byte as *const u8);
        println!("byte at p + 1         = {:#04x}   <- a[0] byte 1", b);

        // 3) Same thing, but cast to a byte pointer first, then add(1)
        let bytes = p as *const u8;
        println!("*(bytes.add(1))       = {:#04x}   <- a[0] byte 1", *bytes.add(1));
        println!("*(bytes.add(16))      = {:#04x}   <- a[1] byte 0", *bytes.add(16));

        // 4) Reading a whole u128 from p + 1 byte is a misaligned read.
        //    read_unaligned is allowed; it gives bytes 1..=16 of memory.
        let shifted = next_byte.read_unaligned();
        println!("u128 read at p + 1    = {:#034x}   <- a[0] bytes 1..15 + a[1] byte 0", shifted);
    }
}
