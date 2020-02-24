
//How to use Cargo

/*
extern crate ansi_term;

use ansi_term::Colour::*;

fn main() {
    println!("{}",RGB(12, 200, 30).paint("a colored string"));
}*/

//mod means module
//mod could be nested

mod learning_programming_language{
    pub mod self_study{
        pub fn buying_books() {}//if the function is public, it could be refered in the crate.
        fn get_online_course() {}
    }
    mod learn_with_teachers {
        fn go_to_class() {}
        fn take_a_seat() {}
        fn do_homework() {}
    }
}

fn im_in_library()
{
    //crate is whole universe
    //Absolute path
    crate::learning_programming_language::self_study::buying_books();
    //Relative path
    learning_programming_language::self_study::buying_books();
}

//current position : crate
mod break_time {
    fn listening_to_music() {}

    fn go_to_library() {
        // super -> parent module (Like cd ../)
        super::im_in_library();
        listening_to_music();
    }

    #[derive(Debug)]
    pub struct Music {
        pub rockstyle: String,
        pub citypop: String,
        nedasship: String,
    }
    impl Music {
        pub fn what_music_you_hear(&self) {
            println!("I'm hearing rockstyle music named {}", self.rockstyle);
            println!("And I'm hearing citypop music named {}", self.citypop);
        }
        // after music_start function
        pub fn youre_music(x: String) -> Music {
            Music {
                rockstyle: String::from("rock"),
                citypop: String::from("city"),
                nedasship: String::from(x),
            }
        }
    }

    

    // enum is all public
}

pub fn music_start() {
    
    /*
    let me = crate::break_time::Music {
        rockstyle: "Rock".to_owned(),
        citypop: "City".to_owned(),
        nedasship: "nedasship".to_owned(),
    };*/
    
    // ! Warning, cause nedasship is private

    let mut me = break_time::Music::youre_music(String::from("Bad Apple"));
    me.rockstyle = String::from("Queen");
    println!("{:?}", me);
}

fn main() {
    {
        println!("Hello, world!");
        music_start();
    }

    // use
    // like namespace
    {
        crate::learning_programming_language::self_study::buying_books();
    }
    {
        use crate::learning_programming_language::self_study;
        self_study::buying_books();
    }
    {
        use crate::learning_programming_language::self_study::*;
        buying_books();
    }
    {
        //* : all  
        let mut map = std::collections::HashMap::new();
        map.insert(1, "abs");
        println!("{:?}", map);
    }
    {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(1, 2);
        println!("{:?}", map);
    }
    {
        use std::fmt;
        use std::io;
        fn func2() ->io::Result<()>{
            Ok(())
        }
    }
    {
        use std::io::Result as IOResult;
        fn func2() ->IOResult<()> {
            Ok(())
        }
    }
}

mod my_mod {
    fn private_func() {}
    pub fn func() {}
    pub fn indirect_access() {
        private_func();
        func();
    }

    pub mod nested {
        pub fn func() {}
        // Only available in current crate
        pub(self) fn func_in_nested() {}
        // Only avaliable in given path
        pub(in crate::my_mod) fn func_in_my_mode() {
            func_in_nested();
        }
        // Only available in parent module
        pub(super) fn func_in_super() {}
    }
    pub fn call_nested_func() {
        //func_in_my_mode();
        //func_in_super();
    }
    // Make avaliable only within current crate
    pub(crate) fn func_in_crate() {}

    mod private_nested {
        // Both can't called by other crate even it's pub and crated
        pub fn func_in_pri_nested() {}
        pub(crate) fn restricted_func() {}
    }
}