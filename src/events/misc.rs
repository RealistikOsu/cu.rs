use crate::packets::{
    builders,
    router::PacketContext
};

pub async fn handle_ping(ctx: &PacketContext<'_>) {
    ctx.player.read().await.queue.enqueue(builders::ping()).await;
}
