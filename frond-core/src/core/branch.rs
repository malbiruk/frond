use super::error::BranchError;
use super::message::Message;
use uuid::Uuid;

define_collection_wrapper!(Messages, Message);

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
            .get_message_index_by_id(message_id)
            .ok_or(BranchError::MessageNotFound(message_id))?;
        let messages = self.messages.as_slice()[..=idx].to_vec();
        Ok(Branch::from_messages(new_name, messages))
    }

    pub fn llm_context(&self) -> Vec<&Message> {
        self.messages().iter().filter(|m| !m.is_hidden()).collect()
    }

    pub(crate) fn insert_message_at_index(&mut self, index: usize, message: Message) {
        if index <= self.messages.len() {
            self.messages.insert(index, message);
        } else {
            self.messages.push(message);
        }
    }
}
