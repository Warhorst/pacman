pub fn get_sub_rect(data: &[u8], data_width: usize, column: usize, row: usize, rect_width: usize, rect_height: usize) -> Vec<u8> {
    let mut rect = Vec::with_capacity(rect_width * rect_height);
    let start_index = data_width * row * rect_height;

    for y in 0..rect_height {
        let start = start_index + y * data_width + column * rect_width;
        let end = start + rect_width;
        data[start..end].into_iter().for_each(|val| rect.push(*val))
    }

    rect
}

#[cfg(test)]
mod tests {
    use crate::helper::get_sub_rect;

    #[test]
    fn get_sub_rect_works() {
        let data = vec![
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,1,1,0,0,0,0,
            0,0,1,1,0,0,0,0,
            0,0,0,0,2,2,2,2,
            0,0,0,0,2,2,2,2,
        ];

        let sub_rect_0 = get_sub_rect(data.as_slice(), 8, 1, 1, 2, 2);
        let sub_rect_1 = get_sub_rect(data.as_slice(), 8, 1, 2, 4, 2);

        assert_eq!(sub_rect_0, vec![1; 4]);
        assert_eq!(sub_rect_1, vec![2; 8]);
    }
}