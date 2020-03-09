fn main() {
    //RAII
    {
        //stack allocate
        let a: u32 = 3;
        println!("{}",a);
    }

    {
        //heap allocate
        //similar with malloc in C
        let a = Box::new(3u32);//3(u32)
        println!("{}",a);
    }

    {
        let a = 5u32;
        let b = a;
        println!("{}-{}", a, b);

        let x = Box::new(12u32);
        let y = x;      
        //println!("{}-{}",x,y);  
    }

    // String::from("as")
    // String::new()
    // "asd".to_owned();
    // String -> &str, length, capacity

    //Vector
    {
        println!("\n<< Vector >>\n");
        
        let mut v1: Vec<i32> = Vec::new();
        println!("{:?}", &v1);//Don't forget to write '&'!
        v1.push(1);
        v1.push(2);
        println!("{:?}",&v1);


        //using MACRO
        let v2: Vec<i32> = vec![1,2,3,4,5];
        println!("{:?}",&v2);

        println!("{:?}", &v2[2]);
        // call v2[999] -> SegFault
        // call v2.get(999) -> returns None(Nullable method)
    
        match &v2.get(2)
        {
            Some(third) => println!("The third element is 3"),
            None => println!("There is no third element."),
        }

        for i in &v2 {
            print!("{} ", i);
        }
    }

    {
        println!("\n<< STRING >>\n");
        // String =/= &str
        // String is kind of 'struct' contains &str, length, capacity

        //String support UTF-8


        ////How to use Concat
        //let mut s3 = s1 + " " + &s2 + &s4 + &s7;
        //s3.push_str("너는 내것");
    }

    {
        println!("\n<< HASHMAP >>\n");
        
        // Fast at finding
        // Slow for Saving
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"),10);
        scores.insert(String::from("Red"),15);
        //Error//scores.insert(25, 33);

        println!("{:?}", scores);


        // Table Mixing
        let teams = vec![String::from("Blue"), String::from("Red")];
        let scores = vec![10,15];

        let scores: HashMap<_,_> = teams.iter().zip(scores.iter()).collect();
        println!(">>> {:?}",scores);
        let scores2 = teams.iter().zip(scores.iter());
        println!("{:?}",scores2);
        //.collect() do Unwraping zip and make b to 'something'
        // [1,2,3] zip [4,5,6] == [(1,4),(2,5),(3,6)] -->List to HashMap 


        let mut new_score = HashMap::new();        
        new_score.insert(String::from("Blue"),10);

        new_score.entry(String::from("Yellow")).or_insert(50);

        let text = "hello my world and hello my scholl and no hello Zzang";

        let mut map = HashMap::new();
        for word in text.split_whitespace()
        {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}",map);

        let a1 = "ab bs ds";

        println!("{:?}", a1.split_whitespace());
    }

    {
        //Segfault vs panic!
        //different with Segfault, you could find why the panic! happened.
        let vec = vec![1,2,3];
        //lets cause panic!//println!("{:?}", vec[99]);

        // Bounds-check elimination ; Boundary check sacrifices the speed of compile.

        fn give_gift(gift: String)
        {
            if gift == "moon".to_owned() {panic!("Fuck you")}
            println!("Oh, thanks you!");
        }

        give_gift(String::from("moon"));

        // Type of panic! is '!', that means could be anything. It could be throwed at anywhere.

        fn div_number(divd: i32, divby: i32)->i32
        {
            if divby == 0 {panic!("Divide by zero")};
            divd / divby
        }
    }


}
