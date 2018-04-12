use Slice;

pub fn SparseCompRow_num_flops(N:i32,nz:i32,num_iterations:i32)->f64 {
    let actual_nz = (nz/N)*N;
    actual_nz as f64 * 2.0 * num_iterations as f64
}

pub fn SparseCompRow_matmult(M:i32,mut y:Slice<f64>,val:Slice<f64>,row:Slice<i32>,col:Slice<i32>,x:Slice<f64>,
                        NUM_ITERATIONS:i32){
    for reps in 0..NUM_ITERATIONS {
        for r in 0..M as usize {
            let mut sum = 0.0;
            let rowR = row[r] as usize;
            let rowRp1 = row[r+1] as usize;
            for i in rowR..rowRp1 {
                sum += x[col[i] as usize]*val[i];
            }
            y[r] = sum;
        }
    }
}
