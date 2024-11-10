use crate::binary::sender::Sender;
use crate::streaming::session::Session;
use crate::streaming::systems::system::SharedSystem;
use anyhow::Result;
use iggy::error::IggyError;
use iggy::messages::send_messages::SendMessages;
use tracing::debug;

pub async fn handle(
    command: SendMessages,
    sender: &mut dyn Sender,
    session: &Session,
    system: &SharedSystem,
) -> Result<(), IggyError> {
    debug!("session: {session}, command: {command}");
    let system = system.read().await;
    let stream_id = command.stream_id;
    let topic_id = command.topic_id;
    let partitioning = command.partitioning;
    let messages = command.messages;
    // TODO add confirmation
    system
        .append_messages(session, stream_id, topic_id, partitioning, messages, None)
        .await?;
    sender.send_empty_ok_response().await?;
    Ok(())
}
