/// Macro for defining actions with their schemas and metadata in one place.
/// 
/// This macro generates:
/// - ActionSchema enum with EnumIter derive
/// - Action enum 
/// - All required trait implementations (id, name, description, requires_focus)
/// - Schema to Action conversion function
///
/// Usage:
/// ```ignore
/// define_actions! {
///     SchemaEnumName, ActionEnumName {
///         // Basic actions (no focus required)
///         ActionName {
///             id: "action_id",
///             name: "Display Name", 
///             description: "Action description",
///         },
///         
///         // Actions that require focus
///         FocusAction {
///             id: "focus_action_id",
///             name: "Focus Action",
///             description: "Action requiring focus", 
///             requires_focus: true,
///         },
///     }
/// }
/// ```
macro_rules! define_actions {
    (
        $schema_name:ident, $action_name:ident {
            $(
                $variant:ident {
                    id: $id:literal,
                    name: $name:literal,
                    description: $description:literal
                    $(, requires_focus: $requires_focus:tt)?
                    $(,)?
                }
            ),* $(,)?
        }
    ) => {
        // Generate the Schema enum
        #[derive(Debug, Clone, strum::EnumIter)]
        pub enum $schema_name {
            $($variant,)*
        }

        impl $schema_name {
            pub fn id(&self) -> &'static str {
                match self {
                    $(Self::$variant => $id,)*
                }
            }

            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }

            pub fn description(&self) -> &'static str {
                match self {
                    $(Self::$variant => $description,)*
                }
            }

            pub fn requires_focus(&self) -> bool {
                match self {
                    $(
                        Self::$variant => define_actions!(@requires_focus $($requires_focus)?),
                    )*
                }
            }
        }

        // Generate the Action enum
        #[derive(Debug, Clone)]
        pub enum $action_name {
            $($variant,)*
        }

        // Generate schema to action conversion
        impl $schema_name {
            pub fn to_action(&self) -> $action_name {
                match self {
                    $(Self::$variant => $action_name::$variant,)*
                }
            }
        }
    };

    // Helper patterns for requires_focus
    (@requires_focus) => { false };
    (@requires_focus true) => { true };
    (@requires_focus false) => { false };
}

pub(crate) use define_actions;