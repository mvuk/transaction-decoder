mod transaction;

// Getting Started
use core::fmt;
use std::io::{Error as ioError, Read};
use std::error::Error;
use self::transaction::{Amount, Input, Output, Transaction, Txid};
use sha2::{Digest, Sha256};

#[allow(unused_variables)]  // this is a macro!
fn read_u32(transaction_bytes: &mut &[u8]) -> Result<u32, ioError> {
    let mut buffer = [0;4];
    transaction_bytes.read(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, ioError> {
    let mut buffer = [0; 8];
    transaction_bytes.read(&mut buffer)?;
    Ok(Amount::from_sat(u64::from_le_bytes(buffer)))
}

fn read_txid(transaction_bytes: &mut &[u8]) -> Result<Txid, ioError> {
    let mut buffer = [0;32];
    transaction_bytes.read(&mut buffer)?;
    buffer.reverse();
    Ok(Txid::from_bytes(buffer))
}

fn read_script(transaction_bytes: &mut &[u8]) -> Result<String, ioError> {
    let script_size = read_compact_size(transaction_bytes)? as usize;
    let mut buffer = vec![0_u8; script_size];
    transaction_bytes.read(&mut buffer)?;
    Ok(hex::encode(buffer))
}

fn hash_raw_transaction(raw_transaction: &[u8]) -> Result<Txid, ioError> {
    let mut hasher = Sha256::new();
    hasher.update(&raw_transaction);
    let hash1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(hash1);
    let hash2 = hasher.finalize();

    Ok(Txid::from_bytes(hash2.into()))
}


fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, ioError> {
    let mut compact_size = [0; 1];
    transaction_bytes.read(&mut compact_size)?;

    match compact_size[0] {
        0..=252 => Ok(compact_size[0] as u64),
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer)?;
            Ok(u16::from_le_bytes(buffer) as u64)
        },
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer)?;
            Ok(u32::from_le_bytes(buffer) as u64)
        },
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer)?;
            Ok(u64::from_le_bytes(buffer))
        }
    }
}

fn decode(transaction_hex: String) -> Result<String, Box<dyn Error>> {
    let transaction_bytes = hex::decode(transaction_hex).map_err(|e| format!("Hex decode error: {}", e))?;
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice)?;
    let input_count = read_compact_size(&mut bytes_slice)?;
    let mut inputs = vec![];

    for _ in 0..input_count {
        let txid = read_txid(&mut bytes_slice)?;
        let output_index = read_u32(&mut bytes_slice)?;
        let script_sig = read_script(&mut bytes_slice)?;
        let sequence = read_u32(&mut bytes_slice)?;

        inputs.push(Input {
            txid,
            output_index,
            script_sig,
            sequence
        });
    }

    let output_count = read_compact_size(&mut bytes_slice)?;
    let mut outputs = vec![];

    for _ in 0..output_count {
        let amount = read_amount(&mut bytes_slice)?;
        let script_pubkey = read_script(&mut bytes_slice)?;

        outputs.push(Output {
            amount,
            script_pubkey
        })
    }

    let lock_time = read_u32(&mut bytes_slice)?;
    let transaction_id = hash_raw_transaction(&transaction_bytes)?;

    let transaction = Transaction {
        version,
        inputs,
        outputs,
        lock_time,
        transaction_id
    };

    Ok(serde_json::to_string_pretty(&transaction)?)

}

fn main() {
    let transaction_hex = "010000000203d88f5043e3653661138f135b8413fe76e2e06e499740a6188d4c2711578e77000000008c493046022100abff60910e31c4e4f1a7069b1722b5713f1e20123b2b3d6c8babe7c7bb52b3a90221009eb38bf18a46a60e86e89003933fc35338347b6919f2df15ba0170df699f4dcc014104b2d872ad172877c722d0cf9886bb314fae7df6d5f1d3299b096823eb09713bacf7337ca3d873f0f0ab17c8e92b1deb5b4e144c583e789f654a31d8cb385edcf4ffffffff2040c782d115ad7192ab902c7c538606225f46cbe42e40605cf5ce2197b0f737020000008c493046022100cab2bce6f3f82a4a761a44184130d61bfd90fcf8aabb4305937d46e5063b86f8022100bc95eeb48eaa3d05b2cff3c6dcf13ca003dfde78bc79297b97d756022ce068e901410415ce05ab909419b1fe8f836bdaa53c51dd652d94cfb81afee06089255497656eac261af477ecae9e56f5a92461afc9e5f870cb839f737a612e68d38d41f18fa3ffffffff03606e3900000000001976a91438248ba06a9044c1581b7ca3de5a144dd965637688ace986c124000000001976a9142644a83ed8042d394720b2bfef7c2febad0f8ba488ac44012400000000001976a914184a01db473d03060cc87d0ef71490f5fa9a975f88ac00000000";

    match decode(transaction_hex.to_string()) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e)
    }

    // println!("Transaction: {}", );
    // Ok(())

    // let json_inputs = serde_json::to_string_pretty(&inputs).unwrap();
    // println!("Version: {}", version);
    // println!("Inputs: {}", json_inputs);
}

#[cfg(test)]
mod unit_tests {
    use super::read_compact_size;
    use super::Error;

    #[test]
    fn test_reading_compact_size() -> Result<(), Box<dyn Error>> {
        let mut bytes = [1_u8].as_slice();
        let length = read_compact_size(&mut bytes)?;
        assert_eq!(length, 1_u64);

        let mut bytes = [253_u8, 0, 1].as_slice();
        let length = read_compact_size(&mut bytes)?;
        assert_eq!(length, 256_u64);

        let mut bytes = [254_u8, 0, 0, 0, 1].as_slice();
        let length = read_compact_size(&mut bytes)?;
        assert_eq!(length, 256_u64.pow(3));

        let mut bytes = [255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
        let length = read_compact_size(&mut bytes)?;
        assert_eq!(length, 256_u64.pow(7));

        // https://mempool.space/tx/52539a56b1eb890504b775171923430f0355eb836a57134ba598170a2f8980c1
        // fd is 253
        // transaction has 20,000 empty inputs
        let hex = "fd204e";
        let decoded = hex::decode(hex)?;
        let mut bytes = decoded.as_slice();
        let length = read_compact_size(&mut bytes)?;
        let expected_length = 20_000_u64;
        assert_eq!(length, expected_length);

        Ok(())
    }
}