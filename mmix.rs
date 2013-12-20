#[allow(dead_code)]; // its too broken to be useful at moment.

mod sim {
    struct octa(u64);
    struct regn(u8);
    struct Addr(octa);

    static lring_size: int = 256;

    type Globals = [octa, ..256];
    type Locals  = [octa, ..lring_size];
    struct SimState {
        global: Globals,
        local: Locals,
        chunk0: Chunk,
    }

    trait Convert_i64 { fn conv_i(&self) -> i64; }
    trait Convert_u64 { fn conv_u(&self) -> u64; }

    fn s<Octa:Convert_i64>(arg: Octa) -> i64 { arg.conv_i() }
    fn u<Octa:Convert_u64>(arg: Octa) -> u64 { arg.conv_u() }

    fn octa_s(arg: i64) -> octa { octa(arg as u64) }
    fn octa_u(arg: u64) -> octa { octa(arg as u64) }

    trait MmixSemantics {
        fn  ldb(&mut self,          y: regn, z: regn) -> i8  { self.mem_1_a(y, z) }
        fn  ldw(&mut self,          y: regn, z: regn) -> i16 { self.mem_2_a(y, z) }
        fn  ldt(&mut self,          y: regn, z: regn) -> i32 { self.mem_4_a(y, z) }
        fn  ldo(&mut self,          y: regn, z: regn) -> i64 { self.mem_8_a(y, z) }

        fn ldbu(&mut self,          y: octa, z: octa) -> u8  { u(self.mem_1(self.a(y, z))) }
        fn ldwu(&mut self,          y: octa, z: octa) -> u16 { u(self.mem_2(a(y, z))) }
        fn ldtu(&mut self,          y: octa, z: octa) -> u32 { u(self.mem_4(a(y, z))) }
        fn ldou(&mut self,          y: octa, z: octa) -> u64 { u(self.mem_8(a(y, z))) }

        fn ldht(&mut self,          y: octa, z: octa) -> u64 { u(self.mem_4(a(y, z))) << 32 }
        fn  lda(&mut self,          y: octa, z: octa) -> u64 { a(y, z) }
    }

    trait MmixInstructionSet : MmixSemantics {
        // If you can get a register reference, you can implement a lot of instructions
        // off the bat.
        fn r<'l>(&mut self, r: regn) -> &'l mut octa;

        // Address from adding $Y to $Z.  A = (u($Y) + u($Z)) mod 2**64
        fn a(&mut self, y: regn, z: regn) -> u64 { u(*self.r(y)) + u(*self.r(z)) }

        fn mem_1_a(y: regn, z: regn) -> i8;
        fn mem_2_a(y: regn, z: regn) -> i16;
        fn mem_4_a(y: regn, z: regn) -> i32;
        fn mem_8_a(y: regn, z: regn) -> i64;

        fn  ldb(&mut self, x: regn, y: regn, z: regn) { *self.r(x).s() = s(self.mem_1_a(y, z)); }
        fn  ldw(&mut self, x: regn, y: regn, z: regn) { *self.r(x).s() = s(self.mem_2_a(y, z)); }
        fn  ldt(&mut self, x: regn, y: regn, z: regn) { *self.r(x).s() = s(self.mem_4_a(y, z)); }
        fn  ldo(&mut self, x: regn, y: regn, z: regn) { *self.r(x).s() = s(self.mem_8_a(y, z)); }

        fn ldbu(&mut self, x: regn, y: octa, z: octa) { *self.r(x).u() = self.mem_1_a(y, z); }
        fn ldwu(&mut self, x: regn, y: octa, z: octa) { *self.r(x).u() = self.mem_2_a(y, z); }
        fn ldtu(&mut self, x: regn, y: octa, z: octa) { *self.r(x).u() = self.mem_4_a(y, z); }
        fn ldou(&mut self, x: regn, y: octa, z: octa) { *self.r(x).u() = self.mem_8_a(y, z); }

        fn ldht(&mut self, x: regn, y: octa, z: octa) { *self.r(x).u() = self.mem_4_a(y, z) << 32; }
        fn  lda(&mut self, x: regn, y: octa, z: octa) { *self.r(x).u() = a(y, z) }

        fn  stb(&mut self, x: octa, y: octa, z: octa) {  set_mem_i8(a(y, z), x); }
        fn  stw(&mut self, x: octa, y: octa, z: octa) { set_mem_i16(a(y, z), x); }
        fn  stt(&mut self, x: octa, y: octa, z: octa) { set_mem_i32(a(y, z), x); }
        fn  sto(&mut self, x: octa, y: octa, z: octa) { set_mem_i64(a(y, z), x); }

        fn stbu(&mut self, x: octa, y: octa, z: octa) {  set_mem_u8(a(y, z), x); }
        fn stwu(&mut self, x: octa, y: octa, z: octa) { set_mem_u16(a(y, z), x); }
        fn sttu(&mut self, x: octa, y: octa, z: octa) { set_mem_u32(a(y, z), x); }
        fn stou(&mut self, x: octa, y: octa, z: octa) { set_mem_u64(a(y, z), x); }

        fn stht(&mut self, x: octa, y: octa, z: octa) { set_mem_u32(a(y, z), x >> 32); }
        fn stco(&mut self, x: octa, y: octa, z: octa) { set_mem_u64(a(y, z), x); }

        fn add(&mut self,  x: octa, y: octa, z: octa) -> i64 { s(y) + s(z) }
        fn sub(&mut self,  x: octa, y: octa, z: octa) -> i64 { s(y) - s(z) }
        fn mul(&mut self,  x: octa, y: octa, z: octa) -> i64 { s(y) * s(z) }
        fn div(&mut self,  x: octa, y: octa, z: octa) -> i64 {
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

    impl<'l> SimRegs<'l> {
        fn r<'l>(&mut self, r: regn) -> &'l mut octa {
            if regn < self.rL { &'l mut self.locals[*r] } else { &'l mut self.g[*r] }
        }

        fn    a(&mut self, y: regn, z: regn) -> u64 { u(*self.r(y)) + u(*self.r(z)) }


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
