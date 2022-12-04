// 功能相同的Future实现
// futures-rs 0.1
fn download_and_write(user_id: String, socket: Socket) -> impl Future<Output = io::Result<()>> {
    pull_down(user_id)
    .and_then(move |msg| upload(socket))
}


// futures-rs 0.3
async download_and_write(user_id: String, socket: &Socket) -> io::Result<()> {
    let msg = await!(pull_down(user_id))?;
    await!(upload(socket))
}
