pub trait TemplateEngine {
    type Template;
    type Context;
    type Error;

    fn add_templates(&mut self, impl Iterator<Item = Self::Template>);
    fn render(&self, impl AsRef<str>, &Self::Context) -> Result<String, Self::Error>;
}
