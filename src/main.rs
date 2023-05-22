use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>,
}

// insertメソッドをTodo構造体に定義
impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content);
        std::fs::write("./data/db.txt", content)
    }

    fn new() -> Result<Todo, std::io::Error> {
        let mut list = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("./data/db.txt")?;
        let mut content = String::new();
        list.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines()  // 文字列の各行に対するイテレータを作成
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(Todo {map})
    }
}

fn main() {

    let action = std::env::args().nth(1).unwrap_or_else(|| {
        panic!("define an action");
    });

    let item = std::env::args().nth(2).unwrap_or_else(|| {
        panic!("define an item");
    });

    println!("{:?}, {:?}", action, item);

    let mut todo = Todo {
        map: HashMap::new(),
    };

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }

}
