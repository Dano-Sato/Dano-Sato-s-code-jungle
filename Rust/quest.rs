fn main()
{
    {
        enum SimCal 
        {
            Cal(i32, i32, char)
        }
        
        impl SimCal
        {
            fn Calculate(x:SimCal) -> i32
            {
                match x
                {
                    SimCal::Cal(a,b,c) =>
                    {
                        match c
                        {
                            '+' => a+b,
                            '-' => a-b,
                            '*' => a*b,
                            '/' => a/b,
                            _=> -1
                        }
                    }
                }
            }
        }
        
        println!("{}",SimCal::Calculate(SimCal::Cal(3,4,'+')));
        println!("{}",SimCal::Calculate(SimCal::Cal(10,4,'-')));
        println!("{}",SimCal::Calculate(SimCal::Cal(3,10,'*')));
        println!("{}",SimCal::Calculate(SimCal::Cal(12,3,'/')));
        println!("{}",SimCal::Calculate(SimCal::Cal(12,3,'.')));

    }

}

