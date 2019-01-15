#![feature(uniform_paths)]

mod test_helper;

use tests_writing::{libx::x, liby::y};
use test_helper::helper;

#[test]
fn test_together() {
    assert_eq!(x() + y(), 66);
    assert_eq!(helper(), "Hello");
}
