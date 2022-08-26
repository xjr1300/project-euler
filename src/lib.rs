#[macro_export]
macro_rules! measure {
    ( $x:expr) => {{
        let start = Instant::now();
        let result = $x;
        let end = start.elapsed();
        println!("elapsed: {}.{:03}s", end.as_secs(), end.subsec_millis(),);
        result
    }};
}
