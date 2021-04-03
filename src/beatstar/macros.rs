#[macro_export]
macro_rules! vec_extern {
    ($clazz:ident, $vec:ident, $r:ty, $func_get_name:ident, $func_len_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $func_get_name(selfI: &$clazz, index: usize) -> *const $r {
            unsafe {
                return match (*selfI.$vec).get(index) {
                    Some(e) => e,
                    None => ptr::null(),
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn $func_len_name(selfI: &$clazz) -> usize {
            unsafe {
                return (*selfI.$vec).len();
            }
        }
    };
}

#[macro_export]
macro_rules! map_extern {
    ($clazz:ident, $hashmap:ident, $k:ty, $r:ty, $func_get_name:ident, $func_len_name:ident, $func_get_key_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $func_get_name(selfI: &$clazz, index: &$k) -> *const $r {
            unsafe {
                return match (*selfI.$hashmap).get(index) {
                    Some(e) => e,
                    None => ptr::null(),
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn $func_get_key_name(selfI: &$clazz, index: usize) -> *const $k {
            unsafe {
                let keys: Vec<&$k> = (*selfI.$hashmap).keys().collect();
                return match keys.get(index) {
                    Some(e) => *e,
                    None => ptr::null(),
                };
            }
        }

        #[no_mangle]
        pub extern "C" fn $func_len_name(selfI: &$clazz) -> usize {
            unsafe {
                return (*selfI.$hashmap).len();
            }
        }
    };
}
