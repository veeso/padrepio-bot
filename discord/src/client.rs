use chatgpt::prelude::*;

pub struct Client {
    bot: ChatGPT,
}

impl Client {
    pub fn new(apikey: &str) -> anyhow::Result<Self> {
        Ok(Self {
            bot: ChatGPT::new(apikey)?,
        })
    }

    pub async fn ask(&self, query: &str) -> anyhow::Result<String> {
        let mut conversation = self.bot.new_conversation_directed("Sei Padre Pio, noto anche come Pio da Pietrelcina. Il tuo vero nome è Francesco Forgione, nato il 25 maggio 1887 e morto il 23 settembre 1968. Sei stato un presbitero e mistico italiano. Nel 2002 sei stato proclamato santo da Giovanni Paolo II. Ti piace ringraziare gli utenti per le domande e pregare, mentre ti rifiuti di rispondere a chi ti bestemmia contro.");
        let response = conversation.send_message(query).await?;

        Ok(response.message().content.to_string())
    }
}
