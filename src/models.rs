#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub is_alive: bool,
    pub next_tick_alive: bool,
}
