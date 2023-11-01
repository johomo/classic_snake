pub trait Component {
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
    fn update(&mut self);
}