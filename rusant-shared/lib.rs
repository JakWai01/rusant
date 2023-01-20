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

pub fn badd() {
    let numbers = [1, 2, 3, 4, 5, 6, 7];
    let mut total = 0;

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];

            unsafe {
                shared::better_add_two_numbers(
                    a,
                    b,
                    Some(add_result_to_total),
                    &mut total as *mut c_int as *mut c_void,
                );
            }
        }
    }

    println!("The sum is {}", total);
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
