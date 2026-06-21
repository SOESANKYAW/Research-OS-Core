use super::zone::Zone;

pub fn closure(zone: &mut Zone) {
    let n = zone.n_clocks;

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let ik = zone.constraints[i].bound;
                let kj = zone.constraints[j].bound;
                let ij = zone.constraints[i].bound;

                let candidate = ik + kj;

                if candidate < ij {
                    zone.constraints[i].bound = candidate;
                }
            }
        }
    }
}
