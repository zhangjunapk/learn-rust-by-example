mod common;

//直接测试整个模块对外提供的功能
#[test]
fn test_add() {
    common::setup();
    unit_test::add(1, 2);
}
