trait Animal {
    fn name(&self);
}

struct Human {
    name:String
}

struct Cat {
    name:String
}

impl Animal for Human {
    fn name(&self) {
        println!("Human name is {}", self.name);
    }
}

impl Animal for Cat {
    fn name(&self) {
        println!("Cat name is {}", self.name);
    }
}

fn say_hello<T: Animal>(animal: T) {
    animal.name();
}
    

fn main() {

    let human = Human { name: "John".to_string() };
    let cat = Cat { name: "Tom".to_string() };

    say_hello(human);
    say_hello(cat);
}