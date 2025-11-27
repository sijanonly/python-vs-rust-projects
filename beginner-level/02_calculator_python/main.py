#!/usr/bin/env python3
"""
Simple Calculator (Python)

Supports:
+  Addition
-  Subtraction
*  Multiplication
/  Division
"""

def get_number(prompt):
    """Get a valid number from the user."""
    while True:
        try:
            return float(input(prompt))
        except ValueError:
            print("Please enter a valid number.")


def get_operator():
    """Get a valid operator from the user."""
    while True:
        operator = input("Enter an operator (+, -, *, /): ").strip()

        if operator in ["+", "-", "*", "/"]:
            return operator

        print("Invalid operator. Please choose +, -, *, or /.")


def calculate(num1, num2, operator):
    """Perform the calculation."""
    if operator == "+":
        return num1 + num2

    if operator == "-":
        return num1 - num2

    if operator == "*":
        return num1 * num2

    if operator == "/":
        if num2 == 0:
            return "Error: Division by zero"
        return num1 / num2


def run_calculator():
    print("\nSimple Calculator (Python)")
    print("-" * 30)

    while True:
        num1 = get_number("Enter first number: ")
        operator = get_operator()
        num2 = get_number("Enter second number: ")

        result = calculate(num1, num2, operator)
        print(f"\nResult: {result}")

        again = input("\nDo you want to calculate again? (y/n): ").lower().strip()
        if again != "y":
            print("Goodbye!")
            break


if __name__ == "__main__":
    run_calculator()
