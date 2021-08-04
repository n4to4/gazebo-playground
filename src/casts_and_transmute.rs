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

    // utilities

    fn assert_all_ones(bytes: &[u8; 8]) {
        let ones: u8 = 0b1111_1111;
        for (idx, &b) in bytes.iter().enumerate() {
            assert_eq!(b, ones, "where idx = {}", idx);
        }
    }
}
