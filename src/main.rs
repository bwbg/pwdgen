// ---------------------------------------------------------------------
// Copyright 2021 Heiko Möllerke
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files
// (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
// ---------------------------------------------------------------------

/// Provides the application's configuration.
mod config {
    pub const APP_NAME: &str = "PasswordGenerator";
    pub const APP_DESCRIPTION: &str = "bulk generation of pseudo random passwords";
    pub const APP_VERSION: &str = "0.1.1";
    pub const APP_AUTHOR: &str = "Heiko Möllerke";
}

/// Provides the application's core-functionalities.
mod core {
    use rand::Rng;
    use rand::seq::SliceRandom;

    /// An alphabet is a collection of symbols which will be used to
    /// create a (pseudo)-random sequence (aka password).
    pub struct Alphabet {
        symbols: Vec<char>,
    }

    impl Alphabet {
        /// Creates a new Alphabet-object with the given symbols.
        pub fn new(symbols: &str) -> Self {
            Alphabet { symbols: symbols.chars().collect() }
        }

        /// Returns the stored symbols-sequence.
        pub fn get(&self) -> &Vec<char> {
            &self.symbols
        }

        /// Returns the alphabet's length (number of symbols).
        pub fn len(&self) -> usize {
            self.symbols.len()
        }

        /// Returns the n-th symbol from this alphabet. This method
        /// panics if `n` exceeds the number of available symbols.
        pub fn nth(&self, n: usize) -> char {
            self.symbols[n]
        }
    }

    /// Produces (pseudo-) random passwords.
    pub struct PasswordFactory {
        length: usize,
        alphabets: Vec<Alphabet>,
    }

    impl PasswordFactory {
        /// Creates a new PasswordFactory-object with the given settings.
        pub fn new(length: usize, alphabets: Vec<Alphabet>) -> Self {
            Self { length, alphabets }
        }

        /// Returns one random symbol from each alphabet.
        fn random_from_each_alphabet(&self) -> Vec<char> {
            let mut rng = rand::thread_rng();
            let mut symbols = Vec::new();

            for abc in self.alphabets.iter() {
                let n = rng.gen_range(0..abc.len());
                symbols.push(abc.nth(n));
            }

            symbols
        }

        /// Returns a vector containing all symbols from each alphabet.
        fn collect_all_symbols(&self) -> Vec<char> {
            let mut symbols = Vec::new();
            for abc in self.alphabets.iter() {
                symbols.extend(abc.get());
            }
            symbols
        }

        /// Returns `length` random symbols out of any alphabet.
        fn random_from_any_alphabet(&self, length: usize) -> Vec<char> {
            let mut rng = rand::thread_rng();
            let all_symbols = self.collect_all_symbols();
            let mut symbols = Vec::new();

            for _ in 0..length {
                let max = all_symbols.len();
                symbols.push(all_symbols[rng.gen_range(0..max)]);
            }

            symbols
        }

        /// Creates a random password with the factory's settings.
        pub fn produce(&self) -> String {
            let mut rng = rand::thread_rng();
            let mut symbols = Vec::new();

            symbols.extend(self.random_from_each_alphabet());
            symbols.extend(self.random_from_any_alphabet(self.length - symbols.len()));
            symbols.shuffle(&mut rng);

            symbols.into_iter().collect()  // Type inference magic
        }
    }
}

/// Provides the functionalities of the application's
/// command-line-interface (cli).
mod cli {
    use crate::core::{Alphabet, PasswordFactory};
    use crate::config::*;
    use clap::{App, Arg};

    /// Run's the application's command-line-interface (aka the app).
    pub fn run() {
        // Use clap for parsing the command line and after successful
        // matching extract the parameters into distinct slice-objects:
        let matches = App::new(APP_NAME)
            .version(APP_VERSION)
            .author(APP_AUTHOR)
            .about(APP_DESCRIPTION)
            .arg(Arg::with_name("ALPHABET")
                .short("a")
                .long("abc")
                .multiple(true)
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("LENGTH")
                .short("l")
                .long("length")
                .takes_value(true))
            .arg(Arg::with_name("NUMBER")
                .short("n")
                .long("number")
                .takes_value(true))
            .get_matches();

        let length = matches.value_of("LENGTH").unwrap_or("8");
        let number = matches.value_of("NUMBER").unwrap_or("1");
        let alphabets: Vec<&str> = matches
            .values_of("ALPHABET")
            .unwrap()
            .collect();

        // After successful parsing and extraction parse the slice
        // parameters into numbers and required objects (shadows):
        let length: usize = length.trim().parse().unwrap_or(8);
        let number: usize = number.trim().parse().unwrap_or(1);
        let alphabets: Vec<Alphabet> = alphabets
            .iter()
            .map(|&s| Alphabet::new(s))
            .collect();

        // Create password-factory and output passwords.
        let factory = PasswordFactory::new(length, alphabets);
        for _ in 0..number {
            println!("{}", factory.produce());
        }
    }
}

/// Application's entry-point.
fn main() {
    crate::cli::run()
}
