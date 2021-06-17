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

pub mod core;

// ---------------------------------------------------------------------
// Unit testing
// ---------------------------------------------------------------------

#[cfg(test)]
mod test {
    /// Provides tests for the core-module
    mod core {
        use crate::core::{Alphabet, produce_password};

        #[test]
        fn alphabet_len() {
            let abc = Alphabet::new("ABCD");
            assert_eq!(4, abc.len());
        }

        #[test]
        fn alphabet_get() {
            let abc = Alphabet::new("ABCD");
            let mut iter = abc.get().iter(); // Expecting &Vec<char>
            assert_eq!(Some(&'A'), iter.next());
            assert_eq!(Some(&'B'), iter.next());
            assert_eq!(Some(&'C'), iter.next());
            assert_eq!(Some(&'D'), iter.next());
            assert_eq!(None, iter.next());
        }

        #[test]
        fn alphabet_nth() {
            let abc = Alphabet::new("ABCD");
            assert_eq!('A', abc.nth(0));
            assert_eq!('D', abc.nth(3));
        }

        #[test]
        #[should_panic]
        fn alphabet_nth_panic_when_out_of_bounds() {
            let abc = Alphabet::new("ABCD");
            let _x = abc.nth(4); // Should panic here.
        }

        #[test]
        #[should_panic]
        fn produce_password_panic_on_false_length() {
            let alphabets = vec![
                Alphabet::new("ABCD"),
                Alphabet::new("12"),
            ];

            let _p = produce_password(1, &alphabets);
        }

        #[test]
        #[should_panic]
        fn produce_password_panic_on_zero_length() {
            let alphabets = vec![
                Alphabet::new("ABCD"),
                Alphabet::new("12"),
            ];

            let _p = produce_password(0, &alphabets);
        }

        #[test]
        #[should_panic]
        fn produce_password_panic_on_empty_alphabets() {
            let alphabets: Vec<Alphabet> = Vec::new();
            let _p = produce_password(8, &alphabets);
        }
    }
}