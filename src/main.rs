use colored::*;
use std::io::prelude::*;
use std::{io, thread::sleep, time::Duration};

struct Card {
    number: String,
    pin_code: u16,
    cvv_cvc: u16,
    balance: f64,
    error_pin: u8,
    blocked: bool,
}

impl Card {
    fn new(number: String, pin_code: u16, cvv_cvc: u16, balance: f64) -> Self {
        Self {
            number,
            pin_code,
            cvv_cvc,
            balance,
            error_pin: 0,
            blocked: false,
        }
    }
}

fn main() {
    let mut cards = create_cards();
    for card in &cards {
        println!(
            "{} {} {} {}",
            card.number, card.pin_code, card.cvv_cvc, card.balance
        );
    }
    'programm: loop {
        println!("{}", "Добро пожаловать в лучший инфокиоск мира!".yellow());
        println!("Выберите тип операции, которую хотите совершить:");
        println!("1. Операция с карточкой");
        println!("2. Операция с наличкой");
        println!("3. Выключить банкомат");

        let input: u32 = read_line().trim().parse().expect("Error");
        match input {
            1 => 'card_operation: loop {
                std::process::Command::new("cls");
                println!("Введите номер карточки, с которой хотите работать, в формате XXXX-XXXX-XXXX-XXXX");
                let card_number = read_line();

                let index_card = find_card(&cards, card_number);
                if index_card == cards.len() {
                    println!("{}", "Вы ввели несуществующую карточку!".red());
                } else if cards[index_card].blocked {
                    println!("{}", "Ваша карточка заблокирована!".red());
                } else {
                    let card = &mut cards[index_card];
                    loop {
                        println!(
                            "Введите пин-код от карточки. У вас осталось {} попытки.",
                            3 - &card.error_pin
                        );
                        let pin: u16 = read_line().trim().parse().expect("Error");

                        if pin != card.pin_code {
                            card.error_pin += 1;
                        } else {
                            break;
                        }

                        if card.error_pin == 3 {
                            println!("Вы превысили доступное кол-во ввода пин-кода! Ваша карточка заблокирована!");
                            card.blocked = true;
                            break;
                        }
                    }
                }
            },
            2 => println!(
                "Вот скажи мне, ты адекватный?! Ты наличку куда пихать будешь??? В дисковод???"
            ),
            3 => {
                println!("Как жаль, что Вы нас покидаете 😢😢😢");
                sleep(Duration::from_millis(3500));
                break 'programm;
            }
            _ => println!("Такого пункта нет..."),
        }
    }
}

fn create_cards() -> [Card; 1] {
    let card1 = Card::new(String::from("1111-1111-1111-1111"), 1234, 123, 145.27);
    [card1]
}

fn find_card(cards: &[Card; 1], number: String) -> usize {
    let mut index = cards.len();
    for i in 0..cards.len() {
        if (cards[i].number).trim() == number.trim() {
            index = i;
        }
    }
    index
}

fn read_line() -> String {
    let mut input = String::new();
    print!(">>> ");
    io::stdout().flush().ok().expect("Error");
    io::stdin().read_line(&mut input).expect("Error");
    input
}
