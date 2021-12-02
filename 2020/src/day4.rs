use regex::Regex;
use std::fs;
use std::io;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day4.txt").unwrap()).unwrap();
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

fn process(raw: &str) -> Vec<&str> {
    let mut result: Vec<&str> = vec![];
    for line in raw.split("\n\n") {
        if line.is_empty() {
            continue;
        }
        result.push(line);
    }
    result
}

fn p1(input: &Vec<&str>) -> usize {
    input
        .iter()
        .filter(|line| {
            // allValid: 0x7f
            let mut valid_flag: u8 = 0x00;
            for item in line.split_whitespace() {
                let mut colon_iter = item.split(":");
                let key = colon_iter.next().unwrap();
                match key {
                    "byr" => valid_flag |= 0x1,
                    "iyr" => valid_flag |= 0x2,
                    "eyr" => valid_flag |= 0x4,
                    "hgt" => valid_flag |= 0x8,
                    "hcl" => valid_flag |= 0x10,
                    "ecl" => valid_flag |= 0x20,
                    "pid" => valid_flag |= 0x40,
                    "cid" => {}
                    _ => unimplemented!(),
                }
            }
            return valid_flag == 0x7f;
        })
        .count()
}

#[derive(Debug)]
enum HeightUnit {
    Cm,
    Inch,
}

#[derive(Debug)]
struct Height {
    value: usize,
    unit: HeightUnit,
}

#[derive(Debug, Clone)]
struct ParseHeightError;

impl FromStr for Height {
    type Err = ParseHeightError;

    // 162cm
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let unit = &s[len - 2..len];
        let val = s[0..len - 2]
            .parse::<usize>()
            .map_err(|_| ParseHeightError)?;
        match unit {
            "cm" => Ok(Height {
                value: val,
                unit: HeightUnit::Cm,
            }),
            "in" => Ok(Height {
                value: val,
                unit: HeightUnit::Inch,
            }),
            _ => Err(ParseHeightError),
        }
    }
}

/**

byr (Birth Year) - four digits; at least 1920 and at most 2002.
iyr (Issue Year) - four digits; at least 2010 and at most 2020.
eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
hgt (Height) - a number followed by either cm or in:
If cm, the number must be at least 150 and at most 193.
If in, the number must be at least 59 and at most 76.
hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pid (Passport ID) - a nine-digit number, including leading zeroes.
cid (Country ID) - ignored, missing or not.

*/
fn p2(input: &Vec<&str>) -> usize {
    let hcl_reg = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecl_reg = Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_reg = Regex::new(r"^[0-9]{9}$").unwrap();
    input
        .iter()
        .filter(|line| {
            // allValid: 0x7f
            let mut valid_flag: u8 = 0x00;
            for item in line.split_whitespace() {
                let mut colon_iter = item.split(":");
                let key = colon_iter.next().unwrap();
                let value = colon_iter.next().unwrap();
                match key {
                    "byr" => match value.parse::<usize>() {
                        Ok(v) => {
                            if v >= 1920 && v <= 2002 {
                                valid_flag |= 0x1
                            }
                        }
                        Err(_) => continue,
                    },
                    "iyr" => match value.parse::<usize>() {
                        Ok(v) => {
                            if v >= 2010 && v <= 2020 {
                                valid_flag |= 0x2
                            }
                        }
                        Err(_) => continue,
                    },
                    "eyr" => match value.parse::<usize>() {
                        Ok(v) => {
                            if v >= 2020 && v <= 2030 {
                                valid_flag |= 0x4
                            }
                        }
                        Err(_) => continue,
                    },
                    "hgt" => match value.parse::<Height>() {
                        Ok(v) => {
                            let value = v.value;
                            match v.unit {
                                HeightUnit::Cm => {
                                    if value >= 150 && value <= 193 {
                                        valid_flag |= 0x8;
                                    }
                                }
                                HeightUnit::Inch => {
                                    if value >= 59 && value <= 76 {
                                        valid_flag |= 0x8;
                                    }
                                }
                            }
                        }
                        Err(_) => continue,
                    },
                    "hcl" => {
                        if hcl_reg.is_match(value) {
                            valid_flag |= 0x10;
                        }
                    }
                    "ecl" => {
                        if ecl_reg.is_match(value) {
                            valid_flag |= 0x20;
                        }
                    }
                    "pid" => {
                        if pid_reg.is_match(value) {
                            valid_flag |= 0x40;
                        }
                    }
                    "cid" => {}
                    _ => unimplemented!(),
                }
            }
            return valid_flag == 0x7f;
        })
        .count()
}
