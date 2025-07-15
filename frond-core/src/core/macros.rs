#[macro_export]
macro_rules! define_collection_wrapper {
    ($wrapper_name:ident, $item_type:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $wrapper_name(Vec<$item_type>);

        #[allow(dead_code)]
        impl $wrapper_name {
            pub fn new() -> Self {
                Self(Vec::new())
            }

            pub fn from_vec(items: Vec<$item_type>) -> Self {
                Self(items)
            }

            pub fn get(&self, index: usize) -> Option<&$item_type> {
                self.0.get(index)
            }

            pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut $item_type> {
                self.0.get_mut(index)
            }

            pub fn get_by_id(&self, id: uuid::Uuid) -> Option<&$item_type> {
                self.0.iter().find(|item| item.id() == id)
            }

            pub(crate) fn get_by_id_mut(&mut self, id: uuid::Uuid) -> Option<&mut $item_type> {
                self.0.iter_mut().find(|item| item.id() == id)
            }

            pub fn get_index_by_id(&self, id: uuid::Uuid) -> Option<usize> {
                self.0.iter().position(|item| item.id() == id)
            }

            pub(crate) fn push(&mut self, item: $item_type) {
                self.0.push(item);
            }

            pub(crate) fn extend(&mut self, items: Vec<$item_type>) {
                self.0.extend(items);
            }

            pub(crate) fn remove_by_id(&mut self, id: uuid::Uuid) -> Option<$item_type> {
                if let Some(pos) = self.get_index_by_id(id) {
                    Some(self.0.remove(pos))
                } else {
                    None
                }
            }

            pub(crate) fn remove(&mut self, index: usize) -> $item_type {
                self.0.remove(index)
            }

            pub(crate) fn insert(&mut self, index: usize, item: $item_type) {
                self.0.insert(index, item)
            }

            pub(crate) fn clear(&mut self) {
                self.0.clear();
            }

            pub fn iter(&self) -> impl Iterator<Item = &$item_type> {
                self.0.iter()
            }

            pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut $item_type> {
                self.0.iter_mut()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn as_slice(&self) -> &[$item_type] {
                &self.0
            }

            pub(crate) fn as_mut_slice(&mut self) -> &mut [$item_type] {
                &mut self.0
            }

            pub fn into_vec(self) -> Vec<$item_type> {
                self.0
            }

            pub fn to_vec(&self) -> Vec<$item_type>
            where
                $item_type: Clone,
            {
                self.0.clone()
            }
        }

        impl std::ops::Index<usize> for $wrapper_name {
            type Output = $item_type;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl Default for $wrapper_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

#[macro_export]
macro_rules! define_core_entity {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident<$child:ty> {
            $child_singular:ident, $child_plural:ident
            $(, extra_fields { $(( $field:ident : $ty:ty, $default:expr )),* $(,)? } )?
        }
    ) => {
        paste::paste! {
            $(#[$meta])*
            #[derive(Debug, Clone, PartialEq)]
            $vis struct $name {
                id: uuid::Uuid,
                name: String,
                description: Option<String>,
                $child_plural: [<$child_plural:camel>],
                $($($field: $ty,)*)?
            }

            #[allow(dead_code)]
            impl $name {
                pub fn new(name: impl Into<String>) -> Self {
                    Self {
                        id: uuid::Uuid::new_v4(),
                        name: name.into(),
                        description: None,
                        $child_plural: [<$child_plural:camel>]::new(),
                        $($($field: $default,)*)?
                    }
                }

                pub fn [<from_ $child_plural>](name: impl Into<String>, children: Vec<$child>) -> Self {
                    Self {
                        id: uuid::Uuid::new_v4(),
                        name: name.into(),
                        description: None,
                        $child_plural: [<$child_plural:camel>]::from_vec(children),
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

                pub fn $child_plural(&self) -> &[<$child_plural:camel>] {
                    &self.$child_plural
                }

                pub(crate) fn [<$child_plural _mut>](&mut self) -> &mut [<$child_plural:camel>] {
                    &mut self.$child_plural
                }

                pub fn [<get_ $child_singular _by_id>](&self, child_id: uuid::Uuid) -> Option<&$child> {
                    self.$child_plural.get_by_id(child_id)
                }

                pub(crate) fn [<get_ $child_singular _by_id_mut>](&mut self, child_id: uuid::Uuid) -> Option<&mut $child> {
                    self.$child_plural.get_by_id_mut(child_id)
                }

                pub fn [<get_ $child_singular _index_by_id>](&self, child_id: uuid::Uuid) -> Option<usize> {
                    self.$child_plural.get_index_by_id(child_id)
                }

                pub(crate) fn [<add_ $child_singular>](&mut self, child: $child) {
                    self.$child_plural.push(child);
                }

                pub(crate) fn [<add_ $child_plural>](&mut self, children: Vec<$child>) {
                    self.$child_plural.extend(children);
                }

                pub(crate) fn [<remove_ $child_singular _by_id>](&mut self, child_id: uuid::Uuid) -> Option<$child> {
                    self.$child_plural.remove_by_id(child_id)
                }

                pub(crate) fn clear(&mut self) {
                    self.$child_plural.clear();
                }

                pub(crate) fn rename(&mut self, name: impl Into<String>) {
                    self.name = name.into();
                }

                pub(crate) fn set_description(&mut self, description: impl Into<String>) {
                    self.description = Some(description.into());
                }

                pub(crate) fn clear_description(&mut self) {
                    self.description = None;
                }
            }
        }
    }
}
