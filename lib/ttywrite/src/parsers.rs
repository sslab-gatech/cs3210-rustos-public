use serial::core::{CharSize, BaudRate, StopBits, FlowControl};

pub fn parse_width(s: &str) -> Result<CharSize, &str> {
    match s {
        "5" => Ok(CharSize::Bits5),
        "6" => Ok(CharSize::Bits6),
        "7" => Ok(CharSize::Bits7),
        "8" => Ok(CharSize::Bits8),
        _ => Err("value must be >= 5 and <= 8")
    }
}

pub fn parse_stop_bits(s: &str) -> Result<StopBits, &str> {
    match s {
        "1" => Ok(StopBits::Stop1),
        "2" => Ok(StopBits::Stop2),
        _ => Err("value must '1' or '2'")
    }
}

pub fn parse_flow_control(s: &str) -> Result<FlowControl, &str> {
    match s {
        "none" => Ok(FlowControl::FlowNone),
        "software" => Ok(FlowControl::FlowSoftware),
        "hardware" => Ok(FlowControl::FlowHardware),
        _ => Err("value must be 'none', 'software' (xon/xoff), or 'hardware' (rts/cts)")
    }
}

pub fn parse_baud_rate(s: &str) -> Result<BaudRate, ::std::num::ParseIntError> {
    Ok(BaudRate::from_speed(s.parse()?))
}
