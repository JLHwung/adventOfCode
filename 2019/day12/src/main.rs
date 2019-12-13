type Pos = (i32, i32, i32);

fn main() {
    let m1pos: Pos = (-6i32, -5i32, -8i32);
    let m2pos: Pos = (0i32, -3i32, -13i32);
    let m3pos: Pos = (-15i32, 10i32, -11i32);
    let m4pos: Pos = (-3i32, -8i32, 3i32);

    let mut pos: [Pos; 4] = [m1pos, m2pos, m3pos, m4pos];

    let mut velocity = [
        (0i32, 0i32, 0i32),
        (0i32, 0i32, 0i32),
        (0i32, 0i32, 0i32),
        (0i32, 0i32, 0i32),
    ];

    let mut step = 0;

    loop {
        // apply gravity
        for i in 0..4 {
            let mut v = velocity[i];
            for j in 0..4 {
                if j == i {
                    continue;
                }
                v.0 += sign(pos[i].0, pos[j].0);
                v.1 += sign(pos[i].1, pos[j].1);
                v.2 += sign(pos[i].2, pos[j].2);
            }
            velocity[i] = v;
        }

        // apply velocity
        for i in 0..4 {
            pos[i].0 += velocity[i].0;
            pos[i].1 += velocity[i].1;
            pos[i].2 += velocity[i].2;
        }

        step += 1;

        if step == 1000 {
            break;
        }
    }

    let mut energy = 0i32;

    for i in 0..4 {
        energy += sum_abs(pos[i]) * sum_abs(velocity[i]);
    }

    println!("Total Energy: {}", energy);
}

fn sum_abs(pos: Pos) -> i32 {
    pos.0.abs() + pos.1.abs() + pos.2.abs()
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
