use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    
    for i in 1..3 {
        let new_tx = tx.clone();
        tokio::spawn(async move {
            for j in (i - 1) * 100 + 1..i * 100 {
                new_tx.send((i, j)).await.ok();
            }
        });
    }

    drop(tx);

    while let Some((id,message)) = rx.recv().await {
        println!("consumer get {} from producer {}", message, id);
    }
}
