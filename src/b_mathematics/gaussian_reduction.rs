use core::mem::swap;
use ultraviolet::DVec2;
use ndarray::{arr1, arr2, Array1};


pub fn my_gaussian_reduction(mut v1: DVec2, mut v2: DVec2) -> (DVec2, DVec2) {
    loop {
        // (a) If ||v2|| < ||v1||, swap v1, v2
        if v2.mag() < v1.mag() {
            swap(&mut v1, &mut v2);
        }
        // v2 is larger

        // (b) Compute m = ⌊ v1∙v2 / v1∙v1 ⌋
        let m = v1.dot(v2) / v1.dot(v1);

        // (c) If m = 0, return v1, v2
        if m.abs() < 0.000000000000001 {
            return (v1, v2);
        }

        // (d) v2 = v2 - m*v1
        v2 -= m * v1;
    }
}


pub fn my_gaussian_reduction_nd(mut v1: Array1<i64>, mut v2: Array1<i64>) -> (Array1<i64>, Array1<i64>) {
    loop {
        // (a) If ||v2|| < ||v1||, swap v1, v2
        if v2.dot(&v2) < v1.dot(&v1) {
            swap(&mut v1, &mut v2);
        }
        // v2 is larger

        // (b) Compute m = ⌊ v1∙v2 / v1∙v1 ⌋
        let m = v1.dot(&v2) / v1.dot(&v1);

        // (c) If m = 0, return v1, v2
        if m.abs() == 0 {
            return (v1, v2);
        }

        // (d) v2 = v2 - m*v1
        v2 = &v2 - m * &v1;
    }


}