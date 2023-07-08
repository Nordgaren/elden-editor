pub fn copy_slice<T: Copy>(src: &[T], dest: &mut [T]) {
    for i in 0..src.len() {
        dest[i] = src[i]
    }
}
