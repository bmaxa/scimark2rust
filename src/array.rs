use Slice;

pub fn allocate_2d(m:usize,n:usize)->(Vec<Vec<f64>>,Vec<Slice<f64>>,Slice<Slice<f64>>) {
    let mut vec = vec![vec![0f64;n];m];
    let mut vec1 = Vec::new();
    for i in vec.iter_mut() {
        vec1.push(Slice::new(i));
    }
    let rc = Slice::new(&mut vec1);
    (vec,vec1,rc)
}

pub fn copy_2d(m:i32,n:i32,b:Slice<Slice<f64>>,a:Slice<Slice<f64>>) {
    let remainder = n & 3;

    for i in 0..m as usize {
        let mut bi = b[i];
        let mut ai = a[i];

        for j in 0..remainder as usize {
            bi[j] = ai[j];
        }
        for j in (remainder as usize..n as usize).step_by(4){
            bi[j] = ai[j];
            bi[j+1] = ai[j+1];
            bi[j+2] = ai[j+2];
            bi[j+3] = ai[j+3];
        }
    }
}
