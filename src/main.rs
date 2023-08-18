trait Animal {
    type Output;
    fn name(&self) -> Self::Output;
    fn say_bye(&self) {
        println!("Bye");
    }
}

#[derive(Debug)]
struct Human {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Human {
    type Output = u32;
    fn name(&self) -> u32 {
        println!("Human name is {}", self.name);
        1
    }
}

impl Animal for Cat {
    type Output = u32;
    fn name(&self) -> u32 {
        println!("Cat name is {}", self.name);
        2
    }
}

fn say_hello<T: Animal>(animal: T) {
    animal.name();
}

fn new_animal(is_cat: bool) -> Box<dyn Animal<Output = u32>> {
    if is_cat {
        Box::new(Cat {
            name: "Tom".to_string(),
        })
    } else {
        Box::new(Human {
            name: "John".to_string(),
        })
    }
}

fn main() {
    let human = Human {
        name: "John".to_string(),
    };

    let v = vec![1, 2, 3];
    println!("v = {:?}", v);
    let mut v2 = Vec::new();
    v2.push(Box::new(human));
    let t1 = &v2[0];

    println!("t1 = {:?}", v2[0]);

    let v3 = vec![Box::new(Cat {
        name: "Tom".to_string(),
    })];

    // say_hello(cat);
    let a = new_animal(true).name();
    let b = new_animal(false).name();
    println!("a = {}, b = {}", a, b)
}
