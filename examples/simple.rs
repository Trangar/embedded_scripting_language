use scripting_lang::*;

fn main() {
    let mut pins = Vec::<Pin>::new();
    let mut total_delay_time = 0.0;

    {
        let mut engine = Engine::new(|name, args, stack, ast| {
            match name {
                "get_pin" => {
                    if let Some(name) = ast.get_return_variable_name(stack) {
                        let arg = args.next().trim_matches('"');
                        let pin = Pin::new(pins.len(), arg.to_owned());
                        pins.push(pin.clone());
                        stack.set(name, pin);
                    }
                }
                "set_high" => {
                    let pin: &Pin = stack.get(args.next());
                    pins[pin.index].high_count += 1;
                }
                "set_low" => {
                    let pin: &Pin = stack.get(args.next());
                    pins[pin.index].low_count += 1;
                }
                "delay" => {
                    let delay: f32 = args.next().parse().unwrap();
                    total_delay_time += delay;
                }
                _ => {
                    panic!("Unknown method call: {:?}", name)
                }
            }
        });

        let mut str = String::new();
        let mut file = std::fs::File::open("examples/simple.script").unwrap();
        use std::io::Read;
        file.read_to_string(&mut str).unwrap();
        drop(file);

        let mut eval_context = engine.start_eval(&str);

        let mut eval_options = EvalOptions::default();
        eval_options.cycles = 100;
        eval_context.execute(&eval_options);
    }

    println!("{} pins registered", pins.len());
    for pin in &pins {
        println!(
            "  {:?} was set high {} times and low {} times",
            pin.name, pin.high_count, pin.low_count
        );
    }
    println!("Total delay time: {} seconds", total_delay_time);
}

#[derive(Clone)]
struct Pin {
    pub name: String,
    pub index: usize,
    pub high_count: usize,
    pub low_count: usize,
}

impl Pin {
    pub fn new(index: usize, name: String) -> Self {
        Self {
            name,
            index,
            high_count: 0,
            low_count: 0,
        }
    }
}
