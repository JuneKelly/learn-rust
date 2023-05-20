use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("Note: {}", note);
                if self.age < 21 {
                    println!("(Do not serve alcohol to {})", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name)
        }
    }
}

fn main() {
    let mut visitor_list: Vec<Visitor> = vec![
        Visitor::new("June", VisitorAction::Accept, 35),
        Visitor::new("Lorraine", VisitorAction::Accept, 38),
        Visitor::new(
            "Ruby",
            VisitorAction::AcceptWithNote {
                note: String::from("She's little and mighty"),
            },
            5,
        ),
        Visitor::new("Marni", VisitorAction::Accept, 30),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor: Option<&Visitor> =
            visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    let new_visitor = Visitor::new(&name, VisitorAction::Probation, 18);
                    new_visitor.greet_visitor();
                    visitor_list.push(new_visitor);
                }
            }
        }
    }
    println!("The final list of visitors is...");
    println!("{:#?}", visitor_list);
}
