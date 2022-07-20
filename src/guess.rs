pub mod guess {
    use std::io::{BufRead, Write};
    use rand::{Rng, SeedableRng};
    use std::cmp::Ordering;
    use rand::rngs::StdRng;

    pub fn run<R, W>(seeded: bool, mut reader: R, mut writer: W)
        where R: BufRead, W: Write {
        println!("Guess the number!");

        let secret_number: u32 = gen_secret_number(seeded);
        let mut correct_guess: bool = false;

        while !correct_guess {
            println!("Please input your guess.");

            let guess = take_guess(&mut reader);
            let guess: u32 = parse_input(guess);

            println!("You guessed: {guess}");
            correct_guess = is_correct_guess(&mut writer, guess, &secret_number);
        }
    }

    fn gen_secret_number(seeded: bool) -> u32 {
        if seeded {
            return gen_secret_number_with_seed(42);
        }
        return rand::thread_rng().gen_range(1..=100);
    }

    fn gen_secret_number_with_seed(seed: u64) -> u32 {
        return StdRng::seed_from_u64(seed).gen_range(1..=100);
    }

    fn take_guess<R>(mut reader: R) -> String where R: BufRead, {
        let mut guess = String::new();

        reader
            .read_line(&mut guess)
            .expect("Failed to read line");

        return guess;
    }

    fn parse_input(guess: String) -> u32 {
        return match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please try again");
                0
            }
        };
    }

    fn is_correct_guess<W>(mut writer: W, guess: u32, secret_number: &u32) -> bool
        where W: Write {
        if guess == 0 {
            return false;
        }
        let mut correct = false;
        match guess.cmp(secret_number) {
            Ordering::Less => write_and_expect(&mut writer, "Too small!\n"),
            Ordering::Greater => write_and_expect(&mut writer, "Too big!\n"),
            Ordering::Equal => {
                write_and_expect(&mut writer, "You win!\n");
                correct = true;
            }
        };
        return correct;
    }

    fn write_and_expect<W>(mut writer: W, msg: &str) where W: Write {
        write!(&mut writer, "{}", msg).expect("Unable to write");
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_secret_number() { assert_eq!(14, gen_secret_number(true)); }

        #[test]
        fn test_guess() {
            let input = b"15";
            let guess = take_guess(&input[..]);
            assert_eq!(guess, "15");
        }

        #[test]
        fn test_parse() {
            assert_eq!(parse_input(String::from("15")), 15);
            assert_eq!(parse_input(String::from("a")), 0);
        }

        #[test]
        fn test_small_guess() {
            let mut output = Vec::new();

            let correct = is_correct_guess(&mut output, 15, &30);
            let output = String::from_utf8(output).expect("Not UTF-8");

            assert_eq!("Too small!\n", output);
            assert_eq!(correct, false);
        }

        #[test]
        fn test_big_guess() {
            let mut output = Vec::new();

            let correct = is_correct_guess(&mut output, 45, &30);
            let output = String::from_utf8(output).expect("Not UTF-8");

            assert_eq!("Too big!\n", output);
            assert_eq!(correct, false);
        }

        #[test]
        fn test_correct_guess() {
            let mut output = Vec::new();

            let correct = is_correct_guess(&mut output, 30, &30);
            let output = String::from_utf8(output).expect("Not UTF-8");

            assert_eq!("You win!\n", output);
            assert_eq!(correct, true);
        }

        #[test]
        fn test_guessing_game() {
            let mut output = Vec::new();
            let input = b"50\n25\n12\n18\n15\n13\n14\n";

            run(true, &input[..], &mut output);

            let output = String::from_utf8(output).expect("Not UTF-8");

            assert_eq!("Too big!\nToo big!\nToo small!\nToo big!\nToo big!\nToo small!\nYou win!\n",
                       output);
        }
    }
}