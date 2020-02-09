fn dense_hash(numbers: &[usize]) -> String {
    let mut result = String::from("");
    for chunk in numbers.chunks(16) {
        let mut chunk_iter = chunk.iter();
        let first = *chunk_iter.next().unwrap();
        let bxor = chunk_iter.fold(first, |acc, n| acc ^ n);
        result.push_str(&format!("{:02x}", bxor));
    }
    result
}

pub fn hash(input: &[usize], rounds: usize) -> (Vec<usize>, String) {
    let mut numbers: Vec<usize> = (0..=255).collect();
    let numbers_len = numbers.len();
    let mut skip_size = 0;
    let mut rotations = 0;
    for _ in 0..rounds {
        for length in input.iter() {
            let (l, r) = numbers.split_at(*length);
            let mut left = l.to_vec();
            left.reverse();
            left.extend_from_slice(&r);
            numbers = left;
            let curpos = (skip_size + *length) % numbers_len;
            numbers.rotate_left(curpos);
            rotations += curpos;
            skip_size += 1;
        }
    }
    numbers.rotate_right(rotations % numbers_len);
    let dense = dense_hash(&numbers);

    (numbers, dense)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dense_hash1() {
        let numbers = [65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
        assert_eq!("40", dense_hash(&numbers));
        assert_eq!(
            "00",
            dense_hash(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])
        );
    }
}
