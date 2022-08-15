#[macro_export]
macro_rules! vec_extern {
    ($clazz:ident, $vec:ident, $r:ty, $func_get_ptr_name:ident, $func_get_name:ident, $func_len_name:ident) => {
        /// Gets the item in the vector from index
        #[no_mangle]
        pub extern "C" fn $func_get_ptr_name(self_i: &$clazz) -> *const $r {
            return self_i.$vec.as_ptr();
        }


        /// Gets the item in the vector from index
        #[no_mangle]
        pub extern "C" fn $func_get_name(self_i: &$clazz, index: usize) -> *const $r {
            return match (self_i.$vec).get(index) {
                Some(e) => e,
                None => ptr::null(),
            }
        }

        /// Gets the length of the vector
        #[no_mangle]
        pub extern "C" fn $func_len_name(self_i: &$clazz) -> usize {
            return (*self_i.$vec).len();
        }
    };
}

#[macro_export]
macro_rules! map_extern {
    ($clazz:ident, $hashmap:ident, $k:ty, $r:ty, $func_get_name:ident, $func_len_name:ident, $func_get_key_name:ident) => {

        ///
        /// Get the value in the hashmap from the key
        ///
        #[no_mangle]
        pub extern "C" fn $func_get_name(self_i: &$clazz, index: &$k) -> *const $r {
            return match (self_i.$hashmap).get(&index) {
                Some(e) => e,
                None => ptr::null(),
            }
        }

        ///
        /// Get the key in the hashmap from the index in it's set.
        ///
        #[no_mangle]
        pub extern "C" fn $func_get_key_name(self_i: &$clazz, index: usize) -> *const $k {

            let keys: Vec<&$k> = (self_i.$hashmap).keys().collect();
            return match keys.get(index) {
                Some(e) => *e,
                None => ptr::null(),
            };
            
        }

        ///
        /// Get the length of the hashmap
        ///
        #[no_mangle]
        pub extern "C" fn $func_len_name(self_i: &$clazz) -> usize {
            return (self_i.$hashmap).len();
        }
    };
}
