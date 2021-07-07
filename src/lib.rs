#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod lu;
use lu::*;
mod fft;
use fft::*;
mod SparseCompRow;
use SparseCompRow::*;
mod sor;
use sor::*;
mod MonteCarlo;
use MonteCarlo::*;
pub mod array;
use array::*;
pub mod random;
use random::*;
use std::ops::IndexMut;
use std::ops::Index;
use array::allocate_2d;
use std::mem;
use std::time::*;

#[derive(Copy,Clone)]
pub struct Slice<T> {
    ptr:*mut T,
    size:usize
}

impl<T> Slice<T> {
    fn new(t:&mut [T])->Self {
        Slice{ptr:t.as_mut_ptr(),size:t.len()}
    }
}

impl<T> IndexMut<usize> for Slice<T> {
    fn index_mut(&mut self,i:usize)->&mut T{
//        assert!(i<self.size);
        unsafe {&mut *((self.ptr as usize + i*mem::size_of::<T>()) as *mut T)}
    }
}


impl<T> Index<usize> for Slice<T> {
    type Output = T;
    fn index(&self,i:usize)->&T{
//        assert!(i<self.size);
        unsafe {&*((self.ptr as usize + i*mem::size_of::<T>()) as *const T)}
    }
}

pub fn kernel_measureFFT(N:i32,mintime:f64,R:&mut Random)->f64 {
    let twoN = 2*N;
    let mut x = R.vector(twoN);

    let mut cycles = 1;
    let mut start;
    loop {
        start = Instant::now();
        for i in 0..cycles {
            fft_transform(twoN,Slice::new(&mut x));
            fft_inverse(twoN,Slice::new(&mut x));
        }
        let end = start.elapsed();
        if end.as_secs() >= mintime as u64 { break }
        cycles *= 2;
    }
    fft_numFlops(N)*cycles as f64 / start.elapsed().as_secs() as f64 * 1e-6
}

pub fn kernel_measure_SOR(N:i32,min_time:f64,R:&mut Random)->f64 {
    let (G1,G2,G) = R.matrix(N,N);
    let mut cycles = 1;
    let mut start;
    loop {
        start = Instant::now();
        sor_execute(N,N,1.25,G,cycles);
        let end = start.elapsed();
        if end.as_secs() >= min_time as u64 {
            break
        }
        cycles *= 2;
    }
    sor_num_flops(N,N,cycles)/start.elapsed().as_secs() as f64*1e-6
}

pub fn kernel_measureMonteCarlo(min_time:f64,R:&mut Random)->f64 {
    let mut cycles = 1;
    let mut start;
    loop {
        start = Instant::now();
        montecarlo_integrate(cycles);
        let end = start.elapsed();
        if end.as_secs() >= min_time as u64 {
            break
        }
        cycles *= 2;
    }
    montecarlo_numFlops(cycles)/start.elapsed().as_secs() as f64 * 1e-6
}

pub fn kernel_measureSparseMatMult(N:i32,nz:i32,min_time:f64,R:&mut Random)->f64 {
    let mut x = R.vector(N);
    let x = Slice::new(&mut x);
    let mut y = vec![0.0;N as usize];
    let y = Slice::new(&mut y);

    let nr = nz/N;
    let anz = nr * N;

    let mut val = R.vector(anz);
    let val = Slice::new(&mut val);
    let mut col = vec![0;nz as usize];
    let mut col = Slice::new(&mut col);
    let mut row = vec![0;N as usize + 1];
    let mut row = Slice::new(&mut row);

    let mut cycles = 1;

    for r in 0..N as usize {
        let rowr = row[r] as usize;
        let mut step = r as i32 / nr;

        row[r+1] = rowr as i32 + nr;

        if step < 1 {
            step = 1;
        }

        for i in 0..nr as usize {
            col[rowr+i]=i as i32 *step;
        }
    }
    let mut start;
    loop {
        start = Instant::now();
        SparseCompRow_matmult(N,y,val,row,col,x,cycles);
        let end = start.elapsed();
        if end.as_secs() >= min_time as u64 {
            break
        }
        cycles *= 2;
    }
    SparseCompRow_num_flops(N,nz,cycles)/start.elapsed().as_secs() as f64*1e-6
}

pub fn kernel_measureLU(N:i32,min_time:f64,R:&mut Random)->f64{
    let mut cycles = 1;
    let mut start;

    let (A1,A2,mut A) = R.matrix(N,N);
    let (lu1,lu2,mut lu) = allocate_2d(N as usize,N as usize);
    let mut pivot = vec![0;N as usize];
    let mut pivot = Slice::new(&mut pivot);

    loop {
        start = Instant::now();
        for i in 0..cycles {
            copy_2d(N,N,lu,A);
            lu_factor(N as usize,N as usize,lu,pivot);
        }
        let end = start.elapsed();
        if end.as_secs() >= min_time as u64 {
            break
        }
        cycles *= 2;
    }
    lu_num_flops(N as usize) * cycles as f64 / start.elapsed().as_secs() as f64 * 1e-6
}
#[test]
fn bitit() {
    let a = allocate_2d(10,10);
}
