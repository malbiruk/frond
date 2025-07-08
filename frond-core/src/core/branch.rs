use super::error::BranchError;
use super::message::Message;
use uuid::Uuid;

define_core_entity! {
    pub struct Branch<Message> {
        message, messages
    }
}

impl Branch {
    pub fn fork_from(
        &self,
        message_id: Uuid,
        new_name: impl Into<String>,
    ) -> Result<Branch, BranchError> {
        let idx = self
            .messages
            .iter()
            .position(|m| m.id() == message_id)
            .ok_or(BranchError::MessageNotFound(message_id))?;
        let messages = self.messages[..=idx].to_vec();
        Ok(Branch::from_messages(new_name, messages))
    }

    pub fn llm_context(&self) -> Vec<&Message> {
        self.messages().iter().filter(|m| !m.is_hidden()).collect()
    }
}
