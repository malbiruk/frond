use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    id: Uuid,
    content: String,
    role: Role,
    is_hidden: bool,
}

impl Message {
    pub fn new(content: impl Into<String>, role: Role) -> Self {
        Message {
            id: Uuid::new_v4(),
            content: content.into(),
            role,
            is_hidden: false,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    pub(crate) fn hide(&mut self) {
        self.is_hidden = true;
    }

    pub(crate) fn show(&mut self) {
        self.is_hidden = false;
    }

    pub(crate) fn edit_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    pub(crate) fn toggle_role(&mut self) {
        self.role = match self.role {
            Role::User => Role::Assistant,
            Role::Assistant => Role::User,
        };
    }
}
