use digest::{
    FixedOutputReset,
    FixedOutput,
    Reset,
    Output,
    OutputSizeUser,
    Update,
    HashMarker,
    consts::U32,
    typenum::Unsigned,
};

use digest::Digest;

use sha2::Sha512;

/// SHA512 hash truncated to 256 bits. This is not the NIST standardized
/// version of SHA512/256, which uses a different IV. This implementation
/// returns the first 256 bits of a a standard SHA512 hash.
///
#[derive(Clone)]
pub struct Sha512_256t(Sha512);

impl OutputSizeUser for Sha512_256t {
    type OutputSize = U32;

    fn output_size() -> usize {
        Self::OutputSize::USIZE
    }

}

impl Update for Sha512_256t {
    fn update(&mut self, data: &[u8]) {
        Digest::update(&mut self.0, data)
    }
}

impl Reset for Sha512_256t {
    fn reset(&mut self) {
        Digest::reset(&mut self.0)
    }
}

impl FixedOutputReset for Sha512_256t {
    fn finalize_into_reset(&mut self, out: &mut Output<Self>) {
        let mut tmp : Output<Sha512> = Default::default();
        Digest::finalize_into_reset(&mut self.0, &mut tmp);
        out.copy_from_slice(&tmp[0..32])
    }
}

impl FixedOutput for Sha512_256t {
    fn finalize_into(self, out: &mut Output<Self>) {
        let mut tmp : Output<Sha512> = Default::default();
        Digest::finalize_into(self.0, &mut tmp);
        out.copy_from_slice(&tmp[0..32]);
    }

}

impl Default for Sha512_256t {
    fn default() -> Self {
        Sha512_256t(Sha512::default())
    }
}

impl HashMarker for Sha512_256t { }

#[test]
fn test0() {
    let data: [u8;4] = [1,2,3,4];
    let a = Sha512_256t::digest(&data);

    let mut ctx = Sha512_256t::new();

    Digest::update(&mut ctx, &data);
    let b = ctx.finalize_reset();
    assert_eq!(a, b);

    Digest::update(&mut ctx, &data[..2]);
    Digest::update(&mut ctx, &data[2..]);
    let c = ctx.finalize_reset();
    assert_eq!(a, c);

    Digest::update(&mut ctx, &data);
    let d = ctx.finalize();
    assert_eq!(a, d);
}
