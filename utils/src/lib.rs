use ark_ff::BigInt;
use solana_program::hash::{self};
use std::{
    env,
    io::{self, prelude::*},
    mem,
    process::{Command, Stdio},
    thread::spawn,
};
use thiserror::Error;

const CHUNK_SIZE: usize = 32;

#[derive(Debug, Error)]
pub enum UtilsError {
    #[error("Invalid input size, expected {0}, got {1}")]
    InvalidInputSize(usize, usize),
    #[error("Invalid chunk size")]
    InvalidChunkSize,
}

pub fn change_endianness<const SIZE: usize>(bytes: &[u8; SIZE]) -> [u8; SIZE] {
    let mut arr = [0u8; SIZE];
    for (i, b) in bytes.chunks(CHUNK_SIZE).enumerate() {
        for (j, byte) in b.iter().rev().enumerate() {
            arr[i * CHUNK_SIZE + j] = *byte;
        }
    }
    arr
}

/// Converts the given big-endian byte slice into
/// [`ark_ff::BigInt`](`ark_ff::BigInt`).
pub fn be_bytes_to_bigint<const BYTES_SIZE: usize, const NUM_LIMBS: usize>(
    bytes: &[u8; BYTES_SIZE],
) -> Result<BigInt<NUM_LIMBS>, UtilsError> {
    let mut bytes = *bytes;
    bytes.reverse();
    le_bytes_to_bigint(&bytes)
}

/// Converts the given little-endian byte slice into
/// [`ark_ff::BigInt`](`ark_ff::BigInt`).
pub fn le_bytes_to_bigint<const BYTES_SIZE: usize, const NUM_LIMBS: usize>(
    bytes: &[u8; BYTES_SIZE],
) -> Result<BigInt<NUM_LIMBS>, UtilsError> {
    let expected_size = NUM_LIMBS * mem::size_of::<u64>();
    if BYTES_SIZE != expected_size {
        return Err(UtilsError::InvalidInputSize(expected_size, BYTES_SIZE));
    }

    let mut bigint: BigInt<NUM_LIMBS> = BigInt::zero();
    for (i, chunk) in bytes.chunks(mem::size_of::<u64>()).enumerate() {
        bigint.0[i] =
            u64::from_le_bytes(chunk.try_into().map_err(|_| UtilsError::InvalidChunkSize)?);
    }

    Ok(bigint)
}

/// Truncates the given 32-byte array, replacing the least important element
/// with 0, making it fit into Fr modulo field.
///
/// # Safety
///
/// This function is used mostly for truncating hashes (i.e. SHA-256) which are
/// not constrainted by any modulo space. At the same time, we can't (yet) use
/// any ZK-friendly function in one transaction. Truncating hashes to 31 should
/// be generally safe, but please make sure that it's appropriate in your case.
///
/// # Examples
///
/// ```
/// let original: [u8; 32] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
///                            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
///                            29, 30, 31, 32];
/// let truncated: [u8; 32] = truncate_function(&original);
/// assert_eq!(truncated, [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
///                        18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
/// ```
// (Jorrit) this is confusing because the value returned is 32 bytes but the actual values is 31 bytes
pub fn truncate_to_circuit(bytes: &[u8; 32]) -> [u8; 32] {
    let mut truncated = [0; 32];
    truncated[1..].copy_from_slice(&bytes[1..]);
    truncated
}

pub fn hash_and_truncate_to_circuit(bytes: &[&[u8]]) -> [u8; 32] {
    truncate_to_circuit(&hash::hashv(bytes).to_bytes())
}

/// Applies `rustfmt` on the given string containing Rust code. The purpose of
/// this function is to be able to format autogenerated code (e.g. with `quote`
/// macro).
pub fn rustfmt(code: String) -> Result<Vec<u8>, anyhow::Error> {
    let mut cmd = match env::var_os("RUSTFMT") {
        Some(r) => Command::new(r),
        None => Command::new("rustfmt"),
    };

    let mut cmd = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = cmd.stdin.take().unwrap();
    let mut stdout = cmd.stdout.take().unwrap();

    let stdin_handle = spawn(move || {
        stdin.write_all(code.as_bytes()).unwrap();
    });

    let mut formatted_code = vec![];
    io::copy(&mut stdout, &mut formatted_code)?;

    let _ = cmd.wait();
    stdin_handle.join().unwrap();

    Ok(formatted_code)
}
