use std::fs::*;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    println!("Hello, world!");
    let f = File::open("hello.txt");//Nothing happens.
    //Returns enum Result<T,E>{
    //  OK(T),
    //  Err(E),   
    //}

    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind()
        {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file {:?}", e),
            },
            other_error => panic!("Can't open file {:?}",other_error),
        },
    };


    //The upper is equivelent with the below.
    //File::open("hello.txt")?;

    fn reader() -> Result<String, io::Error>
    {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s);
        Ok(s)
    }

    fn reader2() -> Result<String, io::Error>
    {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s);
        Ok(s)
    }

    {
        println!("\n<<Simple Linked List >>\n");

        /*
        enum List {
            Nil,
            Cons(i32, List)
        }
        */
        // Error : recursive type has infinite size

        #[derive(Debug)]
        enum List {
            Nil,
            Cons(i32, Box<List>)
        }
        //Box like (smart) pointer, Box uses heap, not stack.
        //So rust compiler doesn't have to track down the size of Box<List>

        impl List
        {
            fn mysingleton(num: i32) ->List 
            {
                List::Cons(num,Box::new(List::Nil))
            }

            fn push(self, num:i32) ->List
            {
                List::Cons(num, Box::new(self))
            }
        }

        
        let test = List::mysingleton(10);
        println!("{:?}", test);
        println!("{:?}",test.push(20).push(3).push(6).push(100));

    
    }

    {
        println!("\n<< Generic>>\n");

        fn largest<T>(list: &[T]) -> T
        {
            let mut largest = list[0];

            for &item in list.iter()
            {
                if item > largest{
                    largest = item;
                }
            }
            largest
        }

    }

}
