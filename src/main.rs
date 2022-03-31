fn merge_sort(src: &mut [i32]) {
    let mut scratch = vec![0; src.len()];
    merge_alg(src, &mut scratch);

    fn merge_alg(src: &mut [i32], scratch: &mut [i32]) {
        if src.len() <= 1 {
            return;
        }
        let (left, right) = src.split_at_mut(src.len() / 2);
        merge_alg(left, scratch);
        merge_alg(right, scratch);
        if merge(left, right, scratch) {
            src.copy_from_slice(&scratch[..src.len()]);
        }
        
    }

    fn merge(left: &mut [i32], right: &mut [i32], scratch: &mut [i32]) -> bool {
        if left.last() < right.first() {
            // left and right are already in correct order
            return false;
        }
        let mut left_iter = left.iter().peekable();
        let mut right_iter = right.iter().peekable();
        let mut pos: usize = 0;


        while let (Some(left_val), Some(right_val)) = (left_iter.peek(), right_iter.peek()) {
            if left_val <= right_val {
                scratch[pos] = **left_val;
                left_iter.next();
            } else {
                scratch[pos] = **right_val;
                right_iter.next();
            }
            pos += 1;
        }
        for val in left_iter.chain(right_iter) {
            scratch[pos] = *val;
            pos += 1;
        }
        return true;
    }
}

fn main() {
    let mut test = [100, 3, 6, 8, 9, 2, 87, 90, 122, 33, 44, 54];
    merge_sort(&mut test);
    println!("{:?}", test)
}
