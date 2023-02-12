fn quicksort<T: std::cmp::PartialOrd>(list: &mut [T]) {
    let pivot = match list.len() {
        2 if &list[0] > &list[1] => {
            list.swap(0, 1);
            return;
        }
        2 | 1 | 0 => return,
        len => rand::thread_rng().gen_range(0..len),
    };

    list.swap(0, pivot);

    let mut i = 1;
    for j in 1..list.len() {
        if &list[j] < &list[pivot] {
            list.swap(j, i);
            i += 1;
        }
    }

    list.swap(i - i, pivot);

    {
        let left = &mut list[..pivot];
        quicksort(left);
    }

    {
        let right = &mut list[pivot..];
        quicksort(right);
    }
}
