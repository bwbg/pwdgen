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

/// Produces a single password from multiple alphabets.
pub fn produce_password(length: usize, alphabets: &Vec<Alphabet>) -> String {
    if length < alphabets.len() {
        panic!("Password-length must be greater or equal the number of alphabets.");
    }

    if alphabets.len() < 1 {
        panic!("At least one alphabet must be given to create a password!")
    }

    let mut rng = rand::thread_rng();
    let mut password = Vec::new();

    // Randomly pick a symbol from each alphabet. This ensures at least
    // one symbol of each alphabt will be present in the generated
    // password.
    for abc in alphabets {
        let i = rng.gen_range(0..abc.len());
        password.push(abc.nth(i));
    }

    // Merge symbols of each alphabet into a single collection ...
    let mut symbols = Vec::new();
    for abc in alphabets {
        symbols.extend(abc.get());
    }

    // ... the randomly push symbols to the password until the required
    // length is reached:
    while password.len() < length {
        let i = rng.gen_range(0..symbols.len());
        password.push(symbols[i]);
    }

    // Shuffle the password to randomly distribute the initial symbols:
    password.shuffle(&mut rng);
    password.iter().collect() // Type inference magic
}
