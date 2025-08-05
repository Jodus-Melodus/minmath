use minmath::set_theory::Set;

#[test]
fn initialization() {
    let mut set = Set::<u8>::new();
    set.name = "S";
    assert_eq!(0, set.cardinality());
    println!("{}", set);
}

#[test]
fn add_element() {
    let mut set = Set::<u8>::new();
    set.add_element(4);
    let expected = Set::from(vec![4]);

    assert_eq!(1, set.cardinality());
    assert_eq!(set, expected);
    println!("{}", set);
}
