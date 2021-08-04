#[cfg(test)]
mod tests {
    use std::mem::transmute;

    #[test]
    fn transmute_experiment1() {
        let x = !0u64;
        let y: [u8; 8] = unsafe { transmute(x) };
        assert_all_ones(&y);
    }

    #[test]
    fn transmute_experiment2() {
        let x = !0u64;
        let y = unsafe { transmute::<u64, [u8; 8]>(x) };
        assert_all_ones(&y);
    }

    #[test]
    fn gazebo_transmute() {
        let x = !0u64;
        let y = unsafe { gazebo::transmute!(u64, [u8; 8], x) };
        assert_all_ones(&y);
    }

    #[test]
    fn ptr_usize() {
        let s = String::from("foo");
        let ptr = &s;

        let usz = gazebo::cast::ptr_to_usize(ptr);
        let ptr: &String = unsafe { gazebo::cast::usize_to_ptr(usz) };

        assert_eq!(&s, ptr);
        assert_eq!(s, *ptr);
    }

    #[test]
    fn cast_string() {
        let s = String::from("あいう");
        let v: &Vec<u8> = unsafe { gazebo::cast::ptr(&s) };

        assert_eq!(&s as *const String as usize, v as *const Vec<u8> as usize);
    }

    // utilities

    fn assert_all_ones(bytes: &[u8; 8]) {
        let ones: u8 = 0b1111_1111;
        for (idx, &b) in bytes.iter().enumerate() {
            assert_eq!(b, ones, "where idx = {}", idx);
        }
    }
}
