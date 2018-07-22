#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate scimark2;
extern crate getopts;
use getopts::*;
use scimark2::array::allocate_2d;
use scimark2::random::*;
use scimark2::*;
use std::env;

const RESOLUTION_DEFAULT:f64 = 2.0;
const RANDOM_SEED:i32 = 101010;

const FFT_SIZE:i32 = 1024;
const SOR_SIZE : i32 = 100;
const SPARSE_SIZE_M:i32 = 1000;
const SPARSE_SIZE_nz:i32 = 5000;
const LU_SIZE:i32 = 100;

const LG_FFT_SIZE:i32 = 1048576;
const LG_SOR_SIZE : i32 = 1000;
const LG_SPARSE_SIZE_M:i32 = 10000;
const LG_SPARSE_SIZE_nz:i32 = 1000000;
const LG_LU_SIZE:i32 = 1000;

const TINY_FFT_SIZE:i32 = 16;
const TINY_SOR_SIZE : i32 = 10;
const TINY_SPARSE_SIZE_M:i32 = 10;
const TINY_SPARSE_SIZE_nz:i32 = 50;
const TINY_LU_SIZE:i32 = 10;

fn main() {
    let mut options = Options::new();
    options.optopt("t","mtime","minimum time for evaluation","");
    options.optflag("l","long","large values");
    options.optflag("h","help","this help");
    let a = allocate_2d(10,10);
    let mut min_time = 2.0;

    let mut FFT_size = FFT_SIZE;
    let mut SOR_size = SOR_SIZE;
    let mut Sparse_size_M = SPARSE_SIZE_M;
    let mut Sparse_size_nz = SPARSE_SIZE_nz;
    let mut LU_size = LU_SIZE;

    let mut R = Random::new_seed(RANDOM_SEED);
    let args: Vec<String> = env::args().collect();
    if let Ok(matches) = options.parse(&args) {

    if matches.opt_present("l") {
        FFT_size = LG_FFT_SIZE;
        SOR_size = LG_SOR_SIZE;
        Sparse_size_M = LG_SPARSE_SIZE_M;
        Sparse_size_nz = LG_SPARSE_SIZE_nz;
        LU_size = LG_LU_SIZE;
    }
    if matches.opt_present("h") {
        print_usage(options);
        return;
    }
    let mt =  matches.opt_str("t");
    min_time = match mt {
        Some(m) => m.parse().unwrap(),
        None => min_time
    };
    print_banner();
    println!("Using {:10.2} seconds min time per kernel",min_time);

    let mut res = [0.0;6];

    res[1] = kernel_measureFFT(FFT_size,min_time,&mut R);
    res[2] = kernel_measure_SOR(SOR_size,min_time,&mut R);
    res[3] = kernel_measureMonteCarlo(min_time,&mut R);
    res[4] = kernel_measureSparseMatMult(Sparse_size_M,Sparse_size_nz,min_time,&mut R);
    res[5] = kernel_measureLU(LU_size,min_time,&mut R);
    res[0] = (res[1]+res[2]+res[3]+res[4]+res[5])/5.0;

    println!("Composite Score:        {:8.2}",res[0]);
    println!("FFT             Mflops: {:8.2}     (N={})",res[1],FFT_size);
    println!("SOR             Mflops: {:8.2}     ({} x {})",res[2],SOR_size,SOR_size);
    println!("MonteCarlo      Mflops: {:8.2}",res[3]);
    println!("Sparse matmult  Mflops: {:8.2}     (N={}, nz={})",res[4],Sparse_size_M,Sparse_size_nz);
    println!("LU              Nflops: {:8.2}     (M={}, N={})",res[5],LU_size,LU_size);
    } else {
        print_usage(options);
    }
}
fn print_banner(){
    println!("**                                                              **");
    println!("** SciMark2 Numeric Benchmark, see http://math.nist.gov/scimark **");
    println!("** for details. (Results can be submitted to pozo@nist.gov)     **");
    println!("**                                                              **");
}
fn print_usage(opts: Options) {
    let brief = format!("Usage: scimark2 [options]");
    print!("{}", opts.usage(&brief));
}
