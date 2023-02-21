const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Copy, Clone)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

struct GPU {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}

impl GPU {
    fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    fn write_vram(&mut self, address: usize, value: u8) {
        self.vram[address] = value;

        // The rest of the function is for tileset storage,
        // so return early if address isn't in that range
        if address >= 0x1800 { return; }

        // Unsetting the lsb makes sure the address is even,
        // that ensures we get the correct 2 bytes
        let normalized_address = address & 0xFFFE;

        let byte1 = self.vram[normalized_address];
        let byte2 = self.vram[normalized_address + 1];

        // Calculate the indices needed to cache in tile_set
        // based on the address
        let tile_index = address / 16;
        let row_index = (address % 16) / 2;

        for pixel_index in 0..8 {

            // yeah the gameboy encoded pixels with 2 bits but the first
            // bit was in the first byte and the second bit on the second byte
            // so we have to do this

            let lsb = byte1 & (1 << (7 - pixel_index));
            let msb = byte2 & (1 << (7 - pixel_index));

            self.tile_set[tile_index][row_index][pixel_index] = match (lsb != 0, msb != 0) {
                (false, false) => TilePixelValue::Zero,
                (true, false) => TilePixelValue::One,
                (false, true) => TilePixelValue::Two,
                (true, true) => TilePixelValue::Three,
            };
        }
    }
}
