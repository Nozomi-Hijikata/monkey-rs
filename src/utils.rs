#[macro_export]
macro_rules! box_it {
    ($e: expr) => {
        Box::new($e)
    };
}

#[macro_export]
macro_rules! downcast_ref {
    ($e:expr, $t:ty) => {
        $e.as_any().downcast_ref::<$t>()
    };
}
