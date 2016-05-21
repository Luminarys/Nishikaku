#[macro_export]
macro_rules! impl_entity_enum {
    ($enum_name:ident, $($variant_name:ident),+) => (
		impl $crate::engine::entity::Entity for $enum_name {
            fn handle_event(&mut self, e: $crate::engine::event::Event) {
                match *self {
                    $(
                        $enum_name::$variant_name(ref mut val) => {
                            val.handle_event(e);
                        }
                    ),+
                }
            }

            fn render(&self) -> Option<$crate::engine::entity::RenderInfo> {
                match *self {
                    $(
                        $enum_name::$variant_name(ref val) => {
                            val.render()
                        }
                    ),+
                }
            }

            fn id(&self) -> usize {
                match *self {
                    $(
                        $enum_name::$variant_name(ref val) => {
                            val.id()
                        }
                    ),+
                }
            }
		}
	);
}
