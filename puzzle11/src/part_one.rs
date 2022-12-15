#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: fn(v: usize) -> usize,
    divisible_by: usize,
    test: [usize; 2],
    inspection_count: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: fn(v: usize) -> usize,
        divisible_by: usize,
        test: [usize; 2],
    ) -> Monkey {
        Monkey {
            items,
            operation,
            divisible_by,
            test,
            inspection_count: 0,
        }
    }
    fn receive_item(&mut self, item: usize) {
        self.items.push(item);
    }
    fn get_item(&mut self) -> usize {
        self.items.remove(0)
    }
    fn get_worry_level(&self, current_item: usize) -> usize {
        let before_relieve: usize = (self.operation)(current_item);
        before_relieve / 3
    }
    fn inspect(&mut self) -> (usize, usize) {
        if self.items.len() == 0 {
            // out of items, we're done
            panic!("No items!")
        }

        self.inspection_count += 1;

        let current_item = self.get_item();
        let worry_level = self.get_worry_level(current_item);

        let target_monkey = if worry_level % self.divisible_by == 0 {
            self.test[0]
        } else {
            self.test[1]
        };

        (target_monkey, worry_level)
    }
}

fn initialize_test() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    monkeys.push(Monkey::new(vec![79, 98], |v: usize| v * 19, 23, [2, 3]));
    monkeys.push(Monkey::new(
        vec![54, 65, 75, 74],
        |v: usize| v + 6,
        19,
        [2, 0],
    ));
    monkeys.push(Monkey::new(vec![79, 60, 97], |v: usize| v * v, 13, [1, 3]));
    monkeys.push(Monkey::new(vec![74], |v: usize| v + 3, 17, [0, 1]));
    monkeys
}

fn initialize() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    monkeys.push(Monkey::new(
        vec![97, 81, 57, 57, 91, 61],
        |v: usize| v * 7,
        11,
        [5, 6],
    ));
    monkeys.push(Monkey::new(
        vec![88, 62, 68, 90],
        |v: usize| v * 17,
        19,
        [4, 2],
    ));
    monkeys.push(Monkey::new(vec![74, 87], |v: usize| v + 2, 5, [7, 4]));
    monkeys.push(Monkey::new(
        vec![53, 81, 60, 87, 90, 99, 75],
        |v: usize| v + 1,
        2,
        [2, 1],
    ));
    monkeys.push(Monkey::new(vec![57], |v: usize| v + 6, 13, [7, 0]));
    monkeys.push(Monkey::new(
        vec![54, 84, 91, 55, 59, 72, 75, 70],
        |v: usize| v * v,
        7,
        [6, 3],
    ));
    monkeys.push(Monkey::new(
        vec![95, 79, 79, 68, 78],
        |v: usize| v + 3,
        3,
        [1, 3],
    ));
    monkeys.push(Monkey::new(vec![61, 97, 67], |v: usize| v + 4, 17, [0, 5]));

    monkeys
}

fn play_round(monkeys: &mut Vec<Monkey>, counter: &mut usize) {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            let (target_monkey, current_item) = monkeys[i].inspect();
            monkeys[target_monkey].receive_item(current_item);
        }
    }

    *counter += 1;
}

fn main() {
    let mut monkeys = initialize();
    let mut counter = 1;

    for _ in 0..20 {
        play_round(&mut monkeys, &mut counter);
    }

    let mut inspection_count_list: Vec<usize> =
        monkeys.into_iter().map(|m| m.inspection_count).collect();

    inspection_count_list.sort();
    inspection_count_list.reverse();

    let res = inspection_count_list[0] * inspection_count_list[1];

    println!("Monkey bussinness {res}");
}
