// automatically generated by rust-bindgen
use libc::*;

#[allow(dead_code)]
const M_E: f64 = 2.718281828459045235360287471353; // e
#[allow(dead_code)]
const M_LOG2E: f64 = 1.442695040888963407359924681002; // log2(e)
#[allow(dead_code)]
const M_LOG10E: f64 = 0.434294481903251827651128918917; // log10(e)
#[allow(dead_code)]
const M_LN2: f64 = 0.693147180559945309417232121458; // ln(2)
#[allow(dead_code)]
const M_LN10: f64 = 2.302585092994045684017991454684; // ln(10)
#[allow(dead_code)]
const M_PI: f64 = 3.141592653589793238462643383280; // pi
#[allow(dead_code)]
const M_2PI: f64 = 6.283185307179586476925286766559; // 2*pi
#[allow(dead_code)]
const M_PI_2: f64 = 1.570796326794896619231321691640; // pi/2
#[allow(dead_code)]
const M_PI_4: f64 = 0.785398163397448309615660845820; // pi/4
#[allow(dead_code)]
const M_1_PI: f64 = 0.318309886183790671537767526745; // 1/pi
#[allow(dead_code)]
const M_2_PI: f64 = 0.636619772367581343075535053490; // 2/pi
#[allow(dead_code)]
const M_2_SQRTPI: f64 = 1.128379167095512573896158903122; // 2/sqrt(pi)
#[allow(dead_code)]
const M_SQRT2: f64 = 1.414213562373095048801688724210; // sqrt(2)
#[allow(dead_code)]
const M_SQRT1_2: f64 = 0.707106781186547524400844362105; // 1/sqrt(2)
#[allow(dead_code)]
const M_SQRT_3: f64 = 1.732050807568877293527446341506; // sqrt(3)
#[allow(dead_code)]
const M_SQRT_32: f64 = 5.656854249492380195206754896838; // sqrt(32)
#[allow(dead_code)]
const M_LOG10_2: f64 = 0.301029995663981195213738894724; // log10(2)
#[allow(dead_code)]
const M_SQRT_PI: f64 = 1.772453850905516027298167483341; // sqrt(pi)
#[allow(dead_code)]
const M_1_SQRT_2PI: f64 = 0.398942280401432677939946059934; // 1/sqrt(2pi)
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
const M_SQRT_2dPI: f64 = 0.797884560802865355879892119869; // sqrt(2/pi)
#[allow(dead_code)]
const M_LN_2PI: f64 = 1.837877066409345483560659472811; // log(2*pi)
#[allow(dead_code)]
const M_LN_SQRT_PI: f64 = 0.572364942924700087071713675677; // log(sqrt(pi))
#[allow(dead_code)]
const M_LN_SQRT_2PI: f64 = 0.918938533204672741780329736406; // log(sqrt(2*pi))
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
const M_LN_SQRT_PId2: f64 = 0.225791352644727432363097614947; // log(sqrt(pi/2))

pub unsafe fn log1p(x: c_double) -> c_double {
    Rlog1p(x)
}

