pub fn common_concepts() {

    /* 
    
    you cannot re-assign a value if you declare a variable in rust bu default

    let x = 5;
    println!("The value of {x}")
    x = 6;
    println!("The value of {x}")

    ------- cannot mutate immutable variable `x` -------
    
    
    */

    // if  you add * mut * keyword before variable it will become mutable that me you can reassign latter
    let mut x = 5;
    println!("The value of {x}");
    x = 6;
    println!("The value of {x}");

    
    /*  

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    we use constance the variable is never going to change in future
    key differentness between variables and constance are
        1. cannot be prefixed with mut keyword to mutate
        2. you have declare with const keyword
        3. naming convention used by all developers is all capital letters word separated y underscore
        4. data type must be annotated
        5.         
    */


    // shadowing 
    let a = 10; // original value
    let a = a + 1; // add one to original  -> 11

    {
        // the value of a is 11 here and if it multiplied by 2, it will be 22
        // this 22 will be only for this scope
        let a = a * 2;
        println!("the value of a in inner scope is {a}");
    }
    // here value is just 11 since it does not know about what happened previous scope
    println!("the value of a in outer scope is {a}");

    /*
        This technique is called shadowing 
         1. This is different way of making variable mutate
         2. The variable be immutable after those transformations have been completed
    */


    
    /*
        // Data types
        
        Length	Signed	Unsigned
        8-bit	i8  	u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128	u128
        arch	isize	usize


        let result = 10;    // i32 by default
        let age:u32 = 20;
        let sum:i32 = 5-15;
        let mark:isize = 10;
        let count:usize = 30;
        println!("result value is {}",result);
        println!("sum is {} and age is {}",sum,age);
        println!("mark is {} and count is {}",mark,count)


        let result = 10.00;        //f64 by default
        let interest:f32 = 8.35;
        let cost:f64 = 15000.600;  //double precision
        
        println!("result value is {}",result);
        println!("interest is {}",interest);
        println!("cost is {}",cost);

    */
    another_function(5);

    let x = five();
    println!("{x}");

}

fn another_function(x: i32) {
    println!("Another function is called ${x}");

    let number = 6;

    // control flow
    if number != 0{
        println!("True")
    }


    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    //just like turnery operator
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // let condition = true;

    // let number = if condition { 5 } else { "six" }; // error here

    println!("The value of number is: {number}");


    // loop {
    //     println!("again!");
    // }


    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");


    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");

}


// in we can return like this
fn five() -> i32 {
    5
}
