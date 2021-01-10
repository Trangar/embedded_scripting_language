pub trait State {
    fn fill_random_bit_buffer(&mut self, bit_buffer: &mut u128);
    fn draw_screen(&mut self, bit_buffer: u128);
    fn wait_for_clock_high(&mut self);
    fn wait_for_clock_low(&mut self);
}
