use crate::engine::traits::as_any::AsAny;

pub trait Component: AsAny {
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
    fn update(&mut self);
}
