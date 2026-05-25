use rust_decimal::Decimal;
use std::io;
use std::str::FromStr;
use colored::*;

struct Expense {
    id: u64,
    title: String,
    amount: Decimal,
    // date: String,
}

fn main() {
    
    let mut expenses: Vec<Expense> = Vec::new();

    let mut expense_id: u64 = 1;

    loop {
        println!("Enter an option:");
        println!("1. Add an expense");
        println!("2. List all expenses");
        println!("3. Delete an expense");
        println!("4. Quit!");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        match option.trim().parse::<u32>() {
            Ok(number) => match number {
                1 => {
                    println!("Enter expense title: ");

                    let mut expense = String::new();

                    io::stdin().read_line(&mut expense).expect("Invalid input");

                    let title = expense.trim().to_string();

                    expense.clear();

                    println!("Enter the amount: ");

                    io::stdin().read_line(&mut expense).expect("Invalid input");

                    let amount = Decimal::from_str(expense.trim()).unwrap();

                    expenses.push(Expense { id: (expense_id), title: title, amount: amount });
                    
                    expense_id += 1;

                    println!("{}", "Expense added successfully!".green().bold());
                    println!("");
                }
                2 => {
                    println!("");
                    println!("{}", "Expenses: ".green().bold());

                    for (i, expense) in expenses.iter().enumerate() {
                        println!("{}. Title: {}, Amount: {}€, (id: {})", i + 1, expense.title, expense.amount, expense.id);
                    }
                    println!("");
                }
                3 => {
                    let mut delete_expense = String::new();
                    println!("Delete expense by id: ");

                    io::stdin().read_line(&mut delete_expense).expect("Failed to read input");

                    let number: u64 = delete_expense.trim().parse().expect("Enter a valid number");

                    expenses.retain(| expense| expense.id != number);

                    println!("{}", "Expense deleted!".green().bold());
                    println!("");

                }
                4 => {
                    println!("Quitting program!!!");
                    break;
                }
                _ => {
                    println!("Enter a number between 1 and 4");
                }
            },
            Err(_) => {
                println!("Enter a valid number");
            }
        }
    }
}
