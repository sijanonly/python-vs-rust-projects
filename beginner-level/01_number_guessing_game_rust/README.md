This code is a classic Number Guessing Game that effectively demonstrates Rust's core concepts like immutability, mutability, shadowing, error handling, and type inference.

Here is an explanation focusing on the data types and declarations used for each variable.

##  Variable Declarations and Data Types

1. `secret_number`	`let (immutable)`	`i32 (Integer 32-bit, default by inference)`: This variable holds the number the user is trying to guess. It never changes after its initial assignment, so immutability (let) is the default and preferred choice in Rust. i32 is a standard integer type, easily large enough for a number between 1 and 100.

2. `attempts`	`let mut (mutable)`	`i32 (Inferred)`:	This counter tracks how many guesses the user has made. It must change (increment) inside the loop every time the user makes a guess, so it requires the mutable keyword (mut).

3. `guess (1st use)`	`let mut (mutable)`	`String`:	This variable is used to capture the user's input from the console. The `read_line` function requires a mutable string because it writes the new input directly into the existing String buffer, changing its contents.

4. guess (2nd use)	let (immutable, shadowing)	u32 (Unsigned Integer 32-bit)	This is an example of shadowing. We create a new variable named guess that holds the integer value converted from the string. Since this number represents a guess between 1 and 100, it cannot be negative, making the unsigned integer type (u32) appropriate. It's declared immutable (let) because its value does not need to change within the rest of the loop iteration.


## Key Declaration Decisions

1. `let mut guess` (Mutability for Input)

    Code: `let mut guess = String::new();`

    Why `mut`? Rust's String is a growable, owned data type. The standard library function io::stdin().read_line(&mut guess) takes a mutable reference (`&mut guess`) to the string. This is because `read_line` doesn't return a new string; it fills the existing guess string with the user's input. The string's contents are being modified "in place," which requires the variable binding (`guess`) to be declared as mutable.

2. Immutability by Default (`secret_number`)

    Code: `let secret_number = rand::rng().random_range(1..=100);`

    Why `not mut`? Rust encourages developers to use immutable variables by default (let). This prevents accidental modifications and makes code easier to reason about. Since the secret number is generated once and should not change for the entire game, declaring it as immutable is the most idiomatic and safest approach.

3. Shadowing and Type Conversion

    Code: `let guess: u32 = match guess.trim().parse()...`

    Why Shadowing? The code shadows the previous `guess` variable (which was a mutable `String`) with a new, immutable guess variable of type `u32`. This is a common Rust pattern because:

        It allows you to reuse the name (`guess`) without the confusion of a single variable changing both its type and its mutability.

        The original string variable is no longer needed after the conversion, so the new variable represents the cleaner, necessary value (`u32`).

    Why `u32`? An unsigned integer (`u32`) is appropriate because a number guess between 1 and 100 can never be negative. Using an unsigned type guarantees this non-negativity at the type level, which is a key part of Rust's safety philosophy.
