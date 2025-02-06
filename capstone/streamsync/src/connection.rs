#[async_trait::async_trait]
trait Connection {
    async fn send(&self, flowfile: FlowFile);
    async fn receive(&self) -> Option<FlowFile>;
}
