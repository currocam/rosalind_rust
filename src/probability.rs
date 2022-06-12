pub fn calculate_dominant_phenotype (k:u64, m:u64, n:u64) -> f64{
    let k = k as f64;
    let m = m  as f64;
    let n = n  as f64;
    let pop_size = k+m+n;
    let pop_size_minus_one = pop_size -1 as f64;
    let prop_recessive_phenotype= n/pop_size*(n-f64::from(1))/pop_size_minus_one +
        m/pop_size*n/pop_size_minus_one+
        m/pop_size*(m-f64::from(1))/pop_size_minus_one/f64::from(4);
    f64::from(1) - prop_recessive_phenotype
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_calculate_dominant_phenotype() {
        assert!(0.78333- calculate_dominant_phenotype(2, 2, 2) < f64::EPSILON);
    }
}