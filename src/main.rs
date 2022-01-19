use clap::{ArgEnum, Parser};
use rayon::prelude::*;

use std::{
    fs,
    io::{self, BufReader},
};

use hasher::hashers;

#[derive(Parser)]
#[clap(version)]

/// Hash a list of files with a given algorithm and print its sums.
struct Cli {
    /// Hashing algorithm to use
    #[clap(long, short, arg_enum, default_value_t = HashAlgo::CRC32)]
    algo: HashAlgo,

    /// Print hashes as they're calculated. Less memory, but no consistent order.
    #[clap(long, short)]
    unsorted: bool,

    /// Files to hash
    files: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum HashAlgo {
    CRC32,
    MD5,
    SHA1,
    SHA256,
    SHA512,
    BLAKE2B160,
    BLAKE2B256,
    BLAKE2b512,
}

fn main() {
    let cli = Cli::parse();
    let hasher = get_hasher(cli.algo);

    let results = cli.files.par_iter().map(|filename| {
        let f = fs::File::open(filename);
        if let Err(err) = f {
            return (filename.to_string(), Err(err));
        }

        let mut f = BufReader::new(f.unwrap());

        (filename.to_string(), hasher.hash(&mut f))
    });

    if cli.unsorted {
        results.for_each(|(filename, sum)| print_result(&filename, sum));
    } else {
        let mut sums = Vec::with_capacity(cli.files.len());
        results.collect_into_vec(&mut sums);

        for (filename, sum) in sums {
            print_result(&filename, sum);
        }
    }
}

fn get_hasher(algo: HashAlgo) -> Box<dyn hashers::Hasher<io::BufReader<fs::File>>> {
    match algo {
        HashAlgo::CRC32 => Box::new(hashers::CRC32Hasher::new()),
        HashAlgo::MD5 => Box::new(hashers::MD5Hasher::new()),
        HashAlgo::SHA1 => Box::new(hashers::SHA1Hasher::new()),
        HashAlgo::SHA256 => Box::new(hashers::SHA256Hasher::new()),
        HashAlgo::SHA512 => Box::new(hashers::SHA512Hasher::new()),
        HashAlgo::BLAKE2B160 => Box::new(hashers::Blake2bHasher::new(20)),
        HashAlgo::BLAKE2B256 => Box::new(hashers::Blake2bHasher::new(32)),
        HashAlgo::BLAKE2b512 => Box::new(hashers::Blake2bHasher::new(64)),
    }
}

fn print_result(filename: &str, sum: Result<Vec<u8>, io::Error>) {
    match sum {
        Ok(sum) => println!("{} {}", bytes_to_hex(&sum), filename),
        Err(err) => eprintln!("{}: {}", filename, err),
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        result.push_str(format!("{:02x}", b).as_str());
    }

    result
}
