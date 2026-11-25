use wasm_bindgen::prelude::*;

struct Message {
    author: String,
    content: String,
}

struct Channel {
    name: String,
    messages: Vec<Message>,
}

#[wasm_bindgen]
pub struct DisceldApp {
    accounts: Vec<String>,
    current_acc: usize,
    channels: Vec<Channel>,
    current_channel: usize,
}

#[wasm_bindgen]
impl DisceldApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            accounts: vec![
                String::from("OdpMod101"),
                String::from("GoofyAlt"),
            ],
            current_acc: 0,
            channels: vec![
                Channel { name: String::from("general"), messages: Vec::new() },
                Channel { name: String::from("goofy-zone"), messages: Vec::new() },
            ],
            current_channel: 0,
        }
    }

    pub fn send_message(&mut self, content: &str) {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return;
        }
        let author = self.accounts[self.current_acc].clone();
        self.channels[self.current_channel].messages.push(Message {
            author,
            content: trimmed.to_string(),
        });
    }

    pub fn switch_account(&mut self, name: &str) {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        if let Some(pos) = self.accounts.iter().position(|a| a == trimmed) {
            self.current_acc = pos;
        } else {
            self.accounts.push(trimmed.to_string());
            self.current_acc = self.accounts.len() - 1;
        }
    }

    pub fn switch_channel(&mut self, name: &str) {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        if let Some(pos) = self.channels.iter().position(|c| c.name == trimmed) {
            self.current_channel = pos;
        } else {
            self.channels.push(Channel {
                name: trimmed.to_string(),
                messages: Vec::new(),
            });
            self.current_channel = self.channels.len() - 1;
        }
    }

    pub fn get_current_account(&self) -> String {
        self.accounts[self.current_acc].clone()
    }

    pub fn get_current_channel(&self) -> String {
        self.channels[self.current_channel].name.clone()
    }

    pub fn render_messages_html(&self) -> String {
        let ch = &self.channels[self.current_channel];
        if ch.messages.is_empty() {
            return String::from("<div class='empty'>Brak wiadomości na tym kanale. Napisz coś!</div>");
        }
        let mut html = String::new();
        for msg in &ch.messages {
            html.push_str(&format!(
                "<div class='msg'><span class='author'>{}</span>: <span class='text'>{}</span></div>",
                msg.author, msg.content
            ));
        }
        html
    }

    pub fn render_channels_html(&self) -> String {
        let mut html = String::new();
        for (i, ch) in self.channels.iter().enumerate() {
            let active = if i == self.current_channel { "active" } else { "" };
            html.push_str(&format!(
                "<button class='item {}' onclick=\"selectChannel('{}')\"># {}</button>",
                active, ch.name, ch.name
            ));
        }
        html
    }

    pub fn render_accounts_html(&self) -> String {
        let mut html = String::new();
        for (i, acc) in self.accounts.iter().enumerate() {
            let active = if i == self.current_acc { "active" } else { "" };
            html.push_str(&format!(
                "<button class='item {}' onclick=\"selectAccount('{}')\">@ {}</button>",
                active, acc, acc
            ));
        }
        html
    }
              }
