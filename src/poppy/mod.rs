use rhai::plugin::*;

#[export_module]
pub mod poppy {
    pub const MY_NUMBER: i64 = 42;

    #[cfg(feature = "greetings")]
    pub fn greet(name: &str) -> String {
        format!("hello, {}", name)
    }
    
    pub fn get_num() -> i64 {
        mystic_number()
    }
    
    #[rhai_fn(global)]
    pub fn increment(num: &mut i64) {
        *num += 1;
    }
    
    fn mystic_number() -> i64 {
        42
    }
    
    pub mod my_sub_module {
        pub fn get_info() -> String {
            "hello".to_string()
        }
    }
    
    #[cfg(feature = "advanced_functions")]
    pub mod advanced {
        pub fn advanced_calc(input: i64) -> i64 {
            input * 2
        }
    }
    
}

