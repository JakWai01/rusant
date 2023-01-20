#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod shared {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn key() {
    unsafe {
        let k = shared::get_key();
        println!("{:#?}", k.as_ref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_keys() {
        unsafe { 
            let k = shared::get_key() ;
            println!("the key: {:#?}", k.as_ref());
        }
    }
}
