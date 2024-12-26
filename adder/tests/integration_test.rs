use adder::add;

#[test]
fn it_adds_two() {
    let result = add(3, 5);
    assert_eq!(result, 8);
}

/**
 * If the important functionality works, 
 * the small amount of code in the src/main.rs 
 * file will work as well, and that small amount 
 * of code doesn’t need to be tested.
 */