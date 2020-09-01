///! # BSPricer
///! Serial and Vectorised version of:
///!
///! * Black scholes
///! * Greeks  
///! * Binomial
///! * Implied vol
///! * Implied Interest rates
///! * Strike from delta
///!
///! This library depends on the wide library which provides the crucial math functions exp/log/pow/cdf in vectorised versions. This makes the difference of over 50%
///! compared to the serial versions of this function.
///!
///! Somewhat surprisingly on the (admittedly) small sample of PCs I've run it on with FMA/AVX instructions, the code generated is faster than the equivalent with Intel's ISPC.
///!
///! Compared to any other open source version of black scholes pricing I've found online, I believe this is the world's fastest CPU version. GPU versions can be faster depending on the circumstances
///!
///! On an i5 7300HQ I'm seeing 10,000,000 prices calculated in just under 100ms.  YMMV
///! Compared to a serialised version of around 1800ms
pub mod bs;
mod bs_f32x8_;
pub mod bs_single;