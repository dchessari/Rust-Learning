fn main() {
    // let v: Vec<i32> = Vec::new(); ---------- This is the same as line 3 using the "vec!" macro
    // let v = vec![1, 2, 3];

    // ! You can add to a Vector using the push function
    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // ! print third element in the vector
    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // println!("The third element is {third}");
    // let third: Option<&i32> = v.get(2);
    // match third {
        // Some(third) => println!("The third element is {third}"),
        // None => println!("There is no third element."),
    // }

    // ! Lists each on a seperate line with a for loop
    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}