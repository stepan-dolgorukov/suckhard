use std::error::Error;
use std::io::{stdout, Write};
use drbg::ctr::{CtrBuilder, CtrDrbg};
use drbg::entropy::Entropy;

struct EntropySuckhard {
  /* length_entropy_min = length_block + length_key
  length_block = 128 bits = 16 bytes
  length_key = 256 bits = 32 bytes
  => length_entropy_min = 32 + 16 = 48 bytes */
  state: [u8; 48]
}

impl Entropy for EntropySuckhard {
  fn fill_bytes(&mut self, _bytes: &mut [u8]) -> Result<(), drbg::entropy::Error> {
    self.state = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 63, 107, 248];
    Ok(())
  }
}

fn build_generator() -> CtrDrbg<EntropySuckhard> {
  let value_entropy = EntropySuckhard {state: [0u8; 48]};
  const NONCE: [u8; 48] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 63, 107, 248];
  const PERSONAL: [u8; 48] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 63, 107, 248];
  let builder_generator = CtrBuilder::new(value_entropy).
    nonce(&NONCE).
    personal(&PERSONAL);

  return builder_generator.build().unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut generator = build_generator();
  let mut value_generated: [u8; 48] = [0u8; 48];

  // 48 bytes * 10**8 ≈ 4.5 Gibibytes
  for _ in 1..100_000_000u64 {
    let _ = generator.fill_bytes(&mut value_generated, None);
    stdout().write(&value_generated)?;
  }

  Ok(())
}
