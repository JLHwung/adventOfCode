use std::collections::{HashMap, HashSet};

macro_rules! DATA_PATH {
    () => {
        "../data/day19.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let mut input = process(raw);
    println!("Answer of p1: {}", p1(&mut input));
    println!("Answer of p2: {}", p2(&input));
}

type Number = i32;

type Point = [Number; 3];

type Vector = Point;

type Permute = [Vector; 3];

const PERMUTATION: [Permute; 24] = [
    //
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    //
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    //
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    //
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
    //
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    //
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
];

const OVERLAP_THRESHOLD: usize = 12 * 11;

fn add(lhs: &Point, rhs: &Point) -> Point {
    [lhs[0] + rhs[0], lhs[1] + rhs[1], lhs[2] + rhs[2]]
}

fn sub(lhs: &Point, rhs: &Point) -> Point {
    [lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2]]
}

fn dot(lhs: &Point, rhs: &Point) -> Number {
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

fn rotate(source: &Point, permute: &Permute) -> Point {
    [
        dot(&permute[0], source),
        dot(&permute[1], source),
        dot(&permute[2], source),
    ]
}

struct Scanner {
    position: Option<Point>,
    points: Vec<Point>,
}

#[derive(Debug)]
struct PointTransform<'a> {
    permute: &'a Permute,
    offset: Vector,
}

type Input = Vec<Scanner>;

type VectorWithStart = HashMap<Vector, Point>;

fn process(raw: &str) -> Input {
    raw.split("\n\n")
        .map(|result| Scanner {
            position: None,
            points: result
                .lines()
                .skip(1)
                .map(|beacon_text| {
                    let pos: Vec<Number> = beacon_text
                        .splitn(3, ',')
                        .map(|x| x.parse().unwrap())
                        .collect();
                    [pos[0], pos[1], pos[2]]
                })
                .collect(),
        })
        .collect()
}

fn generate_vector_with_start(points: &[Point]) -> VectorWithStart {
    let mut result = HashMap::new();
    for i in 0..points.len() {
        // todo: we can optimize here by only computing i < j and derive opposite
        for j in 0..points.len() {
            if i == j {
                continue;
            }
            let vector = sub(&points[j], &points[i]);
            result.insert(vector, points[i]);
        }
    }
    // This assertion requires that the vector is not equal to any other vectors
    assert!(result.len() == points.len() * (points.len() - 1));
    result
}

fn point_transform<'a>(source: &'a Point, transform: &PointTransform) -> Point {
    let PointTransform { permute, offset } = transform;
    add(
        &[
            dot(&permute[0], source),
            dot(&permute[1], source),
            dot(&permute[2], source),
        ],
        offset,
    )
}

fn vectors_rotate(source: &HashMap<Vector, Point>, permute: &Permute) -> HashMap<Vector, Point> {
    source
        .iter()
        .map(|(vector, &data)| (rotate(vector, permute), data))
        .collect()
}

fn vector_starts_transform(source: &mut HashMap<Vector, Point>, transform: &PointTransform) {
    for (_, data) in source.iter_mut() {
        *data = point_transform(data, transform);
    }
}

fn get_shared_vector_when_overlapping_threshold_reached(
    lhs: &HashMap<Vector, Point>,
    rhs: &HashMap<Vector, Point>,
) -> Option<Vector> {
    let mut count = 0;
    let mut result = None;
    for &vector in rhs.keys() {
        if lhs.contains_key(&vector) {
            count += 1;
            if result.is_none() {
                result = Some(vector);
            }
        }
        if count >= OVERLAP_THRESHOLD {
            return result;
        }
    }
    None
}

fn try_extend_aligned_vectors(
    aligned_vectors: &mut VectorWithStart,
    unaligned: &VectorWithStart,
) -> Option<Point> {
    for permute in PERMUTATION.iter() {
        let mut transformed_source = vectors_rotate(unaligned, permute);
        if let Some(vector) = get_shared_vector_when_overlapping_threshold_reached(
            aligned_vectors,
            &transformed_source,
        ) {
            // Permutation found
            let aligned_point = aligned_vectors.get(&vector).unwrap();
            // The same beacon with point representation in the unaligned scanner
            let unaligned_point = *transformed_source.get(&vector).unwrap();
            let offset = sub(aligned_point, &rotate(&unaligned_point, permute));
            vector_starts_transform(&mut transformed_source, &PointTransform { permute, offset });
            aligned_vectors.extend(&transformed_source);
            return Some(offset);
        }
    }
    None
}

fn p1(input: &mut [Scanner]) -> usize {
    let mut first_scanner = &mut input[0];
    first_scanner.position = Some([0, 0, 0]);
    let aligned_vectors = &mut generate_vector_with_start(&input[0].points);
    // push other vectors to vector starts
    let mut vector_starts = vec![];
    for scanner in input.iter().skip(1) {
        vector_starts.push(generate_vector_with_start(&scanner.points));
    }

    let mut aligned_set = HashSet::from([0]);
    loop {
        for i in 1..input.len() {
            if aligned_set.contains(&i) {
                continue;
            }
            if let Some(point) = try_extend_aligned_vectors(aligned_vectors, &vector_starts[i - 1])
            {
                input[i].position = Some(point);
                aligned_set.insert(i);
            }
        }
        if aligned_set.len() == input.len() {
            break;
        }
    }

    // sum the unique vector starting points
    aligned_vectors.values().collect::<HashSet<_>>().len()
}

fn norm_l1(vector: &Vector) -> Number {
    vector[0].abs() + vector[1].abs() + vector[2].abs()
}

fn p2(input: &[Scanner]) -> Number {
    let mut max_distance = Number::MIN;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            max_distance = Number::max(
                max_distance,
                norm_l1(&sub(
                    &input[j].position.unwrap(),
                    &input[i].position.unwrap(),
                )),
            );
        }
    }
    max_distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1_and_p2() {
        let raw = include_str!(DATA_PATH!());
        let mut input = process(raw);
        assert_eq!(p1(&mut input), 512);
        assert_eq!(p2(&input), 16802);
    }
}
