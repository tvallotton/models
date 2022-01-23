

trait Foo {
    fn foo(); 
}

impl<T: Copy> Foo for T {
    fn foo() {
        println!("is copy"); 
    }
}

impl<T: Drop> Foo for T {
    fn foo() {
        println!("is drop");
    }
}

fn main() {
    i32::foo(); 
    String::foo(); 
}