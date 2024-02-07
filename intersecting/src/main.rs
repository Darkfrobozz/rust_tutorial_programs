struct Rec {
    position: (i32, i32),
    dimensions: (i32, i32),
}

fn main() {
    println!("Hello, world!");
    let highrec = Rec {
        position: (4, 4),
        dimensions: (10, 10),
    };

    let lowrec = Rec {
        position: (0, 10),
        dimensions: (10, 10),
    };
    assert_eq!(true, does_intersect(&highrec, &lowrec));
}
fn does_intersect(rec1 : &Rec, rec2 : &Rec) -> bool {
    let (lowest, highest) = if rec1.position.1 > rec2.position.1 {
        (rec2, rec1)
    } else {
        (rec1, rec2)
    };
    let (left, right) = if rec1.position.0 > rec2.position.0 {
        (rec2, rec1)
    } else {
        (rec1, rec2)
    };

    if highest.position.1 - highest.dimensions.1/2 
        > lowest.position.1 + lowest.dimensions.1/2 {
        return false;
    } else if left.position.0 + left.dimensions.0/2 
        < right.position.0 - right.dimensions.0/2 {
        return false;
    }
    true
}

