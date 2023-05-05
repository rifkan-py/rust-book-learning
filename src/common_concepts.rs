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
        println!("the value of a in inner scope is {a}")
    }
    // here value is just 11 since it does not know about what happened previous scope
    println!("the value of a in outer scope is {a}")

    /*
        This technique is called shadowing 
         1. This is different way of making variable mutate
         2. The variable be immutable after those transformations have been completed
    */

}