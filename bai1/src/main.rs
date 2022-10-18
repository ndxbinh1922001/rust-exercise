fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    let mut _a: i32 = 0;
    for i in 0..org_arr.len() {
        if org_arr[i] == sub_arr[0] {
            let mut _b: i32 = 1;
            for j in 1..sub_arr.len() {
                if org_arr[i + j] == sub_arr[j] {
                    _b += 1;
                }
            }
            if _b == sub_arr.len().try_into().unwrap() {
                _a = 1;
                break;
            }
        }
    }
    if _a == 0 {
        print!("sub_arr khong phai la mang con của mang org_arr")
    } else {
        print!("sub_arr la mang con của mang org_arr")
    }
}
