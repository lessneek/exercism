use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}

#[test]
fn test_bigger_prime() {
    assert_eq!(np::nth(100_000), 1299721);
}

#[test]
fn test_super_big_prime() {
    assert_eq!(np::nth(10_000_000), 179424691);
}
