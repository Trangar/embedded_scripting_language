use crate::runtime::Runtime;
use crate::traits::State;
use byteorder::{ByteOrder, NetworkEndian};

pub enum Instructions {
    CallMethod {
        // 0x01
        result_variable: VariableRef,
        method: MethodRef,
        args: [VariableRef; 3],
    },
    CompareEquals {
        // 0x02
        left: VariableRef,
        right: VariableRef,
    },
}

impl Instructions {
    pub fn get(buffer: &[u8]) -> Self {
        unimplemented!()
    }

    pub fn write(buffer: &mut [u8]) {}

    pub fn size(&self) -> usize {
        match self {
            Instructions::CallMethod {
                result_variable,
                method,
                args,
            } => {
                let mut result = 1 + result_variable.size() + method.size();
                for idx in 0..method.arg_len() {
                    result += args[idx].size()
                }
                result
            }
            Instructions::CompareEquals { left, right } => 1 + left.size() + right.size(),
        }
    }

    pub fn execute<S: State>(&self, runtime: &mut Runtime<S>) {}
}

pub enum VariableRef {
    None,
    Idx(u8),
    Num(i32),
    Float(f32),
}

impl VariableRef {
    const fn size(&self) -> usize {
        match self {
            VariableRef::None => 1,
            VariableRef::Idx(_) => 2,
            VariableRef::Num(_) => 5,
            VariableRef::Float(_) => 5,
        }
    }

    fn write(&self, buffer: &mut [u8]) {
        match self {
            VariableRef::None => buffer[0] = 0x00,
            VariableRef::Idx(idx) => {
                buffer[0] = 0x01;
                buffer[1] = *idx;
            }
            VariableRef::Num(num) => {
                buffer[0] = 0x02;
                NetworkEndian::write_i32(&mut buffer[1..], *num);
            }
            VariableRef::Float(num) => {
                buffer[0] = 0x03;
                NetworkEndian::write_f32(&mut buffer[1..], *num);
            }
        }
    }
}

#[repr(C)]
pub enum MethodRef {
    /// get_bit_buffer(buffer_size) -> ref
    GetBitBuffer = 0x01,

    /// fill_random_bit_buffer(buffer_ref)
    FillRandomBitBuffer = 0x02,

    /// set_bit_buffer_index(buffer_ref, index_ref)
    SetBitBufferIndex = 0x03,

    /// clear_bit_buffer_index(buffer_ref, index_ref)
    ClearBitBufferIndex = 0x04,

    /// get_bit_buffer_index(buffer_ref, index_ref) -> value_ref
    GetBitBufferIndex = 0x05,

    /// xy_to_buffer_index(x, y) -> index_ref
    XYToBufferIndex = 0x06,

    /// wait_for_clock_high()
    WaitForClockHigh = 0x07,

    /// wait_for_clock_low()
    WaitForClockLow = 0x08,

    /// set_frame_buffer(buffer)
    SetFrameBuffer = 0x09,
}

impl MethodRef {
    pub const fn size(&self) -> usize {
        1
    }

    pub const fn arg_len(&self) -> usize {
        match self {
            MethodRef::GetBitBuffer => 1,
            MethodRef::FillRandomBitBuffer => 1,
            MethodRef::SetBitBufferIndex => 2,
            MethodRef::ClearBitBufferIndex => 2,
            MethodRef::GetBitBufferIndex => 2,
            MethodRef::XYToBufferIndex => 2,
            MethodRef::WaitForClockHigh => 0,
            MethodRef::WaitForClockLow => 0,
            MethodRef::SetFrameBuffer => 1,
        }
    }

    pub const fn has_result(&self) -> bool {
        match self {
            MethodRef::GetBitBuffer => true,
            MethodRef::FillRandomBitBuffer => false,
            MethodRef::SetBitBufferIndex => false,
            MethodRef::ClearBitBufferIndex => false,
            MethodRef::GetBitBufferIndex => true,
            MethodRef::XYToBufferIndex => true,
            MethodRef::WaitForClockHigh => false,
            MethodRef::WaitForClockLow => false,
            MethodRef::SetFrameBuffer => false,
        }
    }
}
