#[macro_export]
macro_rules! define_core_entity {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident<$child:ty> {
            $child_singular:ident, $child_plural:ident
            $(, extra_fields { $(( $field:ident : $ty:ty, $default:expr )),* $(,)? } )?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq)]
        $vis struct $name {
            id: uuid::Uuid,
            name: String,
            description: Option<String>,
            $child_plural: Vec<$child>,
            $($($field: $ty,)*)?
        }

        paste::paste! {
            impl $name {
                pub fn new(name: impl Into<String>) -> Self {
                    Self {
                        id: uuid::Uuid::new_v4(),
                        name: name.into(),
                        description: None,
                        $child_plural: Vec::new(),
                        $($($field: $default,)*)?
                    }
                }

                pub fn [<from_ $child_plural>](name: impl Into<String>, children: Vec<$child>) -> Self {
                    Self {
                        id: uuid::Uuid::new_v4(),
                        name: name.into(),
                        description: None,
                        $child_plural: children,
                        $($($field: $default,)*)?
                    }
                }

                pub fn id(&self) -> uuid::Uuid {
                    self.id
                }

                pub fn name(&self) -> &str {
                    &self.name
                }

                pub fn description(&self) -> Option<&str> {
                    self.description.as_deref()
                }

                pub fn $child_plural(&self) -> &[$child] {
                    &self.$child_plural
                }

                pub fn [<$child_plural _mut>](&mut self) -> &mut [$child] {
                    &mut self.$child_plural
                }

                pub fn [<get_ $child_singular _by_id>](&self, child_id: uuid::Uuid) -> Option<&$child> {
                    self.$child_plural.iter().find(|c| c.id() == child_id)
                }

                pub fn [<get_ $child_singular _by_id_mut>](&mut self, child_id: uuid::Uuid) -> Option<&mut $child> {
                    self.$child_plural.iter_mut().find(|c| c.id() == child_id)
                }

                pub fn [<get_ $child_singular _index_by_id>](&self, child_id: uuid::Uuid) -> Option<usize> {
                    self.$child_plural.iter().position(|c| c.id() == child_id)
                }

                pub fn [<add_ $child_singular>](&mut self, child: $child) {
                    self.$child_plural.push(child);
                }

                pub fn [<add_ $child_plural>](&mut self, children: Vec<$child>) {
                    self.$child_plural.extend(children);
                }

                pub fn [<remove_ $child_singular _by_id>](&mut self, child_id: uuid::Uuid) -> Option<$child> {
                    if let Some(pos) = self.$child_plural.iter().position(|c| c.id() == child_id) {
                        Some(self.$child_plural.remove(pos))
                    } else {
                        None
                    }
                }

                pub fn clear(&mut self) {
                    self.$child_plural.clear();
                }

                pub fn rename(&mut self, name: impl Into<String>) {
                    self.name = name.into();
                }

                pub fn set_description(&mut self, description: impl Into<String>) {
                    self.description = Some(description.into());
                }

                pub fn clear_description(&mut self) {
                    self.description = None;
                }
            }
        }
    }
}
