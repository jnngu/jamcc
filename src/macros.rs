// Non-debug version
#[cfg(not(feature = "debug"))]
macro_rules! debug_print {
    ($( $args:expr ),*) => {()}
}

#[cfg(feature = "debug")]
macro_rules! debug_print {
    ($( $args:expr ),*) => { println!( $( $args ),* ) }
}

