import random


def number_guessing_game():
    print("Welcome to the Number Guessing Game!")
    print("I'm thinking of a number between 1 and 100...")

    # Generate the secret number
    secret_number = random.randint(1, 100)
    attempts = 0

    while True:
        try:
            guess = int(input("Enter your guess: "))
            attempts += 1
        except ValueError:
            print("Please enter a valid number.")
            continue

        # Basic range check
        if guess < 1 or guess > 100:
            print("Your guess must be between 1 and 100.")
            continue

        # Compare guesses
        if guess < secret_number:
            print("Too low! Try again.")
        elif guess > secret_number:
            print("Too high! Try again.")
        else:
            print("Correct! You guessed it!")
            print(f"The number was {secret_number}.")
            print(f"You got it in {attempts} attempts.")
            break


if __name__ == "__main__":
    number_guessing_game()
