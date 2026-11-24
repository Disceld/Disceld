use std::io::{self, Write};

struct Message {
    author: String,
    content: String,
}

struct Channel {
    name: String,
    messages: Vec<Message>,
}

impl Channel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            messages: Vec::new(),
        }
    }

    fn send(&mut self, author: &str, content: &str) {
        self.messages.push(Message {
            author: author.to_string(),
            content: content.to_string(),
        });
    }

    fn show_history(&self) {
        println!("\n--- #{} ---", self.name);
        if self.messages.is_empty() {
            println!("(Brak wiadomości. Napisz coś!)");
        } else {
            for msg in &self.messages {
                println!("[{}]: {}", msg.author, msg.content);
            }
        }
        println!("----------------");
    }
}

fn main() {
    let mut accounts = vec![
        String::from("OdpMod101"),
        String::from("GoofyAlt"),
    ];
    let mut current_acc = 0;

    let mut channels = vec![
        Channel::new("general"),
        Channel::new("goofy-zone"),
    ];
    let mut current_channel = 0;

    println!("=== DISCEL 0.1 (Project Phoenix Killer) ===");
    println!("Komendy: /channel <nazwa> | /list | /switch <nick> | /accounts | /quit");

    loop {
        channels[current_channel].show_history();

        print!("{} (#{}) > ", accounts[current_acc], channels[current_channel].name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "/quit" {
            println!("Zamykanie Discelda...");
            break;
        } else if input == "/list" {
            println!("Dostępne kanały:");
            for (i, ch) in channels.iter().enumerate() {
                println!("{}. #{}", i + 1, ch.name);
            }
        } else if input == "/accounts" {
            println!("Zapisane konta:");
            for (i, acc) in accounts.iter().enumerate() {
                let active = if i == current_acc { " (aktywne)" } else { "" };
                println!("{}. {}{}", i + 1, acc, active);
            }
        } else if let Some(acc_name) = input.strip_prefix("/switch ") {
            let acc_name = acc_name.trim();
            if let Some(pos) = accounts.iter().position(|a| a == acc_name) {
                current_acc = pos;
                println!("Przełączono na konto: {}", acc_name);
            } else {
                accounts.push(acc_name.to_string());
                current_acc = accounts.len() - 1;
                println!("Dodano i przełączono na nowe konto: {}", acc_name);
            }
        } else if let Some(new_name) = input.strip_prefix("/channel ") {
            let new_name = new_name.trim();
            if let Some(pos) = channels.iter().position(|c| c.name == new_name) {
                current_channel = pos;
            } else {
                channels.push(Channel::new(new_name));
                current_channel = channels.len() - 1;
                println!("Utworzono nowy kanał #{}!", new_name);
            }
        } else {
            channels[current_channel].send(&accounts[current_acc], input);
        }
    }
}