#[link(name = "R")]
extern "C" {
    pub fn Rlog1p(x: c_double) -> ::libc::c_double;
    pub fn R_pow(x: c_double, y: ::libc::c_double) -> ::libc::c_double;
    pub fn R_pow_di(arg1: ::libc::c_double, arg2: ::libc::c_int) -> ::libc::c_double;
    pub fn norm_rand() -> ::libc::c_double;
    pub fn unif_rand() -> ::libc::c_double;
    pub fn exp_rand() -> ::libc::c_double;
    pub fn Rf_dnorm4(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pnorm5(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qnorm5(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rnorm(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_pnorm_both(
        arg1: ::libc::c_double,
        arg2: *mut ::libc::c_double,
        arg3: *mut ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ();
    pub fn Rf_dunif(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_punif(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qunif(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_runif(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dgamma(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pgamma(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qgamma(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rgamma(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_log1pmx(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn log1pexp(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_lgamma1p(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_logspace_add(logx: ::libc::c_double, logy: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_logspace_sub(logx: ::libc::c_double, logy: ::libc::c_double) -> ::libc::c_double;
    pub fn logspace_sum(arg1: *mut ::libc::c_double, arg2: ::libc::c_int) -> ::libc::c_double;
    pub fn Rf_dbeta(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pbeta(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qbeta(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rbeta(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dlnorm(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_plnorm(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qlnorm(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rlnorm(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dchisq(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pchisq(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qchisq(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rchisq(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dnchisq(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pnchisq(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qnchisq(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rnchisq(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_df(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pf(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qf(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rf(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dt(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pt(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qt(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rt(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dbinom_raw(
        x: ::libc::c_double,
        n: ::libc::c_double,
        p: ::libc::c_double,
        q: ::libc::c_double,
        give_log: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_dbinom(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pbinom(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qbinom(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rbinom(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_rmultinom(
        arg1: ::libc::c_int,
        arg2: *mut ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: *mut ::libc::c_int,
    ) -> ();
    pub fn Rf_dcauchy(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pcauchy(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qcauchy(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rcauchy(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dexp(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pexp(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qexp(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rexp(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dgeom(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pgeom(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qgeom(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rgeom(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dhyper(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_double,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_phyper(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_double,
        arg5: ::libc::c_int,
        arg6: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qhyper(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_double,
        arg5: ::libc::c_int,
        arg6: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rhyper(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
    ) -> ::libc::c_double;
    pub fn Rf_dnbinom(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pnbinom(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qnbinom(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rnbinom(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dnbinom_mu(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pnbinom_mu(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qnbinom_mu(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rnbinom_mu(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dpois_raw(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_dpois(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_ppois(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qpois(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rpois(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dweibull(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pweibull(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qweibull(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rweibull(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dlogis(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_plogis(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qlogis(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rlogis(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dnbeta(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_double,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pnbeta(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_double,
        arg5: ::libc::c_int,
        arg6: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qnbeta(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_double,
        arg5: ::libc::c_int,
        arg6: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rnbeta(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
    ) -> ::libc::c_double;
    pub fn Rf_dnf(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_double,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pnf(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_double,
        arg5: ::libc::c_int,
        arg6: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qnf(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_double,
        arg5: ::libc::c_int,
        arg6: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_dnt(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pnt(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qnt(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_ptukey(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_double,
        arg5: ::libc::c_int,
        arg6: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qtukey(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_double,
        arg5: ::libc::c_int,
        arg6: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_dwilcox(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_pwilcox(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qwilcox(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: ::libc::c_int,
        arg5: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rwilcox(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_dsignrank(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_psignrank(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_qsignrank(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
    ) -> ::libc::c_double;
    pub fn Rf_rsignrank(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_gammafn(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_lgammafn(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_lgammafn_sign(arg1: ::libc::c_double, arg2: *mut ::libc::c_int) -> ::libc::c_double;
    pub fn Rf_dpsifn(
        arg1: ::libc::c_double,
        arg2: ::libc::c_int,
        arg3: ::libc::c_int,
        arg4: ::libc::c_int,
        arg5: *mut ::libc::c_double,
        arg6: *mut ::libc::c_int,
        arg7: *mut ::libc::c_int,
    ) -> ();
    pub fn Rf_psigamma(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_digamma(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_trigamma(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_tetragamma(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_pentagamma(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_beta(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_lbeta(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_choose(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_lchoose(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_bessel_i(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
    ) -> ::libc::c_double;
    pub fn Rf_bessel_j(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_bessel_k(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
    ) -> ::libc::c_double;
    pub fn Rf_bessel_y(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_bessel_i_ex(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: *mut ::libc::c_double,
    ) -> ::libc::c_double;
    pub fn Rf_bessel_j_ex(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: *mut ::libc::c_double,
    ) -> ::libc::c_double;
    pub fn Rf_bessel_k_ex(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: ::libc::c_double,
        arg4: *mut ::libc::c_double,
    ) -> ::libc::c_double;
    pub fn Rf_bessel_y_ex(
        arg1: ::libc::c_double,
        arg2: ::libc::c_double,
        arg3: *mut ::libc::c_double,
    ) -> ::libc::c_double;
    pub fn Rf_pythag(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_imax2(arg1: ::libc::c_int, arg2: ::libc::c_int) -> ::libc::c_int;
    pub fn Rf_imin2(arg1: ::libc::c_int, arg2: ::libc::c_int) -> ::libc::c_int;
    pub fn Rf_fmax2(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_fmin2(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_sign(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_fprec(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_fround(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_fsign(arg1: ::libc::c_double, arg2: ::libc::c_double) -> ::libc::c_double;
    pub fn Rf_ftrunc(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn cospi(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn sinpi(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn tanpi(arg1: ::libc::c_double) -> ::libc::c_double;
}

#[test]
fn test_pow() {
    assert_eq!(unsafe { R_pow(1.0, 0.0) }, 1.0);
}
