struct NFT {
    image: String,
    price: u128
}

impl NFT {
    fn transfer_nft(&self, address: String){
        println!("The NFT has been transfered to {}", address);
    }
}

fn main() {
    let new_nft: NFT = NFT {
        image: "A photo of a dog".to_string(),
        price: 100000
    };
    println!("NFT: {}, {}", new_nft.image, new_nft.price);
    new_nft.transfer_nft("Izzy Wallet".to_string());
}