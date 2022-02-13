fn main() {
    let mut x = 4;
    println!("x = {}",x);
    x = 5;
    println!("x = {}",x);
    
    //conditional statements
    let y = true;
    if y {
        println!("true");
    }
    

    //tuples and destructuring
    let z = (1,"hello",3.14);
    println!("second value is: {}",z.2);
    let (val_1, val_2, _) = z;
    println!("the two values are {} and {}", val_1, val_2);

    //arrays
    let mut a = [1,2,3,4];
    println!("the first value is {}",a[0]);
    a[1] = 3; //changes value from 2 to 3
    let b: [i32;10] = [0;10];
    println!("the array is {:?}",b);

    //control flow
    let num = 2;
    if num == 1 {
        println!("You won")
    }else if num == 2 {
        println!("Try again")
    }else {
        println!("You lost")
    }

    //loop construct
    let mut p = 1;
    loop {
        p = p*2;

        if p>5000 {
            break;
        }

        println!("value of p is {}",p);
    }

    //while loop
    let mut q = 1;
    while q<5000 {
        q = q*2;
        println!("the value of q is {}",q);
    }

    //for loop
    let mut r:i32;
    for r in 1..10 { //1-9 only, 10 exclusive
        println!("value of r is {}", r);
    }
    for r in 1..=10 { //1-9, 9 inclusive
        println!("value of r is {}", r);
    }

    let s = [1,2,3];
    for val in s {
        println!("array elements are {}", val);
    }

    //match statements
    let t = 1;
    match t {
        1 => println!("value of t is 1"),
        2 => println!("value of t is 2"),
        _ => println!("value of t is invalid"),
    }

    let u = true;
    let v = true;
    match (u, v) {
        (true, true) => println!("u and v are true, true"),
        (true, false) => println!("u and v are true, false"),
        (false, true) => println!("u and v are false, true"),
        _ => println!("u and v are false, false"),
    }


}
