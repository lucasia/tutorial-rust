// Unsafe superpowers:
// 1. Dereference a raw pointer.
// 2. Call an unsafe function or method.
// 3. Access or modify a mutable static variable.
// 4. Implement an unsafe trait. // see Send and Sync
// 5. Access fields of unions. // mostly for C Unions

#[cfg(test)]
mod tests {
    use test_log::test;

    // ================================================
    // 1. Dereference a raw pointer.
    // ================================================
    #[test]
    fn test_raw_pointer() {
        let mut num = 5;

        let r1 = &raw const num;
        let r2 = &raw mut num;

        unsafe {
            assert_eq!(5, *r1);
            assert_eq!(5, *r2);
        }
    }

    // ================================================
    // 2. Call an unsafe function or method.
    // ================================================
    unsafe fn dangerous() {}

    #[test]
    fn test_unsafe_fn() {
        unsafe {
            dangerous();
        }
    }

    fn my_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    #[test]
    fn test_split_at_mut() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = my_split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    // ================================================
    // 3. Access or modify a mutable static variable.
    // ================================================

    static mut COUNTER: u32 = 0;

    /// SAFETY: Calling this from more than a single thread at a time is undefined
    /// behavior, so you *must* guarantee you only call it from a single thread at
    /// a time.
    unsafe fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    #[test]
    fn test_increment_count() {
        unsafe {
            // SAFETY: This is only called from a single thread in `main`.
            add_to_count(3);

            assert_eq!(3, *(&raw const COUNTER));
        }
    }
}
