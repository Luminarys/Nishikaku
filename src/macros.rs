#[macro_export]
macro_rules! impl_entity_enum {
    ($enum_name:ident, $($variant_name:ident),+) => (
        use std::rc::Rc;
		impl $crate::engine::entity::Entity for $enum_name {
            fn handle_event(&mut self, e: Rc<$crate::engine::event::Event>) {
                match *self {
                    $(
                        $enum_name::$variant_name(ref mut val) => {
                            val.handle_event(e);
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
