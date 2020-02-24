fn main() {
    /* The function could not return the reference of the internal value.
    {
        fn test()->&String
        {
            let s=String::from("a");
            &s //Dangling pointer problem. could not return the reference of the internal value.
        }
    }*/

    //How to get Subarray//
    {
        let s = String::from("Hello world");
        let s1 = &s[0..5]; //get "Hello" from S as read-only reference(0~4)
                           //let s1=&s[0..=5];//get 0~5 elements
                           //let s1=&s[6..];//get 6~ elements
        println!("{}", s1);

        //problem : CJK(Chinese,Japanese, Korean) is not allowed. only uses ASCII-code.
    }

    //How to build struct//
    {
        #[derive(Debug)] //Debug setting
        struct Student {
            name: String,
            email: String,
            age: u32,
            undergraduate: bool,
        }
        let mut me = Student {
            name: String::from("Kim"),
            email: "abc@a".to_owned(),
            age: 23 as u32, //Type Casting
            undergraduate: false,
        };
        me.age = 32;
        me.undergraduate = true;
        let friend = Student {
            name: "abc".to_owned(),
            ..me //me is moved to friend
        };
        //println!("{:#?}",me);//me is dead.
        println!("{:#?}", friend);
    }

    //Tuple style structure//
    {
        #[derive(Debug)] //Debug setting
        struct Color(i32, i32, i32);

        let red = Color(255, 0, 0);
        println!("{}", red.0); //How to get elements of the struct
        println!("{}", red.1);
        println!("{:#?}", red);
    }

    //Implement struct//
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
            fn smaller(&self, other: &Rectangle) -> bool {
                self.width < other.width && self.height < other.height
            }
        }
        let rect1 = Rectangle {
            width: 30,
            height: 10,
        };
        let rect2 = Rectangle {
            width: 40,
            height: 20,
        };

        println!("Area is {:?}", rect1.area());
        println!("{:?}", rect1.smaller(&rect2));
    }

    //How to implement Built-in trait "Drop"
    {
        struct Person{name:String}
        //Built-in function drop.
        impl Drop for Person{
            fn drop(&mut self) {
                println!("Unji {}", self.name);
            }
        }

        {
            let mut a = Person{name: String::from("Muhyon")};
            {
                let mut b = Person{name: "Roh".to_owned()};
                {
                    let mut c = Person{name: "Owl".to_owned()};
                    //Drop function is called when the object is droped.
                }
            }
        }
    }

    {
        #[derive(Debug)]
        struct Fib{
            curr:u32,
            next:u32,
        }
        impl Iterator for Fib{
            type Item = u32;
            fn next(&mut self)->Option<u32>
            {
                let new_next = self.curr+self.next;
                self.curr=self.next;
                self.next=new_next;
                Some(self.curr)//Some is nullable operator                
            }
        }
        let mut fib=Fib{curr:1,next:1};
        println!("{:?}",fib.next());
        println!("{:?}",fib.next());
        println!("{:?}",fib.next());
        println!("{:?}",fib.next());
        println!("{:?}",fib.next());
        
    }

    {
        enum IpAddress
        {
            V4,
            V6,
        }

        struct Ip{
            kind:IpAddress,
            value:String,
        }

        let mut x=Ip{kind:IpAddress::V4,value:"127.0.0.1".to_owned()};
    }
    {
        enum SimpleIp{
            V4(String),
            V6(String),
        }

        let mut home = SimpleIp::V4("127.0.0.1".to_owned());
    }

    //Enum is superset of struct//
    {
        enum Message
        {
            End,
            Message(String),
            Color(i32,i32,i32),
            Fib{curr:u32,next:u32},
        }
    }

    //Some data type.
    {
        /*
        enum Option<T>
        {
            Some(T),
            None,
        }*/

        let num = Some(5);
        let unji= Some(String::from("Unji"));
        let k = Some(true);
        let n:Option<u32> = None;
        let n = Option::<u32>::None;
        
        println!("{:?}",num);
    }
    {
        enum Grade
        {
            A,
            B,
            C,
            D,
            F
        }
    
        let me = Grade::D;
        match me{
            Grade::A=>println!("Wuhung"),
            Grade::B=>println!("Nomuhyun"),
            Grade::C=>println!("AngGimoti?"),
            Grade::D=>println!("Rock of Owl"),
            Grade::F=>println!("Unji")
        }                   
        match me{
            Grade::A=>println!("Wuhung"),
            _=> println!("Unji"),
        }                   

    }

    {
        fn plus_one(x:Option<i32>)->Option<i32>
        {
            match x
            {
                Some(i)=>Some(i+1),
                None=>None
            }
        }

        println!("{:?}",plus_one(Some(5)));
    }

    {
        let pair = (2,-4);

        match pair
        {
            (a,b) if a==b => println!("A"),
            (a,b) if a+b==0 => println!("B"),
            (a,_) if a==7 => println!("7"),
            _=>println!("F")

        }
    }

    {
        fn age()->u32{
            15
        }
        match age()
        {
            0=>println!("body"),
            n@1..=12 =>println!("child"),
            n@13..=19 => println!("teen of {}",n),
            _=>println!("adult")
        }

    }

}
