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
    println!("Существующие карточки");
    println!("Номер карточки        PIN-код   CVV/CVC   Баланс");
    for card in &cards {
        println!(
            "{}   {}      {}       {}",
            card.number, card.pin_code, card.cvv_cvc, card.balance
        );
    }
    'programm: loop {
        println!();
        println!("Добро пожаловать в лучший инфокиоск мира!");
        println!("Выберите тип операции, которую хотите совершить:");
        println!("1. Операция с карточкой");
        println!("2. Операция с наличкой");
        println!("3. Выключить банкомат");

        let input: u32 = read_line().trim().parse().expect("Error");
        match input {
            1 => 'card_operation: loop {
                println!();
                std::process::Command::new("cls");
                println!("Введите номер карточки, с которой хотите работать, в формате XXXX-XXXX-XXXX-XXXX");
                let card_number = read_line();

                let index_card = find_card(&cards, card_number);
                if index_card == cards.len() {
                    println!("Вы ввели несуществующую карточку!");
                } else if cards[index_card].blocked {
                    println!("Ваша карточка заблокирована!");
                } else {
                    let card = &mut cards[index_card];
                    'input_pincode: loop {
                        println!();
                        println!(
                            "Введите PIN-код от карточки. У вас осталось {} попытки.",
                            3 - &card.error_pin
                        );
                        let pin: u16 = read_line().trim().parse().expect("Error");

                        if pin != card.pin_code {
                            card.error_pin += 1;
                        } else {
                            break 'input_pincode;
                        }

                        if card.error_pin == 3 {
                            println!("Вы превысили доступное кол-во ввода PIN-кода! Ваша карточка заблокирована!");
                            card.blocked = true;
                            break;
                        }
                    }

                    loop {
                        println!();
                        if card.blocked {
                            break;
                        }
                        println!("Какую операцию вы хотите совершить?");
                        println!("1. Посмотреть баланс");
                        println!("2. Вернуться на главную");
                        let card_option: u32 = read_line().trim().parse().expect("Error");
                        match card_option {
                            1 => {
                                println!("Баланс карточки составляет {}BYN", card.balance);
                            }
                            2 => {
                                break 'card_operation;
                            }
                            _ => println!("Такого пункта нет..."),
                        }
                        println!();
                    }
                }
            },
            2 => println!(
                "Вот скажи мне, ты адекватный?! Ты наличку куда пихать будешь??? В дисковод???"
            ),
            3 => loop {
                println!();
                println!("Оцените работу банкомата от 1 до 10, где 1 - это хорошо, а 10 - сногсшибательно");
                let mark = read_line().trim().parse::<u8>().expect("Error");
                match mark {
                    1..=4 => {
                        println!("Я запомнил тебя -,-\nНу и вали отсюда");
                        sleep(Duration::from_millis(3500));
                        break 'programm;
                    }
                    5..=7 => {
                        println!("Спасибо за оценочку😊. Надеюсь, увидим вас снова");
                        sleep(Duration::from_millis(3500));
                        break 'programm;
                    }
                    8..=10 => {
                        println!("Вы тоже были отличным киентом, которого мы будем ждать! 😘");
                        sleep(Duration::from_millis(3500));
                        break 'programm;
                    }
                    _ => println!("Такой оценки нет..."),
                }
            },
            _ => println!("Такого пункта нет..."),
        }
    }
}

fn create_cards() -> [Card; 3] {
    let card1 = Card::new(String::from("1111-1111-1111-1111"), 1234, 123, 145.27);
    let card2 = Card::new(String::from("2222-2222-2222-2222"), 2345, 234, 256.38);
    let card3 = Card::new(String::from("3333-3333-3333-3333"), 3456, 345, 367.49);
    [card1, card2, card3]
}

fn find_card(cards: &[Card], number: String) -> usize {
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
