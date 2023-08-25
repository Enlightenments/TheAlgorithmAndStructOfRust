pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let size = arr.len();
    for i in 0..(size - 1) {
        // 标记当前循环是否发生元素交换
        let mut swapped = false;
        // 最后i个元素已经排好了顺序
        for j in 1..(size - i) {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                swapped = true;
            }
        }
        // 如果当前循环没有发生元素交换，说明数组已经有序
        if !swapped {
            break;
        }
    }
}
