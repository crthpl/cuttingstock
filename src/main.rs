use std::collections::HashMap;
use std::rc::Rc;
fn main() {
    //vector:
    /*60
    65
    72
    78
    87
    98
    108
    119
    127
    130
    135
    142
    151
    157
    162
    169
    174
    104
    110
    119
    127
    137
    145
    146
    151
    94
    102
    110
    119
    126
    134
    140
    150
    155
    111
    118
    126
    132
    137
    137*/
    let posts = vec![
        60,  // 1
        65,  // 2
        72,  // 3
        78,  // 4
        87,  // 5
        98,  // 6
        108, // 7
        119, // 8
        127, // 9
        130, // 10
        135, // 11
        142, // 12
        151, // 13
        157, // 14
        162, // 15
        169, // 16
        174, // 17
        104, // 18
        110, // 19
        119, // 20
        127, // 21
        137, // 22
        145, // 23
        146, // 24
        151, // 25
        94,  // 26
        102, // 27
        110, // 28
        119, // 29
        126, // 30
        134, // 31
        140, // 32
        150, // 33
        155, // 34
        111, // 35
        118, // 36
        126, // 37
        132, // 38
        137, // 39
        137, // 40
    ];
    let woods = vec![243, 304]; // TODO only like 14 243s and 30 304s
    let mut combos: HashMap<Combo, u32> = HashMap::new(); // (bitfield of which posts, wood used) :: leftover
    for wood in woods {
        enumerate_combos(&mut combos, 0, wood, &posts, wood);
    }
    println!("{:?}", combos);
    for (combo, left) in combos.iter() {
        println!("{:#043b} [{}] left: {}", combo.posts, combo.wood, left);
    }
    println!("{} combos", combos.len());
    println!("{} posts", posts.len());
    let cuts = cut_all(0, &combos, &posts, Combos::Root, 0);
    println!("{:?}", cuts);
    println!("{} cuts", cuts.len());
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Combo {
    posts: u64, // bitfield
    wood: u32,
}

#[derive(Debug)]
struct Cutset {
    combos: Combos,
}

fn enumerate_combos(
    combos: &mut HashMap<Combo, u32>,
    bitfield: u64,
    leftover: u32,
    posts: &Vec<u32>,
    wood: u32,
) {
    for (i, len) in posts.iter().enumerate() {
        if *len > leftover {
            continue;
        }
        let new_leftover = leftover - len;
        let new_bitfield: u64 = bitfield | (1 << i);
        let new_posts: Vec<u32> = posts.iter().skip(i + 1).map(|x| *x).collect::<Vec<_>>();
        enumerate_combos(combos, new_bitfield, new_leftover, &new_posts, wood);
    }
    if bitfield != 0 {
        combos.insert(
            Combo {
                posts: bitfield,
                wood,
            },
            leftover,
        );
    }
}

// create a singly linked list
#[derive(Debug)]
enum Combos {
    Leaf(Combo, Rc<Combos>),
    Root,
}

fn cut_all(
    bitfield: u64,
    combos: &HashMap<Combo, u32>,
    posts: &Vec<u32>,
    comboset: Combos,
    depth: u32,
) -> Vec<Cutset> {
    if depth > 50 {
        panic!("too deep")
    }
    //println!("depth {} bitfield: {:#043b}", depth, bitfield);
    let mut cuts: Vec<Cutset> = Vec::new();
    let comboset2 = Rc::new(comboset);
    for (combo, leftover) in combos.iter() {
        if (combo.posts & bitfield) != 0 {
            // already did that post
            continue;
        }
        let new_comboset = Combos::Leaf(combo.clone(), comboset2.clone());
        if (combo.posts | bitfield) == 0b11111_11111_11111_11111__11111_11111_11111_11111 {
            println!("found!");
            cuts.push(Cutset {
                combos: new_comboset,
            });
        } else {
            //println!("not skiiped &: {} posts: {}", combo.posts & bitfield, combo.posts);
            cuts.append(&mut cut_all(
                bitfield | combo.posts,
                combos,
                posts,
                new_comboset,
                depth + 1,
            ))
        }
    }
    cuts
}
