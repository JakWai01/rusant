use std::os::raw::{c_int, c_void};

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

unsafe extern "C" fn add_result_to_total(
    result: c_int,
    user_data: *mut c_void,
) {
    let total = &mut *(user_data as *mut c_int);
    *total += result;
}

unsafe extern "C" fn trampoline<F>(result: c_int, user_data: *mut c_void) where F: FnMut(c_int) {
    let user_data = &mut *(user_data as *mut F);
    user_data(result);
}

pub fn get_trampoline<F>(_closure: &F) -> shared::AddCallback where F: FnMut(c_int) {
    Some(trampoline::<F>)
}

pub fn badd() {
    let mut got = 0;

    {
        let mut closure = |result: c_int| got = result;
        let trampoline = get_trampoline(&closure);

        unsafe {
            shared::better_add_two_numbers(
                1,
                2,
                trampoline,
                &mut closure as *mut _ as *mut c_void,
            );
        }
    }

    println!("got {}", got);
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
