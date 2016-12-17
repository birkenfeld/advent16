extern crate advtools;

fn main() {
    let mut lines = advtools::iter_input::<String>();
    let first_line = lines.next().unwrap();
    let mut arrs = vec![[0; 26]; first_line.len()];
    for line in std::iter::once(first_line).chain(lines) {
        for (arr, ch) in arrs.iter_mut().zip(line.chars()) {
            arr[(ch as u8 - b'a') as usize] += 1;
        }
    }
    let collect_by_freq = |weight| arrs.iter().map(|arr| {
        let freqs = advtools::sorted(arr.iter().enumerate().map(|(i, v)| (weight * v, i)));
        (freqs[0].1 as u8 + b'a') as char
    }).collect::<String>();
    println!("Message (most common): {}", collect_by_freq(-1));
    println!("Message (least common): {}", collect_by_freq(1));
}