use Slice;
const PI:f64 = 3.1415926535897932;

fn int_log2(n:i32)->u32 {
    let mut log = 0;
    let mut k = 1;
    while k<n {
        log+=1;
        k*=2;
    }
    if n != (1 << log) {
        panic!(format!("FFT: Data length is not a power of 2!: {}",n));
    }
    log
}

pub fn fft_numFlops(N: i32)->f64 {
    let (Nd,logN) = (N as f64, int_log2(N) as f64);
    (5.0*Nd-2.0)*logN+2.0*(Nd+1.0)
}

fn fft_transform_internal(N:i32,mut data:Slice<f64>,direction:i32) {
    let (mut n,mut logn,mut dual) = (N/2,0,1);

    if n == 1 { return }

    logn = int_log2(n);

    if N == 0 { return }

    fft_bitreverse(N,data);

    for bit in 0..logn {
        let (mut w_real,mut w_imag) = (1.0,0.0);

        let theta = 2.0 * direction as f64 * PI / (2.0 * dual as f64);
        let s = theta.sin();
        let t = (theta/2.0).sin();
        let s2 = 2.0 * t * t;

        for b in (0..n).step_by(2*dual as usize) {
            let i:usize  = 2 * b as usize;
            let j:usize = 2*(b+dual) as usize;

            let wd_real = data[j];
            let wd_imag = data[j+1];

            data[j] = data[i] - wd_real;
            data[j+1] = data[i+1] - wd_imag;
            data[i] += wd_real;
            data[i+1] += wd_imag;

        }
        for a in 1..dual {
            let tmp_real = w_real - s * w_imag - s2 * w_real;
            let tmp_imag = w_imag + s * w_real - s2 * w_imag;
            w_real = tmp_real;
            w_imag = tmp_imag;
            for b in (0..n).step_by(2*dual as usize) {
                let i = 2*(b+a) as usize;
                let j = 2*(b+a+dual) as usize;

                let z1_real = data[j];
                let z1_imag = data[j+1];

                let wd_real = w_real * z1_real - w_imag * z1_imag;
                let wd_imag = w_real * z1_imag + w_imag * z1_real;

                data[j] = data[i] - wd_real;
                data[j+1] = data[i+1] - wd_imag;
                data[i] += wd_real;
                data[i+1] += wd_imag;
            }
        }
        dual *= 2;
    }
}

fn fft_bitreverse(N:i32, mut data: Slice<f64>) {
    let n = N/2;
    let nm1 = n-1;
    let mut j:usize = 0;
    for i in 0..nm1 as usize {
        let mut ii:usize = i << 1;
        let mut jj:usize = j << 1;

        let mut k:usize = n as usize >> 1;

        if i < j {
            let tmp_real = data[ii];
            let tmp_imag = data[ii+1];

            data[ii] = data[jj];
            data[ii+1] = data[jj+1];

            data[jj]= tmp_real;
            data[jj+1] = tmp_imag;
        }
        while k<=j {
            j -= k;
            k >>= 1;
        }
        j += k;
    }
}

pub fn fft_transform(N:i32,data:Slice<f64>) {
    fft_transform_internal(N,data,-1);
}

pub fn fft_inverse(N:i32, mut data:Slice<f64>){
    let n = N/2;
    fft_transform_internal(N,data, 1);

    let norm = 1.0/n as f64;
    for i in 0..N as usize {
        data[i] *= norm;
    }
}
