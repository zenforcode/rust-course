pub trait Processor {
    fn on_trigger(&self); 
    fn get_name(&self) -> &'static str;
}
pub struct FileProcessor {
    processor_context: ProcessorContext,
}
impl Processor for FileProcessor {
    pub fn new(context: ProcessorContext) -> Self {
        Self { context }
    }
    fn on_trigger(&self) {
        println!("MyProcessor is executing!");
    }

    fn get_name(&self) -> &'static str {
        "FileProcessor"
    }
}