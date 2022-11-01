struct NFT {
    image: String,
    value: u128
}

fn main() {
    let new_nft = NFT {
        image: "The Corinthians Symbol".to_string(),
        value: 100000
    };
    println!("{}, {}", new_nft.image, new_nft.value);
}
