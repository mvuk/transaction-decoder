use clap::Parser;

#[derive(Parser)]
#[command(name = "Transaction Decoder")]
#[command(version = "1.0")]
#[command(about = "Bitcoin Transaction Decoder", long_about = None)]
struct Cli {
    #[arg(
        required = true,
        help = "(string, required) Raw transaction hex"
    )]
    transaction_hex: String,
}

fn main() {
    // let transaction_hex = "010000000203d88f5043e3653661138f135b8413fe76e2e06e499740a6188d4c2711578e77000000008c493046022100abff60910e31c4e4f1a7069b1722b5713f1e20123b2b3d6c8babe7c7bb52b3a90221009eb38bf18a46a60e86e89003933fc35338347b6919f2df15ba0170df699f4dcc014104b2d872ad172877c722d0cf9886bb314fae7df6d5f1d3299b096823eb09713bacf7337ca3d873f0f0ab17c8e92b1deb5b4e144c583e789f654a31d8cb385edcf4ffffffff2040c782d115ad7192ab902c7c538606225f46cbe42e40605cf5ce2197b0f737020000008c493046022100cab2bce6f3f82a4a761a44184130d61bfd90fcf8aabb4305937d46e5063b86f8022100bc95eeb48eaa3d05b2cff3c6dcf13ca003dfde78bc79297b97d756022ce068e901410415ce05ab909419b1fe8f836bdaa53c51dd652d94cfb81afee06089255497656eac261af477ecae9e56f5a92461afc9e5f870cb839f737a612e68d38d41f18fa3ffffffff03606e3900000000001976a91438248ba06a9044c1581b7ca3de5a144dd965637688ace986c124000000001976a9142644a83ed8042d394720b2bfef7c2febad0f8ba488ac44012400000000001976a914184a01db473d03060cc87d0ef71490f5fa9a975f88ac00000000";
    let cli = Cli::parse();
    match transaction_decoder::decode(cli.transaction_hex) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e)
    }

    // println!("Transaction: {}", );
    // Ok(())

    // let json_inputs = serde_json::to_string_pretty(&inputs).unwrap();
    // println!("Version: {}", version);
    // println!("Inputs: {}", json_inputs);
}
