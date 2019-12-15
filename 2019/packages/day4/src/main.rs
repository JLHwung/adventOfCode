use std::io;

fn main() -> io::Result<()> {
    let mut count = 0;
    let mut cursor = 158126;
    while cursor <= 624574 {
        let mut digit_pairs: Vec<[char; 2]> = Vec::new();
        let mut i = 0;

        let digits: String = cursor.to_string();

        while i + 1 < digits.len() {
            let mut dp = digits[i..i + 2].chars();
            digit_pairs.push([dp.next().unwrap(), dp.next().unwrap()]);
            i += 1;
        }

        let equal_pairs: Vec<&[char; 2]> = digit_pairs.iter().filter(|p| p[0] == p[1]).collect();
        if digit_pairs.iter().all(|p| p[0] <= p[1])
            && equal_pairs
                .iter()
                .any(|p| equal_pairs.iter().filter(|q| p[0] == q[0]).count() == 1)
        {
            count += 1;
        }

        cursor += 1;
    }

    println!("{}", count);

    Ok(())
}
