use minmath::{
    probability::{Venn, complement},
    set_theory::Set,
};

#[test]
fn probability_complement() {
    assert_eq!(complement(0.0), 1.0);
    assert_eq!(complement(1.0), 0.0);
    assert_eq!(complement(0.5), 0.5);
    assert!((complement(0.25) - 0.75).abs() < 1e-12);
}

#[test]
fn venn_initialization() {
    let mut venn = Venn::<u8>::new();
    let set1 = Set::from(vec![2, 4, 6]);
    let set2 = Set::from(vec![2]);
    venn.add_set("E", set1);
    venn.add_set("P", set2);

    assert_eq!(venn.n_elements(), 3);
}
