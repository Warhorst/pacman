pub mod pacmap;
pub mod board;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum FieldType {
    Free,
    Wall,
    LeftTunnel,
    RightTunnel
}
