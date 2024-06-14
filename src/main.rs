#![allow(unused)]

pub enum SizeDesc
    {
        Small,
        Medium,
        Large
    }

    pub struct Item
    {
        pub id: u16,
        pub quantity: u32,
        pub size: SizeDesc 
    }

fn main() 
{
    println!("Hello, World!");

    let mut test_item = Item
    {
        id: 256,
        quantity: 56,
        size: SizeDesc::Small
    };

    println!("{}",test_item.id);
}