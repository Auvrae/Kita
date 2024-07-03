use std::fs;
use std::io::BufReader;
use std::io::Read;
use sha2::*;
use sha1::*;
use md5::*;
use crc32fast;
use rustc_serialize::hex::ToHex;

use super::threads;

#[derive(Clone, Debug)]
pub enum HashType {
    CRC32,
    MD5,
    Sha1,
    Sha256
}

pub fn hash_file(path: String, hash_mode: &HashType, endianness: threads::Endianness) -> String {
    let file = fs::File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];
    match hash_mode {
        HashType::CRC32 => {
            let mut hasher = crc32fast::Hasher::new();
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 { break }
                hasher.update(&buffer[..count]);
            }
            match endianness {
                threads::Endianness::BigEndian => {
                    return format!("{}", &hasher.finalize().to_be_bytes().as_slice().to_hex().to_ascii_uppercase());
                },
                threads::Endianness::_LittleEndian => {
                    return format!("{}", &hasher.finalize().to_le_bytes().as_slice().to_hex().to_ascii_uppercase());
                }
            }
        },
        HashType::MD5 => {
            let mut hasher = Md5::new();
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 { break }
                hasher.update(&buffer[..count]);
            }

            return format!("{}", &hasher.finalize().as_slice().to_hex());
        },
        HashType::Sha1 => {
            let mut hasher = Sha1::new();
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 { break }
                hasher.update(&buffer[..count]);
            }
            return format!("{}", &hasher.finalize().as_slice().to_hex());
        },
        HashType::Sha256 => {
            let mut hasher = Sha256::new();
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 { break }
                hasher.update(&buffer[..count]);
            }
            return format!("{}", &hasher.finalize().as_slice().to_hex());
        }
    };
}