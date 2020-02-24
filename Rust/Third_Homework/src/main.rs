#[derive(Debug, Clone)]
pub enum BinaryTree
{
    Node
    {
        value: i32,
        leftTree: Box<BinaryTree>,
        rightTree: Box<BinaryTree>,
    },
    Empty,
}

impl BinaryTree{
    fn singleton(x: i32) ->BinaryTree{
        BinaryTree::Node{
            value: x,
            leftTree: Box::new(BinaryTree::Empty),
            rightTree: Box::new(BinaryTree::Empty),
        }
    }

    fn simple_print(&self) ->()
    {
        println!("{:?}",self);
    }

    fn find(&self, x: i32) ->Option<i32>
    {
        match self.value
        {
            x=>Some(x),
            x if x>self.value=>find(self.rightTree),
            x if <self.value=>find(self.leftTree),
        }

    }
    
}


fn main() {
    println!("Hello, world!");
}
