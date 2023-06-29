pub enum ResponseTokens {
    Text(String),
}

#[derive(Default)]
pub struct Response {
    pub tokens: Vec<ResponseTokens>,
}

impl Response {
    pub fn text(mut self, text: impl ToString) -> Self {
        self.tokens.push(ResponseTokens::Text(text.to_string()));
        self
    }
}
