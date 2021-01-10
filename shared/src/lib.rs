#![allow(dead_code, unused_variables)]
#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate std;

#[cfg(feature = "compiler")]
mod compiler;

mod evaluator;
mod instructions;
mod runtime;
mod traits;

#[test]
#[cfg(feature = "compiler")]
fn test_simple_script() {
    let script = r#"
buffer = get_bit_buffer(10*10)
next_buffer = get_bit_buffer(10*10)

fill_random_bit_buffer(buffer)

loop:
    wait_for_clock_high()
    for x in 0,10:
        for y in 0,10:
            neighbour_count = 0
            if get_bit_buffer_index(xy_to_buffer_index(x - 1, y - 1)):
                neighbour_count += 1
            if get_bit_buffer_index(xy_to_buffer_index(x, y - 1)):
                neighbour_count += 1
            if get_bit_buffer_index(xy_to_buffer_index(x + 1, y - 1)):
                neighbour_count += 1
            if get_bit_buffer_index(xy_to_buffer_index(x - 1, y)):
                neighbour_count += 1
            if get_bit_buffer_index(xy_to_buffer_index(x + 1, y)):
                neighbour_count += 1
            if get_bit_buffer_index(xy_to_buffer_index(x - 1, y + 1)):
                neighbour_count += 1
            if get_bit_buffer_index(xy_to_buffer_index(x, y + 1)):
                neighbour_count += 1
            if get_bit_buffer_index(xy_to_buffer_index(x + 1, y + 1)):
                neighbour_count += 1

            idx = xy_to_buffer_index(x, y)
            is_alive = get_bit_buffer_index(buffer, idx)
            if not is_alive:
                if neighbour_count == 3:
                    set_bit_buffer_index(next_buffer, idx)
            if is_alive:
                if neighbour_count < 2:
                    clear_bit_buffer_index(next_buffer, idx)
                if neighbour_count > 3:
                    clear_bit_buffer_index(next_buffer, idx)

    buffer = next_buffer
    set_frame_buffer(buffer)
"#;
    let mut bytecode = [0u8; 10 * 1024];
    let len = compiler::compile(script, &mut bytecode).unwrap();
    let bytecode = &mut bytecode[..len];

    let mut runtime = runtime::Runtime::new(bytecode, test_state::TestState::default());

    while runtime.state.screens.is_empty() {
        runtime.step();
    }
}

#[cfg(test)]
mod test_state {
    #[derive(Default)]
    pub struct TestState {
        pub screens: Vec<u128>,
        pub wait_clock_high_count: usize,
        pub wait_clock_low_count: usize,
    }

    impl crate::traits::State for TestState {
        fn fill_random_bit_buffer(&mut self, buffer: &mut u128) {
            *buffer = 0;
            *buffer |= 1 << 2;
            *buffer |= 1 << 12;
            *buffer |= 1 << 22;
        }
        fn draw_screen(&mut self, screen: u128) {
            self.screens.push(screen);
        }
        fn wait_for_clock_high(&mut self) {
            self.wait_clock_high_count += 1;
        }
        fn wait_for_clock_low(&mut self) {
            self.wait_clock_low_count += 1;
        }
    }
}
