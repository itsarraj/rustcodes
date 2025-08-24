fn main() {
    // // let v = vec![1, 2, 3, 4, 5];
    //  //
    //  //   let does_not_exist = &v[100];
    //  //   let does_not_exist = v.get(100);
    //    let mut s1 = String::from("foo");
    //    // let s2 = "bar";
    //    let mut s2 = String::from("aaaaaa");
    //    s1.push_str(&s2);
    //    s2.push_str(&s1);
    //    println!("s2 is {s2}");
    //    println!("s1 is {s1}");
    //   // s2.push_str(s1);
    //    // println!("s2 is {s2}");
    //    // println!("s1 is {s1}");
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("s1 is {s1}");
    // println!("s2 is {s2}");
    // println!("s3 is {s3}");
    //
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{field_name},{field_value}");

    // let s = format!("{s1}-{s2}-{s3}");
    // println!("s is {s}");
}
