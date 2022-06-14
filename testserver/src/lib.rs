/*
* Archibald: a loyal web server
* Test function to see if it actually works like it should do
* Author: @danielcuthbert
*
*/

use tonic::{transport::Server};



#[cfg(test)]
mod tests {
    #[test]
    fn connect_to_server() {
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .serve(addr)
        .await?;

    Ok(())

    }
    #[test]
    fn forced_fail() {
        panic!("Oh, m'lud, I have failed you")
    }
}
}
