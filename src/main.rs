use std::sync::{atomic::AtomicBool, atomic::AtomicU64, atomic::Ordering, Arc};

use bech32::ToBase32;
use clap::{App, Arg};
use digest::Digest;
use indicatif::{ProgressBar, ProgressStyle};
use number_prefix::NumberPrefix;
use secp256k1::{
    rand::{thread_rng, RngCore},
    Secp256k1,
};
use sha2::Sha256;

const BECH32_CHARSET: [char; 32] = [
    'q', 'p', 'z', 'r', 'y', '9', 'x', '8', 'g', 'f', '2', 't', 'v', 'd', 'w', '0', 's', '3', 'j',
    'n', '5', '4', 'k', 'h', 'c', 'e', '6', 'm', 'u', 'a', '7', 'l',
];

const BECH32_SEPARATOR: char = '1';

/// Derive first 20 bytes of PKH from public key bytes
fn derive_pkh(bytes: &[u8]) -> [u8; 20] {
    // Hash public key
    let mut hasher = Sha256::new();
    hasher.input(&bytes);
    // Extract First 20 bytes of the hash
    let mut pkh = [0; 20];
    pkh.copy_from_slice(&hasher.result()[..20]);

    pkh
}

/// Progress bar with some pre-computed messages to shown while progressing
struct ExtProgressBar {
    pub pb: ProgressBar,
    pub estimated_runs: f64,
    pub total_msg: String,
}

impl ExtProgressBar {
    /// Creates progress bar and sets part of the progress message
    fn new(estimated_runs: f64) -> Self {
        // Progress bar init
        let obj = Self {
            pb: ProgressBar::new(1_000_000_000),
            estimated_runs,
            total_msg: match NumberPrefix::decimal(estimated_runs) {
                NumberPrefix::Standalone(num) => format!("{}", num),
                NumberPrefix::Prefixed(prefix, n) => format!("{:.2}{}", n, prefix),
            },
        };
        obj.pb.set_style(ProgressStyle::default_bar()
            //.template("{spinner:.green} [{elapsed_precise}] [{bar:80.cyan/blue}] {pos:>7} / {len:7} {msg} (ETA: {eta})")
            .template(&format!(
                "{{spinner:.green}} [{{elapsed_precise}}] [{{bar:50.cyan/blue}}] {{msg}} (ETA: {{eta}})"
            ))
            .progress_chars("#>-"));

        // Update progress bar message
        obj.update(0u64);

        obj
    }

    /// Ticks the progress animation
    fn tick(&self) {
        self.pb.tick();
    }

    /// Updates the progress bar position and message
    fn update(&self, total_count: u64) {
        self.pb.tick();

        // Progress bar position
        let position = (total_count * 1_000_000_000) as f64 / self.estimated_runs;
        self.pb.set_position(position as u64);

        // Message of progress
        let position_msg = match NumberPrefix::decimal(total_count as f64) {
            NumberPrefix::Standalone(num) => format!("{}", num),
            NumberPrefix::Prefixed(prefix, n) => format!("{:.1}{}", n, prefix),
        };

        self.pb
            .set_message(&format!("{}/{} tries", position_msg, self.total_msg));
    }

    fn finish(&self, total_count: u64, msg: &str) {
        self.update(total_count);
        self.pb.set_message(msg);
        self.pb.finish_at_current_pos();
    }
}

