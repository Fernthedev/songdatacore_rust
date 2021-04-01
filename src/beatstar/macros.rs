
#[macro_export]
macro_rules! vec_extern {
    ($vec:ident, $r:ty, $func_get_name:ident, $func_len_name:ident) => {
        #[no_mangle]
        pub extern fn $func_get_name(&self, index: usize) -> *const $r {
            return match self.$vec.get(index) {
                Some(e) => e,
                None => ptr::null()
            }
        }

        #[no_mangle]
        pub extern fn $func_len_name(&self) -> usize {
            return self.$vec.len();
        }
    };
}

#[macro_export]
macro_rules! map_extern {
    ($hashmap:ident, $k:ty, $r:ty, $func_get_name:ident, $func_len_name:ident, $func_get_key_name:ident) => {
        #[no_mangle]
        pub extern fn $func_get_name(&self, index: &$k) -> *const $r {
            return match self.$hashmap.get(index) {
                Some(e) => e,
                None => ptr::null()
            }
        }

        #[no_mangle]
        pub extern fn $func_get_key_name(&self, index: usize) -> *const $k {
            let keys: Vec<&$k> = self.$hashmap.keys().collect();
            return match keys.get(index) {
                Some(e) => *e,
                None => ptr::null()
            };
        }

        #[no_mangle]
        pub extern fn $func_len_name(&self) -> usize {
            return self.$hashmap.len();
        }
    };
}
