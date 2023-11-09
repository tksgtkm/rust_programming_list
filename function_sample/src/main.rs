fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);

    // 関数定義
    more5(x);
    more5(2);

    structure_example();
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn more5(x: i32) {
    let number = x;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// enum Storage {
//     HDD { size: u32, rpm: u32},
//     SSD (u32),
// }

fn structure_example() {
    #[derive(Debug)]
    enum Storage {
        HDD { size: u32, rpm: u32},
        SSD (u32),
    }

    struct PCspec {
        cpus: u16,
        memory: u32,
        storage: Storage,
    }

    let spec = PCspec{
        cpus: 8,
        memory: 16,
        storage: Storage::SSD(1024),
    };

    println!("{}", spec.cpus);
    println!("{}", spec.memory);
    println!("{:#?}", spec.storage);
}