fn run(
    id: usize,
    prefix: String,
    vanity: String,
    count: Arc<AtomicU64>,
    success: Arc<AtomicBool>,
    epb: Arc<ExtProgressBar>,
) {
    // Contexts
    let secp = Secp256k1::new();
    let address_start = format!("{}{}{}", prefix, BECH32_SEPARATOR, vanity);

    let mut thread_count = 0;
    let update_count = 500;
    let print_count = 1_000;

    loop {
        // Extract address
        let (secret_key, public_key) = secp.generate_keypair(&mut thread_rng());
        let pkh = derive_pkh(&public_key.serialize());
        let address = bech32::encode(&prefix, pkh.to_base32()).unwrap();

        // Check if starts with prefix + vanity
        if address.starts_with(&address_start) {
            // Break if other thread found a result
            if success.load(Ordering::SeqCst) {
                break;
            }

            // Bytes to encode
            let capacity = 1       // 1 byte for depth (keypath is `0`)
                + 32                    // 32 bytes for chain code
                + 33; // 33 bytes for 0x00 || private key
            let mut bytes = vec![0; capacity];

            // Random chaincode
            let mut chaincode = [0u8; 32];
            thread_rng().fill_bytes(&mut chaincode);

            // 0x00 + Chaincode + 0x00 + Secret key
            bytes[1..33].copy_from_slice(chaincode.as_ref());
            bytes[34..capacity].copy_from_slice(&secret_key[..]);

            let xprv = bech32::encode("xprv", bytes.to_base32()).unwrap();

            // Update stats with current thread count
            // Note: it is not the "real" final count (other threads will stop only when `local_count % update_count == 0`)
            count.fetch_add(thread_count % update_count, Ordering::SeqCst);

            // Print results
            epb.finish(count.load(Ordering::SeqCst), "address found!");
            println!("\nVanity address found:");
            println!("\tSK bytes:\t{}", secret_key);
            println!("\tPrivate key:\t{}", xprv);
            println!("\tAddress:\t{}", address);

            // Set success flag and break
            success.store(true, Ordering::SeqCst);
            break;
        }

        // Check if other threads found a valid result
        if thread_count % update_count == 0 {
            if success.load(Ordering::SeqCst) {
                break;
            }
            count.fetch_add(update_count, Ordering::SeqCst);
            epb.tick();
        }
        // Only first thread is printing progress
        if id == 0 && thread_count % print_count == 0 {
            let total_count = count.load(Ordering::SeqCst);
            epb.update(total_count);
        }

        thread_count += 1;
    }
}

fn main() {
    // Extract arguments
    let matches = App::new("Witnet vanity address generator")
        .version(env!("CARGO_PKG_VERSION"))
        .about(
            "Vanity address generator using curve Secp256k1 and in Bech32 format: <hrp>1<string>",
        )
        .author("Stampery Labs")
        .arg(
            Arg::with_name("vanity-string")
                .help("Vanity prefix string to generate")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("hrp")
                .long("hrp") // allow --hrp
                .short("H")
                .takes_value(true)
                .default_value("wit")
                .help("Human-readable part of the vanity address (e.g. wit, twit, bc)")
                .required(false),
        )
        .arg(
            Arg::with_name("threads")
                .long("threads") // allow --num-threads
                .short("t")
                .takes_value(true)
                .help(
                    "Number of running threads executed in parallel [default: threads = num_cpus]",
                )
                .required(false),
        )
        .get_matches();

    // Extract the actual name
    let vanity = matches
        .value_of("vanity-string")
        .expect("Vanity string is required");

    // Extract the hrp
    let hrp = matches
        .value_of("hrp")
        .expect("Human-readable part should be set");

    // Extract number of cpus to be used
    let num_cpus = num_cpus::get();
    let num_threads = match matches.value_of("threads") {
        None => num_cpus,
        Some(s) => match s.parse::<usize>() {
            Ok(n) => std::cmp::min(n, num_cpus),
            Err(_) => {
                eprintln!("The `num-threads` argument must be a number: {}", s);
                return;
            }
        },
    };

    // Check if vanity string is valid, i.e. contains only Bech32 chars
    for c in vanity.chars() {
        if !BECH32_CHARSET.contains(&c) {
            eprintln!(
                "Vanity string contains the invalid character `{}`.\n\
                      Only Bech32 characters are allowed {:?}.",
                c, BECH32_CHARSET
            );
            return;
        }
    }

    // Show info and progress bar
    println!(
        "\nSearching vanity addresses with the prefix: {}{}{} (threads: {})\n",
        hrp, BECH32_SEPARATOR, vanity, num_threads
    );
    let estimated_runs = 32f64.powi(vanity.len() as i32);
    let pb = Arc::new(ExtProgressBar::new(estimated_runs));

    // Run threads
    let counter = Arc::new(AtomicU64::new(0));
    let flag = Arc::new(AtomicBool::new(false));
    let mut threads = Vec::new();
    for thread_id in 0..num_threads {
        let prefix = hrp.to_string();
        let vanity = vanity.to_string();
        let pb = pb.clone();
        let local_counter = counter.clone();
        let local_flag = flag.clone();
        let thread = std::thread::spawn(move || {
            run(thread_id, prefix, vanity, local_counter, local_flag, pb)
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
