use std::io::{stdin, stdout, BufRead, Write};

#[derive(Debug)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug)]
pub struct Beagle<'a> {
    pub name: &'a str,
    pub age: u8,
    pub sex: Sex,
    pub hunger_points: u8,
    pub max_hunger_points: u8,
}

impl<'a> Beagle<'a> {
    pub fn feed(&mut self, amount: u8) {
        if (self.hunger_points + amount) <= self.max_hunger_points {
            self.hunger_points += amount;
        } else {
            self.hunger_points = self.max_hunger_points;
        }
    }
}

#[derive(Debug)]
pub struct Pound<'a> {
    pub kennels: Vec<Beagle<'a>>,
}

impl<'a> Pound<'a> {
    pub fn new() -> Self {
        Self {
            kennels: Vec::new(),
        }
    }

    pub fn add(&mut self, beagle: Beagle<'a>) {
        self.kennels.push(beagle);
    }

    pub fn remove(&mut self, name: &str) -> Option<Beagle<'a>> {
        match self.find_kennel_by_name(name) {
            Some(index) => {
                let removed = self.kennels.remove(index);
                Some(removed)
            },
            None => None,
        }
    }

    pub fn feed_all(&mut self, amount: u8) {
        for dog in self.kennels.iter_mut() {
            dog.feed(amount);
        }
    }

    pub fn find_kennel_by_name(&self, name: &str) -> Option<usize> {
        self.kennels.iter().position(|a| a.name == name)
    }

    pub fn swap_kennels(&mut self, pos1: usize, pos2: usize) {
        self.kennels.swap(pos1, pos2);
    }
}

fn main() {
    let mut pound = Pound::new();

    println!("== Interactive Beagle Creator ==");
    print!("  => name: ");
    let name = readline();

    print!("  => age: ");
    let age = readline().parse::<u8>().unwrap();

    print!("  => sex: ");
    let sex = match readline().to_lowercase().as_str() {
        "male" => Sex::Male,
        "female" => Sex::Female,
        _ => panic!("Invalid sex. Must be one of: male, female"),
    };

    print!("  => current hunger: ");
    let hunger_points = readline().parse::<u8>().unwrap();

    print!("  => max hunger: ");
    let max_hunger_points = readline().parse::<u8>().unwrap();

    pound.add(Beagle {
        name: name.as_str(),
        age,
        sex,
        hunger_points,
        max_hunger_points,
    });

    pound.add(Beagle {
        name: "Dog1",
        age: 6,
        sex: Sex::Male,
        hunger_points: 10,
        max_hunger_points: 20,
    });

    pound.add(Beagle {
        name: "Dog2",
        age: 8,
        sex: Sex::Female,
        hunger_points: 8,
        max_hunger_points: 18,
    });

    pound.add(Beagle {
        name: "Dog3",
        age: 1,
        sex: Sex::Male,
        hunger_points: 2,
        max_hunger_points: 8,
    });

    println!("{:#?}", pound);

    pound.remove("Dog2");

    pound.feed_all(3);
    println!("{:#?}", pound);

    pound.swap_kennels(
        pound.find_kennel_by_name("Dog1").unwrap(),
        pound.find_kennel_by_name("Dog3").unwrap(),
    );

    println!("{:#?}", &pound);
}

fn readline() -> String {
    let mut buf = String::new();
    let mut handle = stdin().lock();

    stdout().flush().expect("Failed to flush stdout");
    handle
        .read_line(&mut buf)
        .expect("Failed to read from stdin");

    buf.trim().into()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn feed_dog() {
        let mut b = Beagle {
            name: "Ben",
            sex: Sex::Male,
            age: 18,
            hunger_points: 4,
            max_hunger_points: 5,
        };

        assert_eq!(b.hunger_points, 4);
        b.feed(1);
        assert_eq!(b.hunger_points, 5);
    }

    #[test]
    fn add_to_pound() {
        let mut pound = Pound::new();
        assert_eq!(pound.kennels.len(), 0);
        pound.add(Beagle {
            name: "Ben",
            sex: Sex::Male,
            age: 18,
            hunger_points: 4,
            max_hunger_points: 5,
        });
        assert_eq!(pound.kennels.len(), 1);
    }

    #[test]
    fn remove_from_pound() {
        let mut pound = Pound {
            kennels: vec![Beagle {
                name: "Ben",
                sex: Sex::Male,
                age: 18,
                hunger_points: 4,
                max_hunger_points: 5,
            }],
        };

        assert_eq!(pound.kennels.len(), 1);
        pound.remove("Ben");
        assert_eq!(pound.kennels.len(), 0);
    }

    #[test]
    fn swap_kennels() {
        let mut pound = Pound {
            kennels: vec![
                Beagle {
                    name: "Dog1",
                    age: 6,
                    sex: Sex::Male,
                    hunger_points: 10,
                    max_hunger_points: 20,
                },
                Beagle {
                    name: "Dog2",
                    age: 8,
                    sex: Sex::Female,
                    hunger_points: 8,
                    max_hunger_points: 18,
                },
                Beagle {
                    name: "Dog3",
                    age: 1,
                    sex: Sex::Male,
                    hunger_points: 2,
                    max_hunger_points: 8,
                },
            ],
        };

        assert_eq!(pound.find_kennel_by_name("Dog1"), Some(0));
        assert_eq!(pound.find_kennel_by_name("Dog2"), Some(1));
        assert_eq!(pound.find_kennel_by_name("Dog3"), Some(2));
        pound.swap_kennels(
            pound.find_kennel_by_name("Dog1").unwrap(),
            pound.find_kennel_by_name("Dog3").unwrap(),
        );
        assert_eq!(pound.find_kennel_by_name("Dog1"), Some(2));
        assert_eq!(pound.find_kennel_by_name("Dog2"), Some(1));
        assert_eq!(pound.find_kennel_by_name("Dog3"), Some(0));
    }

    #[test]
    fn feed_all_dogs() {
        let mut pound = Pound {
            kennels: vec![
                Beagle {
                    name: "Dog1",
                    age: 6,
                    sex: Sex::Male,
                    hunger_points: 10,
                    max_hunger_points: 20,
                },
                Beagle {
                    name: "Dog2",
                    age: 8,
                    sex: Sex::Female,
                    hunger_points: 8,
                    max_hunger_points: 18,
                },
                Beagle {
                    name: "Dog3",
                    age: 1,
                    sex: Sex::Male,
                    hunger_points: 2,
                    max_hunger_points: 8,
                },
            ],
        };

        assert_eq!(pound.kennels[0].hunger_points, 10);
        assert_eq!(pound.kennels[1].hunger_points, 8);
        assert_eq!(pound.kennels[2].hunger_points, 2);
        pound.feed_all(8);
        assert_eq!(pound.kennels[0].hunger_points, 18);
        assert_eq!(pound.kennels[1].hunger_points, 16);
        assert_eq!(pound.kennels[2].hunger_points, 8);
    }
}
