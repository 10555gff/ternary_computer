mod ternary_utils;// 引入模块
use ternary_utils::ternary_io;
use ternary_utils::logical_calculate;

fn main() {
    // let a = 1;
    // let b = 1;
    // let c=1;
    // ternary_io::test_half_adder(a, b);//半加器
    // ternary_io::test_full_adder(a, b, c);//平衡全加器

    // let s1=String::from("++++++++");
    // let s2=String::from("+0000000");

    // ternary_io::test_stack_adder(&s1, &s2);
    // ternary_io::test_stack_adder("++++++++", "+0000000");



    let stack1=vec![1,1,0,1];
    let stack2=vec![1,0,1,1];
    let re=logical_calculate::ternary_mul_base(stack1,stack2);
    print!("{:?}",re);
}
