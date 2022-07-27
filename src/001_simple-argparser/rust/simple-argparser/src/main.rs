use std::{collections::HashMap, env};

struct Parameter {
    options: String,
    filename: String,
}

impl Parameter {
    // Main code logic is found here.
    // 1. Push the arguments into a HashMap. `zip()` is used to combine two iterators
    //    `env::args()` and `env::args().skip(1)`. If you call the program with arguments
    //    `-O xyz -F abc`, then the HashMap is constructed from
    //    `[("-O", "xyz"), ("xyz", "-F"), ("-F", "abc")]`.
    // 2. Use turbofish syntax to collect into an explicit type HashMap<String, String>.
    // 3. `args.get("-O")` will return an Option<&String>, in this case, `Some(&"xyz")`.
    //    A reference to the string "xyz".
    // 4. Convert the Option<&String> to Result<&String> with `ok_or_else`.
    fn new() -> Result<Parameter, String> {
        let args = env::args()
            .zip(env::args().skip(1))
            .collect::<HashMap<String, String>>();

        let options: String = args.get("-O")
            .ok_or_else(|| String::from("Options not specified"))?
            .to_owned();

        let filename: String = args.get("-F")
            .ok_or_else(|| String::from("Filename not specified"))?
            .to_owned();

        Ok(Parameter { options, filename })
    }
}

fn main() {
    let p = Parameter::new();
    let pv: Parameter = match p {
        Ok(p) => p,
        Err(error) => {
            println!("{}", error);
            return
        }
    };

    println!("Option {}\nFile name: {}", pv.options, pv.filename);
}
