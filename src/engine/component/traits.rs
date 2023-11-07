use crate::engine::component::as_any::AsAny;

pub trait Component: AsAny {
    fn get_id(&self) -> &str;
    fn update(&self);
}
