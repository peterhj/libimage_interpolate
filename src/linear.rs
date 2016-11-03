use rand::{Rng};

pub fn interpolate2d_linear_u8sr<R>(in_dim: (usize, usize, usize), in_buf: &[u8], out_dim: (usize, usize, usize), out_buf: &mut [u8], rng: &mut R) where R: Rng {
  let (in_chan, in_w, in_h) = in_dim;
  let (out_chan, out_w, out_h) = out_dim;
  assert_eq!(in_chan, out_chan);
  let sx = out_w as f32 / in_w as f32;
  let sy = out_h as f32 / in_h as f32;
  for kv in 0 .. out_h {
    for ku in 0 .. out_w {
      let u = ku as f32;
      let v = kv as f32;
      let ru = (u + 0.5) / out_w as f32;
      let rv = (v + 0.5) / out_h as f32;
      let rx = ru * sx;
      let ry = rv * sy;
      let x = rx * in_w as f32 - 0.5;
      let y = ry * in_h as f32 - 0.5;
      let x0 = x.floor().max(0.0);
      let x1 = x.ceil().min((in_w-1) as f32);
      let y0 = y.floor().max(0.0);
      let y1 = y.ceil().min((in_h-1) as f32);
      let kx0 = x0 as usize;
      let kx1 = x1 as usize;
      let ky0 = y0 as usize;
      let ky1 = y1 as usize;
      for c in 0 .. out_chan {
        let y0_value = if x0 != x1 {
          ((x1 - x) * in_buf[c + in_chan * (kx0 + in_w * ky0)] as f32
              + (x - x0) * in_buf[c + in_chan * (kx1 + in_w * ky0)] as f32)
              / (x1 - x0)
        } else {
          in_buf[c + in_chan * (kx0 + in_w * ky0)] as f32
        };
        let fvalue = if y0 != y1 {
          let y1_value = if x0 != x1 {
            ((x1 - x) * in_buf[c + in_chan * (kx0 + in_w * ky1)] as f32
                + (x - x0) * in_buf[c + in_chan * (kx1 + in_w * ky1)] as f32)
                / (x1 - x0)
          } else {
            in_buf[c + in_chan * (kx0 + in_w * ky1)] as f32
          };
          ((y1 - y) * y0_value + (y - y0) * y1_value) / (y1 - y0)
        } else {
          y0_value
        };
        let fvalue_lo = fvalue.floor();
        let fvalue_hi = fvalue.ceil();
        let z: f32 = rng.gen();
        let value = if z < fvalue - fvalue_lo {
          fvalue_hi
        } else {
          fvalue_lo
        }.min(255.0).max(0.0) as u8;
        out_buf[c + out_chan * (ku + out_w * kv)] = value;
      }
    }
  }
}
