This package provides a non-standard version of truncated SHA512.

The NIST specification uses different initialization vectors for SHA512 and
SHA512_256.

This package implements SHA512 truncated to 256 bits (SHA512_256t) which uses
the *same* initialization vector as full  SHA512.

The motivation of this package is to support SHA512 truncated to 256 bits for
systems where an efficient implementation of the NIST version of truncated
SHA512 is not available for all required platforms or environments. Nowadays,
this usually is the case only for legacy systems.
