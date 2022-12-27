fn main() {
    let mut arr = vec![1, 2, -3, 3, -2, 0, 4];

    let mut indices = arr
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<Vec<_>>();

    dbg!(&indices);

    for i in 0..indices.len() {
        let value = arr.remove(i) as i32;
        let new_idx = value + i as i32;
        arr.insert(new_idx as usize, value);
        indices[i].1 = new_idx as usize;
        for j in i..(new_idx as usize) {
            indices[j].1 -= value as usize;
        }
    }
}
