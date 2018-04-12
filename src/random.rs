use Slice;
const MDIG :i32 = 32;
const ONE: i32 = 1;
const m1: i32 = (ONE << (MDIG-2)) + ((ONE << (MDIG-2))-ONE);
const m2: i32 = ONE << MDIG/2;

static mut dm1: f64=0.0;

pub struct Random {
    m:[i32;17],
    seed:i32,
    i:i32,
    j:i32,
    haveRange:bool,
    left:f64,
    right:f64,
    width:f64
}
impl Random {
    pub fn new(seed:i32,left:f64,right:f64)->Random {
        let mut rc = Random{m:[0;17],seed:0,i:0,j:0,
                     haveRange:true,left:left,right:right,width:right-left};
        Random::initialize(&mut rc,seed);
        rc
    }
    pub fn new_seed(seed:i32)->Random {
        let mut rc = Random{m:[0;17],seed:0,i:0,j:0,
                     haveRange:false,left:0.0,right:1.0,width:1.0};
        Random::initialize(&mut rc,seed);
        rc
    }

    pub fn nextDouble(&mut self)->f64 {
        let mut k;

        let (mut i,mut j) = (self.i as usize,self.j as usize);
        let mut m = Slice::new(&mut self.m);

        k = m[i]-m[j];

        if  k<0 { k += m1; }
        self.m[j] = k;

        if i == 0 { i = 16; }
        else { i -= 1; }
        self.i = i as i32;

        if j == 0 { j = 16; }
        else { j -= 1; }
        self.j = j as i32;
        unsafe {
        if self.haveRange {
            self.left + dm1 * k as f64 * self.width
        } else {
            dm1 * k as f64
        }}
    }
    pub fn vector(&mut self,N:i32)->Vec<f64> {
        let mut rc = vec![0.0;N as usize];
        for i in rc.iter_mut() {
            *i = self.nextDouble();
        }
        rc
    }
    pub fn matrix(&mut self,M:i32,N:i32)->(Vec<Vec<f64>>,Vec<Slice<f64>>,Slice<Slice<f64>>) {
        let mut rc = vec![vec![0.0;N as usize];M as usize];
        for i in rc.iter_mut() {
            for j in i.iter_mut() {
                *j = self.nextDouble();
            }
        }
        let mut rc1 = Vec::new();
        for i in rc.iter_mut() {
            rc1.push(Slice::new(i));
        }
        let rc2 = Slice::new(&mut rc1);
        (rc,rc1,rc2)
    }
    fn initialize(&mut self, mut seed:i32) {
        let (mut jseed,mut k0,mut k1,mut j0,mut j1);
        unsafe { dm1 = 1.0 / m1 as f64; }
        self.seed = seed;
        if seed < 0 {
            seed = -seed;
        }
        jseed = if seed < m1 {
            seed
        } else {
            m1
        };
        if jseed % 2 == 0 {
            jseed -= 1;
        }
        k0 = 9069 % m2;
        k1 = 9069 / m2;
        j0 = jseed % m2;
        j1 = jseed / m2;
        for iloop in 0..17 {
            jseed = j0 * k0;
            j1 = (jseed / m2 + j0 * k1 + j1 * k0) % (m2/2);
            j0 = jseed % m2;
            self.m[iloop] = j0 + m2 * j1;
        }
        self.i = 4;
        self.j = 16;
    }
}
