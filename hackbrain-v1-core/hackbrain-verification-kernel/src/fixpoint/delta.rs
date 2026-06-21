use crate::fixpoint::lattice::Mtbdd;

pub fn infinity_norm_delta(a: &Mtbdd, b: &Mtbdd) -> f64 {
    let mut max_delta = 0.0;

    a.traverse_pairwise(b, &mut |va, vb| {
        let d = (va - vb).abs();
        if d > max_delta {
            max_delta = d;
        }
    });

    max_delta
}
