use Slice;

pub fn sor_num_flops(M:i32,N:i32,num_iterations:i32)->f64 {
    let Md = M as f64;
    let Nd = N as f64;
    let num_iterD = num_iterations as f64;

    (Md - 1.0) * (Nd - 1.0) * num_iterD*6.0
}

pub fn sor_execute(M:i32,N:i32,omega:f64,G:Slice<Slice<f64>>,num_iterations:i32){
    let omega_over_four = omega * 0.25;
    let one_minus_omega = 1.0 - omega;

    let Mm1 = M as usize - 1;
    let Nm1 = N as usize - 1;
    let (mut Gi,mut Gim1,mut Gip1);

    for p in 0..num_iterations as usize {
        for i in 1..Mm1 {
            Gi = G[i];
            Gim1 = G[i-1];
            Gip1 = G[i+1];
            for j in 1..Nm1 {
                Gi[j] = omega_over_four * (Gim1[j] + Gip1[j] + Gi[j-1] +
                                           Gi[j+1]) + one_minus_omega * Gi[j];
            }
        }
    }
}
