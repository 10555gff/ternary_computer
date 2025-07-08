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
    //ternary_io::test_stack_mul("++0+","+0++");





    // let stack1=vec![1,0,1,1,0,1];
    // let stack2=vec![0,0,0,1,0,2];

    // let stack1=vec![1,0,1];
    // let stack2=vec![2,0];
    
    // let re=logical_calculate::ternary_div_base(stack1,stack2);
    // print!("{:?}",re);

    // // let stack3=vec![1,0,1];
    // // println!("{}",logical_calculate::ternary_sign(&stack3));

    //     let stack1=vec![1,1];
    // let stack2=vec![1,1];
    // let result=logical_calculate::ternary_mul_base(stack1, stack2);
    // println!("乘法结果:{:?}",result);


//    let mut qq = vec![0];
//    qq =logical_calculate::ternary_stack_accumulate(qq, 1);
//    qq =logical_calculate::ternary_stack_accumulate(qq, 1);
//    //qq =logical_calculate::ternary_stack_accumulate(qq, 2);
//    qq =logical_calculate::ternary_stack_accumulate(qq, 0);
//    println!("{:?}",qq);




    // let stack1=vec![1,1];
    // let stack2=vec![1,2];
    // let re=logical_calculate::ternary_div_step(stack1,stack2);
    // println!("商和余数:{:?}",re);

    // let stack1=vec![2,2];
    // let stack2=vec![2,1];
    // let re=logical_calculate::ternary_div_step(stack1,stack2);
    // println!("商和余数:{:?}",re);


    // let stack1=vec![1,1];
    // let stack2=vec![2,1];
    // let re=logical_calculate::ternary_div_step(stack1,stack2);
    // println!("商和余数:{:?}",re);

    // let stack1=vec![2,2];
    // let stack2=vec![1,2];
    // let re=logical_calculate::ternary_div_step(stack1,stack2);
    // println!("商和余数:{:?}",re);

//println!("//---------------------------------------------------------------------------------------//");

    // let stack1=vec![1,0,0];
    // let stack2=vec![1,0,2];
    // let re=logical_calculate::ternary_div_step(stack1,stack2);
    // println!("商和余数:{:?}",re);

    // let stack1=vec![2,0,0];
    // let stack2=vec![2,0,1];
    // let re=logical_calculate::ternary_div_step(stack1,stack2);
    // println!("商和余数:{:?}",re);


    // let stack1=vec![2,0,0];
    // let stack2=vec![1,0,2];
    // let re=logical_calculate::ternary_div_step(stack1,stack2);
    // println!("商和余数:{:?}",re);

    // let stack1=vec![1,0,0];
    // let stack2=vec![2,0,1];
    // let re=logical_calculate::ternary_div_step(stack1,stack2);
    // println!("商和余数:{:?}",re);

//println!("//---------------------------------------------------------------------------------------//");



//     let stack1=vec![1,0,1];
//     let stack2=vec![2,1];
//     let re=logical_calculate::ternary_div_base2(stack1,stack2);

//    



    let stack1=vec![1,0,1];
    let stack2=vec![2,1];
    let re=logical_calculate::ternary_div_base2(stack1,stack2);
    println!("商和余数:{:?}",re);

    let stack1=vec![1,0,1];
    let stack2=vec![1,0];
    let re=logical_calculate::ternary_div_base2(stack1,stack2);
    println!("商和余数:{:?}",re);

    let stack1=vec![1,2,1,1,2];//65/5
    let stack2=vec![1,2,2];
    let re=logical_calculate::ternary_div_base2(stack1,stack2);
    println!("商和余数:{:?}",re);

    let stack1=vec![1,0,1,1,0,1];//280/8
    let stack2=vec![1,0,2];
    let re=logical_calculate::ternary_div_base2(stack1,stack2);
    println!("商和余数:{:?}",re);

    let stack1=vec![1,2,1,1];//22/-5
    let stack2=vec![2,1,1];
    let re=logical_calculate::ternary_div_base2(stack1,stack2);
    println!("商和余数:{:?}",re);


}
