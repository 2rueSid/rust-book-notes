pub fn linear<const T: usize>(arr: [i32; T], v: i32) -> bool {
    for i in 0..arr.len() {
        if arr[i] == v {
            return true;
        }
    }

    false
}

pub fn binary_search_rec(arr: &[i32], v: i32) -> bool {
    if arr.is_empty() {
        return false;
    }

    if arr.len() == 1 {
        return arr[0] == v;
    }

    let mid = arr.len() / 2;

    if v == arr[mid] {
        return true;
    } else if arr[mid] > v {
        return binary_search_rec(&arr[..mid], v);
    } else {
        return binary_search_rec(&arr[mid..], v);
    }
}

pub fn binary_search_loop(arr: &[i32], v: i32) -> bool {
    let mut l = 0;
    let mut hi = arr.len();

    while l < hi {
        let mid = l + (hi - l) / 2;

        if arr[mid] == v {
            return true;
        } else if arr[mid] > v {
            hi = mid;
        } else {
            l = mid + 1;
        }
    }

    false
}

pub fn two_balls(arr: &[bool]) -> i32 {
    if arr.is_empty() {
        return -1;
    }
    let jump = (f32::sqrt(arr.len() as f32)).ceil() as usize;
    let mut i = jump;

    while i < arr.len() {
        if arr[i] {
            break;
        }

        i += jump;
    }

    i -= jump;
    let mut j = 0;

    while j <= jump && i < arr.len() {
        if arr[i] {
            return i as i32;
        }

        j += 1;
        i += 1;
    }

    -1
}
