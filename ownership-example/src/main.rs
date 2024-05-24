fn main() {
    // Primitive types: i32
    let x = 5; // x owns the value 5

    let y = x; // y now owns the value 5, x and y are independent copies
    println!("x: {}, y: {}", x, y);

    // &str is a primitive type that is a reference to a str (not String)
    let str1 = "hello"; // str1 owns the string data "hello"

    // Non-primitive type: String
    let s1 = String::from(str1); // s1 owns the string data "hello"

    let s2 = s1; // s2 now owns the string data "hello", s1 is invalidated
                 //println!("s1: {}", s1); // This will cause a compile error because s1 is invalidated
    println!("s2: {}", s2);

    // However, we can clone the string to create a new owner
    let s3 = s2.clone(); // s3 now owns a new copy of the string data "hello"
    println!("s2: {}, s3: {}", s2, s3);

    // Non-primitive type: Vec<i32>
    let v1 = vec![1, 2, 3]; // v1 owns the vector [1, 2, 3]

    let v2 = v1; // v2 now owns the vector [1, 2, 3], v1 is invalidated
                 //println!("v1: {:?}", v1); // This will cause a compile error because v1 is invalidated
    println!("v2: {:?}", v2);

    // Similarly, we can clone the vector to create a new owner
    let v3 = v2.clone(); // v3 now owns a new copy of the vector [1, 2, 3]
    println!("v2: {:?}, v3: {:?}", v2, v3);
}
