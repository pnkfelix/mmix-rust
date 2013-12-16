#[allow(dead_code)]; // its too broken to be useful at moment.

mod sim {
    struct octa(u64);
    struct Addr(octa);

    static lring_size: int = 256;

    type Globals = [octa, ..256];
    type Locals  = [octa, ..lring_size];
    struct SimState {
        global: Globals,
        local: Locals,
        chunk0: Chunk,
    }

    fn s(arg: octa) -> i64 { *arg as i64 }
    fn u(arg: octa) -> u64 { *arg as u64 }

    fn octa_s(arg: i64) -> octa { octa(arg as u64) }
    fn octa_u(arg: u64) -> octa { octa(arg as u64) }

    impl<'l> SimRegs<'l> {
        fn add(&mut self, y: octa, z: octa) -> i64 { s(y) + s(z) }
        fn sub(&mut self, y: octa, z: octa) -> i64 { s(y) - s(z) }
        fn mul(&mut self, y: octa, z: octa) -> i64 { s(y) * s(z) }
        fn div(&mut self, y: octa, z: octa) -> i64 {
            let ret = s(y) / s(z);
            self.rR = octa_s(s(y) % s(z));
            ret
        }
        fn addu(&mut self, y: octa, z: octa) -> octa { octa_u(u(y) + u(z)) }
        fn subu(&mut self, y: octa, z: octa) -> octa { octa_u(u(y) - u(z)) }
        fn mulu(&mut self, y: octa, z: octa) -> octa {
            // (yh*K+yl)(zh*K+zl)
            //    == (yh*zh) * K*K + (yh*zl+zh*yl) * K + yl*zl
            //             let mid = (yh*zl+zh*yl)
            //    == (yh*zh) * K*K +           mid * K + yl*zl
            //    == (yh*zh) * K*K + (mid div K) * K*K + (mid mod K)*K + yl*zl
            //    == (yh*zh + (mid div K)) * K*K  +  (mid mod K)*K + yl*zl

            fn lo(a:u64) -> u64 { a & 0xffff_ffff }
            fn hi(a:u64) -> u64 { a >> 32 }

            let y_lo = lo(*y);
            let y_hi = hi(*y);
            let z_lo = lo(*z);
            let z_hi = hi(*z);

            let mid = y_hi * z_lo + z_hi * y_lo;
            let mid_lo = lo(mid);
            let mid_hi = hi(mid);

            let result_lower = mid_lo << 32 + y_lo*z_lo;
            let result_upper = (y_hi*z_hi) + mid_hi;
            self.rH = octa_u(result_upper);
            octa_u(result_lower)
        }
        fn divu(&mut self, _y: octa, _z: octa) -> octa {
            fail!("unimplemented");
        }


        fn cmp(&mut self, y: octa, z: octa) -> octa {
            octa_s(if s(y) < s(z) { -1 } else if s(y) == s(z) { 0 } else { 1 })
        }
        fn cmpu(&mut self, y: octa, z: octa) -> octa {
            octa_s(if u(y) < u(z) { -1 } else if u(y) == u(z) { 0 } else { 1 })
        }

    }

    mod mem {
        pub static Chunk : u64 = 0x1000;
        static mask : u64 = Chunk - 1;

        pub struct Regs<'l> {
            head: &'l super::Chunk,
            curkey: super::Addr,
        }

        impl<'l> super::SimRegs<'l> {
            fn mem_find(&mut self, addr: super::Addr) {
                let key = **addr & !mask;
                self.t = self.cmpu(super::octa_u(key),  *self.mem.curkey);
                
            }
        }
    }
    struct SimRegs<'l> {
        t: octa,
        g: &'l Globals,
        l: &'l Locals,
        rA: octa, rB: octa, rC: octa, rD: octa, rE: octa, rF: octa, rG: octa, rH: octa,
        rI: octa, rJ: octa, rK: octa, rL: octa, rM: octa, rN: octa, rO: octa, rP: octa,
        rQ: octa, rR: octa, rS: octa, rT: octa, rU: octa, rV: octa, rW: octa, rX: octa,
        rY: octa, rZ: octa, rBB: octa, rTT: octa, rWW: octa, rXX: octa, rYY: octa, rZZ: octa,
        mem: mem::Regs<'l>,
    }

    struct Chunk {
        key: Addr,
        link: Option<~Chunk>,
        data: [u8, ..mem::Chunk],
        pad: [u8, ..8],
    }
}

fn main() {

}
