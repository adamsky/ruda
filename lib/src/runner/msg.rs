use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Message {
    IntroductionRequest(Uuid),
    IntroductionResponse(bool),
    StatusRequest,
    StatusResponse(String),
}

impl TryInto<tokio_tungstenite::tungstenite::Message> for Message {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<tokio_tungstenite::tungstenite::Message, Self::Error> {
        Ok(tokio_tungstenite::tungstenite::Message::binary(
            pot::to_vec(&self)?,
        ))
    }
}

impl TryFrom<tokio_tungstenite::tungstenite::Message> for Message {
    type Error = anyhow::Error;

    fn try_from(msg: tokio_tungstenite::tungstenite::Message) -> Result<Self, anyhow::Error> {
        match msg {
            tokio_tungstenite::tungstenite::Message::Binary(bytes) => Ok(pot::from_slice(&bytes)?),
            _ => unimplemented!(),
        }
    }
}
