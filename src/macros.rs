#[macro_export]
macro_rules! print_end {
    () => {
        println!("{:-<1$}", "", 40);
    };
}