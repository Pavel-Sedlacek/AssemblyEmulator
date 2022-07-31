pub trait Memory {
    fn fetch_byte(&self, index: u8) -> u8;
    fn fetch_word(&self, index: u8) -> u16;
    fn write_byte(&mut self, index: u8, byte: u8);
    fn write_word(&mut self, index: u8, word: u16);

    fn reset(&mut self);
}