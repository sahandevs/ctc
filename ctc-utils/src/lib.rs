#[macro_export]
macro_rules! import_conf {
    ($name: literal) => {
        mod ctc_inner_conf {
            #![allow(non_upper_case_globals)]
            #![allow(dead_code)]
            include!(concat!(env!("OUT_DIR"), "/", $name, ".rs"));
        }

        pub use ctc_inner_conf::conf::*;
    };
}
