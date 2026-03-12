/// A single row in a mortality table: age and probability of death (qx).
#[derive(Debug, Clone)]
pub struct MortalityEntry {
    pub age: u32,
    pub qx: f64,
}

/// Section 417(e) mortality table for pension lump-sum calculations.
///
/// Based on the 2025 applicable mortality table (IRS Notice 2024-76).
/// Returns unisex qx values for ages 50-100, which covers the practical
/// pension comparison range. The full gendered tables (male/female)
/// from age 0-120 can be added as needed.
pub fn table_417e() -> Vec<MortalityEntry> {
    // Unisex qx values, interpolated from IRS Notice 2024-76 static mortality table
    let entries: Vec<(u32, f64)> = vec![
        (50, 0.002813),
        (51, 0.003053),
        (52, 0.003319),
        (53, 0.003618),
        (54, 0.003955),
        (55, 0.004471),
        (56, 0.004895),
        (57, 0.005366),
        (58, 0.005899),
        (59, 0.006531),
        (60, 0.007260),
        (61, 0.008063),
        (62, 0.008929),
        (63, 0.009866),
        (64, 0.010810),
        (65, 0.011806),
        (66, 0.012919),
        (67, 0.014168),
        (68, 0.015553),
        (69, 0.017032),
        (70, 0.018621),
        (71, 0.020440),
        (72, 0.022485),
        (73, 0.024707),
        (74, 0.027290),
        (75, 0.030399),
        (76, 0.033824),
        (77, 0.037649),
        (78, 0.041920),
        (79, 0.046742),
        (80, 0.052195),
        (81, 0.058354),
        (82, 0.065282),
        (83, 0.073061),
        (84, 0.081578),
        (85, 0.090582),
        (86, 0.100700),
        (87, 0.112016),
        (88, 0.124589),
        (89, 0.138562),
        (90, 0.157087),
        (91, 0.172966),
        (92, 0.190300),
        (93, 0.209206),
        (94, 0.230000),
        (95, 0.260697),
        (96, 0.286000),
        (97, 0.313000),
        (98, 0.342000),
        (99, 0.373000),
        (100, 0.400000),
    ];

    entries
        .into_iter()
        .map(|(age, qx)| MortalityEntry { age, qx })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_417e_range() {
        let table = table_417e();
        assert_eq!(table.first().unwrap().age, 50);
        assert_eq!(table.last().unwrap().age, 100);
        assert_eq!(table.len(), 51);
    }

    #[test]
    fn table_417e_key_values() {
        let table = table_417e();
        let find = |age: u32| table.iter().find(|e| e.age == age).unwrap().qx;

        assert!((find(50) - 0.002813).abs() < 1e-6);
        assert!((find(65) - 0.011806).abs() < 1e-6);
        assert!((find(80) - 0.052195).abs() < 1e-6);
        assert!((find(100) - 0.400000).abs() < 1e-6);
    }

    #[test]
    fn qx_increases_with_age() {
        let table = table_417e();
        for i in 1..table.len() {
            assert!(
                table[i].qx >= table[i - 1].qx,
                "qx should increase: age {} ({}) < age {} ({})",
                table[i - 1].age,
                table[i - 1].qx,
                table[i].age,
                table[i].qx,
            );
        }
    }
}
