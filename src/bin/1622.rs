struct Fancy {
    cache:Vec<i64>,
    opcodes:Vec<u8>,
    opargs:Vec<i32>,
    inserts:Vec<usize>,
    last_insert:i32,
}

const MOD:i64 = 1_000_000_007;
/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {

    fn new() -> Self {
        Self{
            cache:Vec::new(),
            opcodes:Vec::new(),
            opargs:Vec::new(),
            inserts:Vec::new(),
            last_insert:-1,
        }
    }
    
    fn append(&mut self, val: i32) {
        self.inserts.push(self.opcodes.len());
        self.cache.push(val as i64);
        self.last_insert = self.opcodes.len() as i32;
    }
    
    fn add_all(&mut self, inc: i32) {
        if self.opcodes.len() > 0 
            && self.opcodes.len() as i32 > self.last_insert 
            && *self.opcodes.last().unwrap() == 0 {
                let val = (inc + self.opargs.pop().unwrap()) % MOD as i32;
                self.opargs.push(val);
        } else {
            self.opcodes.push(0);
            self.opargs.push(inc);
        }
    }
    
    fn mult_all(&mut self, m: i32) {
        if self.opcodes.len() > 0 
            && self.opcodes.len() as i32 > self.last_insert 
            && *self.opcodes.last().unwrap() == 1 {
                let val = (m as i64 * self.opargs.pop().unwrap() as i64) % MOD;
                self.opargs.push(val as i32);
        } else {        
            self.opcodes.push(1);
            self.opargs.push(m);
        }
    }
    
    fn get_index(&mut self, idx: i32) -> i32 {
        let idx = idx as usize;
        if idx >= self.cache.len() {return -1};

        let mut res = self.cache[idx];

        for i in (self.inserts[idx])..(self.opcodes.len()) {
            if self.opcodes[i] == 0 {
                res = (res + self.opargs[i] as i64) % MOD;
            } else {
                res = (res * self.opargs[i] as i64) % MOD;
            }    
        }
        self.cache[idx] = res;
        self.inserts[idx] = self.opcodes.len();
        self.last_insert = self.opcodes.len() as i32;
        return res as i32;
    }
}

/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */



fn main() {

}