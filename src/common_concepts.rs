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

}