type Pos = [i32; 3];

fn simulate_until_reset(dimension: usize) -> u64 {
    let m1pos: Pos = [-6i32, -5i32, -8i32];
    let m2pos: Pos = [0i32, -3i32, -13i32];
    let m3pos: Pos = [-15i32, 10i32, -11i32];
    let m4pos: Pos = [-3i32, -8i32, 3i32];

    let mut pos: [Pos; 4] = [m1pos, m2pos, m3pos, m4pos];

    let mut velocity: [Pos; 4] = [
        [0i32, 0i32, 0i32],
        [0i32, 0i32, 0i32],
        [0i32, 0i32, 0i32],
        [0i32, 0i32, 0i32],
    ];

    let mut step: u64 = 0;

    loop {
        // apply gravity
        for i in 0..4 {
            let mut v = velocity[i];
            for j in 0..4 {
                if j == i {
                    continue;
                }
                v[dimension] += sign(pos[i][dimension], pos[j][dimension]);
            }
            velocity[i] = v;
        }

        step += 1;

        // apply velocity
        for i in 0..4 {
            pos[i][dimension] += velocity[i][dimension];
        }

        step += 1;

        let kin_dimension: i32 = velocity.iter().map(|x| x[dimension].abs()).sum();
        if kin_dimension == 0 {
            println!(
                "State of dimension {} is reset after step: {}",
                dimension, step
            );
            break;
        }
    }

    step
}

fn main() {
    let total_steps = [0, 1, 2]
        .iter()
        .map(|d| simulate_until_reset(*d))
        .fold(1, |acc, x| lcm(acc, x));
    println!("State is reset after step: {}", total_steps);
}

fn gcd(m: u64, n: u64) -> u64 {
    if m == 0 {
        n
    } else {
        gcd(n % m, m)
    }
}

fn lcm(m: u64, n: u64) -> u64 {
    m * n / gcd(m, n)
}

fn sign(_self: i32, other: i32) -> i32 {
    if _self == other {
        0i32
    } else if other > _self {
        1i32
    } else {
        -1i32
    }
}
