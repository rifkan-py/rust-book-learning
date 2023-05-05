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

}