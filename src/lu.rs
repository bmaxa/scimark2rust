use Slice;

pub fn lu_num_flops(n:usize)->f64{
    let nd = n as f64;
    
    2.0 * nd * nd * nd / 3.0
}

pub fn lu_factor(m:usize,n:usize, mut a:Slice<Slice<f64>>,mut pivot:Slice<usize>)->usize {
    let min_mn = if m<n {m} else {n};
    
    for j in 0..min_mn{
        let mut jp = j;
        let mut t = a[j][j].abs();
        
        for i in j+1..m {
            let ab = a[i][j].abs();
            if ab > t {
                jp = i;
                t = ab;
            }
        }
        
        pivot[j] = jp;
        
        if a[jp][j] == 0.0 {
            return 1;
        }
        
        if jp != j {
            let ta = a[j];
            a[j] = a[jp];
            a[jp] = ta;
        }
        
        if j < m - 1{
            let recp = 1.0/a[j][j];
            for k in j+1..m {
                a[k][j] *= recp;
            }
        }
        
        if j < min_mn - 1 {
            for ii in j+1..m {
                let mut aii = a[ii];
                let aj = a[j];
                let aiij = aii[j];
                for jj in j+1..n {
                    aii[jj] -= aiij * aj[jj];
                }
            }
        }
    }
    0
}
