use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn hide(&mut self) {
        self.is_hidden = true;
    }

    pub fn show(&mut self) {
        self.is_hidden = false;
    }

    pub fn edit_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    pub fn switch_role(&mut self, role: Role) {
        self.role = role;
    }
}
