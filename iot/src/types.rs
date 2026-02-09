use heapless::Vec;

struct Data {
    position: u16,

    avg_speed: u8,
    vehicles: Vec<(u16, u8), 8>,
}
