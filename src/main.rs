mod client;

fn main() {
    let key = String::from("YOUR_API_KEY");
    let secret = String::from("YOUR_API_SECRET");
    let client = client::new(key, secret);

    client.request(String::from("viewer/accounts/BTC"))
}
