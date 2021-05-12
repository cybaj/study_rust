use std::io;

fn main() {

    const MAX_LENGTH : u32 = 20;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let length : u32 = guess.len() as u32;
        let guess: String = match guess
            .trim()
            .parse::<String>() {
                Ok(num) => num.to_string(),
                Err(_) => {
                    println!("You must type numbers!");
                    continue
                },
            };

        if length.gt(&MAX_LENGTH) {
            println!("You must type number smaller than {}.", MAX_LENGTH);
            continue
        }

        println!("{:0>20}", guess.display().trim());
    }
}

trait FormatThousand {
    fn display(&self)->String;
}

impl FormatThousand for String {
    fn display(&self)->String {
        let mut res = vec![];
        let len : u32 = self.len() as u32;
        let mut reversed = self.chars().rev();

        for i in 0..len {
            match reversed.next() {
                Some(value) => {
                    if i%3 == 0 && 0 < i && i < len - 1 {
                        res.push(format!(",{}", value));
                    } else {
                        res.push(format!("{}", value));
                    }
                },
                None=>{}
            }
        }
        return res
            .into_iter()
            .rev()
            .collect::<std::vec::Vec<std::string::String>>()
            .join("");
    }
}
