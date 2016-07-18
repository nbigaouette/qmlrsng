// While QML is weakly typed, libqmlbind assumes with double precision (f64) in
// its interface. Crreate the ToDoublePrecision trait to handle the conversion.


pub trait ToDoublePrecision {
    fn to_f64(&self) -> f64;
}

impl ToDoublePrecision for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
}

impl ToDoublePrecision for f32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToDoublePrecision for i32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToDoublePrecision for i64 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToDoublePrecision for u32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToDoublePrecision for u64 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
