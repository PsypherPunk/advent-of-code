use std::collections::HashSet;

peg::parser! {
    pub grammar scanner() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> isize
            = n:$("-"* ['0'..='9']+) {? n.parse().or(Err("Invalid integer.")) }

        rule beacon() -> [isize; 3]
            = x:integer() "," y:integer() "," z:integer()
                { [x, y, z] }

        rule scanner() -> Vec<[isize; 3]>
            = "--- scanner " integer() " ---"
              _
              beacons:beacon() ++ _
                { beacons }

        pub rule scanners() -> Vec<Vec<[isize; 3]>>
            = scanners:scanner() ++ _
                { scanners }

    }
}

fn get_rotation([x, y, z]: [isize; 3], rotation: u8) -> [isize; 3] {
    match rotation {
        0 => [x, y, z],
        1 => [x, z, -y],
        2 => [x, -y, -z],
        3 => [x, -z, y],
        4 => [y, x, -z],
        5 => [y, z, x],
        6 => [y, -x, z],
        7 => [y, -z, -x],
        8 => [z, x, y],
        9 => [z, y, -x],
        10 => [z, -x, -y],
        11 => [z, -y, x],
        12 => [-x, y, -z],
        13 => [-x, z, y],
        14 => [-x, -y, z],
        15 => [-x, -z, -y],
        16 => [-y, x, z],
        17 => [-y, z, -x],
        18 => [-y, -x, -z],
        19 => [-y, -z, x],
        20 => [-z, x, -y],
        21 => [-z, y, x],
        22 => [-z, -x, y],
        23 => [-z, -y, -x],
        _ => unreachable!(),
    }
}

fn find_matching_beacons(beacons: &mut HashSet<[isize; 3]>, scan: &[[isize; 3]]) -> bool {
    for rotation in 0..24 {
        let scan_rotations = scan
            .iter()
            .map(|&beacon| get_rotation(beacon, rotation))
            .collect::<Vec<_>>();

        let distances = beacons.iter().flat_map(|&[x1, y1, z1]| {
            scan_rotations
                .iter()
                .map(move |&[x2, y2, z2]| [x1 - x2, y1 - y2, z1 - z2])
        });

        for [dx, dy, dz] in distances {
            let shifted_scan_rotations = scan_rotations
                .iter()
                .map(|[x, y, z]| [x + dx, y + dy, z + dz])
                .collect::<Vec<_>>();

            let matching_beacon_count = shifted_scan_rotations
                .iter()
                .filter(|&beacon| beacons.contains(beacon))
                .count();
            if matching_beacon_count >= 12 {
                beacons.extend(shifted_scan_rotations);
                return true;
            }
        }
    }
    false
}

pub fn get_part_one(input: &str) -> usize {
    let mut scans = scanner::scanners(input.trim()).unwrap();

    let mut beacons = scans.remove(0).into_iter().collect::<HashSet<_>>();

    while !scans.is_empty() {
        for i in (0..scans.len()).rev() {
            if find_matching_beacons(&mut beacons, &scans[i]) {
                // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove
                scans.swap_remove(i);
            }
        }
    }

    beacons.len()
}

pub fn get_part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(79, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
