use alloc::boxed::Box;
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}
