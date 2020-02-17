#![allow(unused_mut, unused_variables, dead_code,non_snake_case)]//Get rids of warning signs

fn main()
{
    let x=5;//define constant(=immutable) with "let" keyword
    let x=3;
    let x=6;//Shadowing.
    let x:i32//type i8 uses 8bit(0~255),Explicit type definition.
    =3;

    println!("{}",x);     
    //x=6; %% x is immutable. if you want to use mutable value, define x as mut. "let mut x"

    let some = "Sex";
    let some = some.len();//Shadowing    
    println!("{}",some);
    let guess:u32//Too complicated case. Requires explicit type. u32=uint32.
    ="42 ".trim().parse().expect("Sex");
    println!("{}",guess);

    let f:f64=2.0;
    let y:bool=true;
    let z:char = 'a';

    //Test tuple
    let t=(3.0,true,7);
    let samet:(f64,bool,i64)=(3.0,true,7);
    println!("{}",t.0);
    println!("{}",t.1);
    println!("{}",t.2);

    let L=[1,2,3,4];
    println!("{}",L[0]);

    let x = { 
                let mut t=0;
                for i in 0..10{
                    t=t+1;
                }
                t//(return) t
            };
    println!("{}",x);

    let num=3;
    let b:bool =
    {
        if num>5 {true}
        else {false}
        //return type must be same in every case.
    };
    println!("{}",b);

    //Function test
    fn test()
    {
        println!("hellow");
    }
    test();

    fn test2(x:i32)
    {
        println!("hellow {}",x);
    }
    test2(3);

    fn add(x:i32, y:i32)->i32
    {
        x+y
    }
    println!("{}",add(3,4));

    let s = String::from("aa");
    let t = s;//ownership is transfered to t.
    println!("{}",t);

    fn leng(s:String)->usize
    {
        s.len()
    }
    let s=String::from("kk");
    leng(s);//Function eats up the value "s"!
    println!("{}",s);

    //Refer the value "s". but can't change the value.(Read-only)
    fn leng(s:&String)->usize
    {
        s.len()
    }
    let s=String::from("kk");
    leng(&s);
    println!("{}",s);


    //Reference, but not read-only
    fn plus(s: &mut String)
    {
        s.push_str("Don't buy iPad pro");//Changes the value of s
    }

    let mut s=String::from("a,");
    plus(&mut s);
    println!("{}",s);




}