use drbg::ctr::{CtrBuilder, CtrDrbg};
use drbg::entropy::Entropy;
use std::error::Error;
use std::io::{Write, stdout};

struct EntropySuckhard {
  /* length_entropy_min = length_block + length_key
  length_block = 128 bits = 16 bytes
  length_key = 256 bits = 32 bytes
  => length_entropy_min = 32 + 16 = 48 bytes */
  value: [u8; 48],
}

impl Entropy for EntropySuckhard {
  fn fill_bytes(&mut self, value: &mut [u8]) -> Result<(), drbg::entropy::Error> {
      if value.len() > self.value.len() {
        return Result::Err(drbg::entropy::Error::new("value.len() > self.value.len()"));
      }

      for position in 1..value.len() {
        self.value[position] = value[position];
      }

    Ok(())
  }
}

fn build_generator() -> CtrDrbg<EntropySuckhard> {
    const NONCE: [u8; 48] = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 63, 107, 248,
    ];

    const PERSONAL: [u8; 48] = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 63, 107, 248,
    ];

    let builder_generator = CtrBuilder::new(EntropySuckhard {
      value: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 63, 107, 248,
      ],
    })
    .nonce(&NONCE)
    .personal(&PERSONAL);

    return builder_generator.build().unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut generator = build_generator();
  let mut value_generated: [u8; 48] = [0u8; 48];

  // 48 bytes * 2**28 = 12 Gibibytes
  for _ in 1..(u32::MAX >> 4) {
    generator.fill_bytes(&mut value_generated, None)?;
    stdout().write(&value_generated)?;
  }

  Ok(())
}
