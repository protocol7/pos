struct Mem {
    ptr: *mut u8
}

impl Mem {
    pub fn new(ptr: *mut u8) -> Self {
        Mem { ptr }
    }

    pub fn put_byte(&mut self, a: usize, c: u8) {
        unsafe {
            let ptr = (self.ptr.add(a)) as *mut u8;
            core::ptr::write_volatile(ptr, c);
        }
    }

    pub fn get_byte(&mut self, a: usize) -> u8 {
        unsafe {
            let ptr = (self.ptr.add(a)) as *mut u8;
            return core::ptr::read_volatile(ptr);
        }
    }

    pub fn put_bytes(&mut self, a: usize, buf: &[u8]) {
        for i in 0..(buf.len()) {
            self.put_byte(a + i, buf[i]);
        }
    }

    pub fn get_bytes(&mut self, a: usize, buf: &mut [u8]) {
        for i in 0..(buf.len()) {
            buf[i] = self.get_byte(a + i);
        }
    }

    pub fn put_i64(&mut self, a: usize, i: i64) {
        let mut bytes = i.to_ne_bytes();
        self.put_bytes(a, &mut bytes);
    }

    pub fn get_i64(&mut self, a: usize) -> i64 {
        let mut buf = [0u8; 8];
        self.get_bytes(a, &mut buf);
        return i64::from_ne_bytes(buf);
    }

    pub fn get(&mut self, a: i64) -> i64 {
        return self.get_i64((a * 8) as usize);
    }

    pub fn put(&mut self, a: i64, v: i64) {
        return self.put_i64((a * 8) as usize, v);
    }
}

pub fn run(prog: &[i64], ptr: *mut u8) {
    let mut mem = Mem::new(ptr);

    for i in 0..(prog.len()) {
        mem.put(i as i64, prog[i])
    }

    let mut ip = 0;

    loop {
        let mut o = mem.get(ip);
        let op = o % 100;
        o = o / 100;

        let mut ps = [0; 3];
        for i in 0..3 {
            ps[i] = o % 10;
            o /= 10;
        }

        if op == 1  || op == 2{
            // add or mult
            let mut a = mem.get(ip + 1);
            let mut b = mem.get(ip + 2);
            let c = mem.get(ip + 3);

            if ps[0] == 0 {
                a = mem.get(a);
            }
            if ps[1] == 0 {
                b = mem.get(b);
            }

            let res = if op == 1 {
                a + b
            } else {
                a * b
            };
            mem.put(c, res);

            ip += 4;
        } else if op == 3 {
            let a = mem.get(ip + 1);
            mem.put(a, 1);
            ip += 2;
        } else if op == 4 {
            let mut a = mem.get(ip + 1);
            a = mem.get(a);
            println!("out {}", a);
            ip += 2;
        } else if op == 99 {
            break;
        } else {
            panic!("Unknown opcode: {}", op);
        }
    }

    println!("{}", mem.get(0));
}
