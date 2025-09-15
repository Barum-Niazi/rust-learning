fn main() {
    let mut i = 0;
    while i <= 5 {
        print!("{i}\n");
        if i == 1 {
            i += 1;
            continue;
        }
        if i == 2 {
            break;
        }
        i += 1;
    }
}
