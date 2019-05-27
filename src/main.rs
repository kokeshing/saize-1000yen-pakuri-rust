use std::io::{BufRead, BufReader};
use std::fs::File;

const MAX_YEN : usize = 1000;

#[derive(Clone)]
struct Item {
    id: usize,
    name: String,
    price: usize,
    calorie: usize,
}

fn solv_impl(menu : &Vec<Item>, max : usize, index : usize) -> (usize, Vec<Item>) {
    if index >= menu.len() {
        return (0, vec![]);
    }

    let not_used = solv_impl(&menu, max, index + 1);
    let price = menu[index].price;
    if price <= max {
        let mut used = solv_impl(&menu , max - price, index + 1);
        let used_calorie = used.0 + menu[index].calorie;
        if used_calorie > not_used.0 {
            used.0 = used_calorie;
            used.1.push(menu[index].clone());

            return used;
        }
    }

    return not_used;
}

fn main() {
    let f = File::open("./menu.csv").expect("Faild open file");
    let f = BufReader::new(f);

    let mut menu : Vec<Item> = Vec::new();
    for line in f.lines().filter_map(|result| result.ok()).skip(1) {
        let row: Vec<String> = line.split(",").map(|e| e.parse().ok().unwrap()).collect();
        let i : usize = row[0].parse().unwrap();
        let n : String = row[1].to_owned();
        let p : usize = row[4].parse().unwrap();
        let c : usize = row[5].parse().unwrap();
        menu.push(Item {id: i, name: n, price: p, calorie: c});
    }

    let result = solv_impl(&menu, MAX_YEN, 0);

    for item in result.1 {
        println!("Name: {} Price: {} Calorie: {}", item.name, item.price, item.calorie);
    }
}
