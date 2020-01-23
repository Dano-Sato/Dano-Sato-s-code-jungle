using System;
using System.Collections;
using System.Collections.Generic;
using System.ComponentModel;
using System.Numerics;
using System.Text;

namespace ConsoleTest1
{

    public static class Example1
    {
        public static Action Act = () =>
        {
            Console.WriteLine("Hello World!");

            //Nameof Method
            var Mystring = "String Contents";
            Console.WriteLine(nameof(Mystring));

            string greeting = "Hello!";
            Object mailMessageBody = greeting;

            Console.WriteLine(nameof(greeting));
            Console.WriteLine(nameof(mailMessageBody));

            var person = new Person();
            person.PropertyChanged += (s, e) => Console.WriteLine("Changed " + e.PropertyName);

            person.Address = "123 Fake Street";
            person.Address = "123 Fake Street2";
            person.Address = "123 Fake Street2";
            person.Address = "123 Fake Street1";
        };

        public class Person : INotifyPropertyChanged
        {
            private string _address;
            public event PropertyChangedEventHandler PropertyChanged;

            private void OnPropertyChanged(string propertyName)
            {
                PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
            }

            public string Address
            {
                get { return _address; }
                set
                {
                    if (_address != value)
                    {
                        _address = value;
                        OnPropertyChanged(nameof(Address));
                    }
                }
            }
        }

    }

    public static class Example2
    {
        [Flags]
        enum Colors
        { 
            Red = 1,
            Blue = 2,
            Green = 4,
            Yellow = 8        
        }

        enum ColorsWithoutFlag
        {
            Red = 1,
            Blue = 2,
            Green = 4,
            Yellow = 8
        }

        //The Flag Attribute changes how does the Console.WriteLine method work
        public static Action Act = () =>
          {
              var color = Colors.Red | Colors.Blue;
              var color2 = ColorsWithoutFlag.Red | ColorsWithoutFlag.Blue;

              Console.WriteLine(color.ToString());//Red, Blue
              Console.WriteLine(color2.ToString());//3(1+2)

          };
    }

    public static class Example3
    {
        //Tuple : Components-Combined Container without any granularity... Looks like old C's struct.
        //Maybe it could be used for simple data structure.
        //Supports sort based on tuple's element.


        public static Action Act = () =>
        {
            var tuple = new Tuple<string, int, bool>("foo", 123, true);
            var item1 = tuple.Item1;

            List<Tuple<int, string>> list = new List<Tuple<int, string>> ();
            list.Add(new Tuple<int, string>(2, "foo"));
            list.Add(new Tuple<int, string>(1, "bar"));
            list.Add(new Tuple<int, string>(2, "qux"));

            list.Sort((a, b) => a.Item2.CompareTo(b.Item2));// Sort based on TUples.Item2 (Sort by component precedence)

            foreach(var element in list)
            {
                Console.WriteLine(element);
            }

            Write();
        };

        //Return as tuple. Return multiple values using tuple

        public static void Write()
        {
            var result = AddMultiply(25, 28);
            Console.WriteLine(result.Item1);
            Console.WriteLine(result.Item2);
        }

        public static Tuple<int, int> AddMultiply(int a, int b)
        {
            return new Tuple<int, int>(a + b, a * b);
        }
    }

    public static class Example4
    {
        //BigInteger in System.Numerics has no limitation.
        public static Action Act = () =>
          {
              BigInteger l1 = 1;
              BigInteger l2 = 1;
              BigInteger current = l1 + l2;
              while (current.ToString().Length < 1000)
              {
                //Push memories backward to get next Fibonacci number.
                l2 = l1;
                  l1 = current;
                  current = l1 + l2;
              }
              Console.WriteLine(current);
          };
    }

    
    public static class Example5
    {
        public static Action Act = () =>
        {
            //These are bit cool ways to initialize Dictionary.
            var numberDictionary = new Dictionary<int, string>
            {
                {1,"One" },
                {2, "Two" },
                {3, "Three" },
            };

            var Dict = new Dictionary<int, string>
            {
                [1]="One",
                [2]="Two"
            };

            var People = new Dictionary<string, int>
            {
                {"John", 30 },
                {"Mary", 35 },
                {"Jack", 48 }
            };


            //Method with out keyword
            int age;
            if(People.TryGetValue("Mary",out age))
            {
                Console.WriteLine(age);
            }

        };    
    }

    public static class Example6
    {
        public static Action Act = () =>
        {
            string foo = "some string";
            var anon3 = new { foo.Length };
            var anon4 = new { Discription = foo.Length <= 10 ? "short string" : "long string" };

        };
    }




}
