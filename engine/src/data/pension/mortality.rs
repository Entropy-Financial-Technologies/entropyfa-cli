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
    vec![
        MortalityEntry {
            age: 50,
            qx: 0.0009,
        },
        MortalityEntry {
            age: 51,
            qx: 0.001,
        },
        MortalityEntry {
            age: 52,
            qx: 0.00112,
        },
        MortalityEntry {
            age: 53,
            qx: 0.00125,
        },
        MortalityEntry {
            age: 54,
            qx: 0.00142,
        },
        MortalityEntry {
            age: 55,
            qx: 0.0017,
        },
        MortalityEntry {
            age: 56,
            qx: 0.00208,
        },
        MortalityEntry {
            age: 57,
            qx: 0.00242,
        },
        MortalityEntry {
            age: 58,
            qx: 0.00281,
        },
        MortalityEntry {
            age: 59,
            qx: 0.00322,
        },
        MortalityEntry {
            age: 60,
            qx: 0.00373,
        },
        MortalityEntry {
            age: 61,
            qx: 0.00427,
        },
        MortalityEntry {
            age: 62,
            qx: 0.00504,
        },
        MortalityEntry {
            age: 63,
            qx: 0.00582,
        },
        MortalityEntry {
            age: 64,
            qx: 0.00645,
        },
        MortalityEntry {
            age: 65,
            qx: 0.00729,
        },
        MortalityEntry {
            age: 66,
            qx: 0.00819,
        },
        MortalityEntry {
            age: 67,
            qx: 0.00906,
        },
        MortalityEntry {
            age: 68,
            qx: 0.01001,
        },
        MortalityEntry {
            age: 69,
            qx: 0.01108,
        },
        MortalityEntry {
            age: 70,
            qx: 0.01232,
        },
        MortalityEntry {
            age: 71,
            qx: 0.01374,
        },
        MortalityEntry {
            age: 72,
            qx: 0.01535,
        },
        MortalityEntry {
            age: 73,
            qx: 0.01717,
        },
        MortalityEntry {
            age: 74,
            qx: 0.01928,
        },
        MortalityEntry {
            age: 75,
            qx: 0.0217,
        },
        MortalityEntry {
            age: 76,
            qx: 0.02447,
        },
        MortalityEntry {
            age: 77,
            qx: 0.02761,
        },
        MortalityEntry {
            age: 78,
            qx: 0.0312,
        },
        MortalityEntry {
            age: 79,
            qx: 0.03528,
        },
        MortalityEntry {
            age: 80,
            qx: 0.04015,
        },
        MortalityEntry {
            age: 81,
            qx: 0.04512,
        },
        MortalityEntry {
            age: 82,
            qx: 0.0507,
        },
        MortalityEntry {
            age: 83,
            qx: 0.05697,
        },
        MortalityEntry {
            age: 84,
            qx: 0.06407,
        },
        MortalityEntry {
            age: 85,
            qx: 0.07216,
        },
        MortalityEntry {
            age: 86,
            qx: 0.08139,
        },
        MortalityEntry {
            age: 87,
            qx: 0.09182,
        },
        MortalityEntry {
            age: 88,
            qx: 0.10363,
        },
        MortalityEntry {
            age: 89,
            qx: 0.11672,
        },
        MortalityEntry {
            age: 90,
            qx: 0.13112,
        },
        MortalityEntry {
            age: 91,
            qx: 0.14617,
        },
        MortalityEntry {
            age: 92,
            qx: 0.1617,
        },
        MortalityEntry {
            age: 93,
            qx: 0.17767,
        },
        MortalityEntry {
            age: 94,
            qx: 0.19383,
        },
        MortalityEntry {
            age: 95,
            qx: 0.21015,
        },
        MortalityEntry {
            age: 96,
            qx: 0.22749,
        },
        MortalityEntry {
            age: 97,
            qx: 0.24529,
        },
        MortalityEntry {
            age: 98,
            qx: 0.26363,
        },
        MortalityEntry {
            age: 99,
            qx: 0.28252,
        },
        MortalityEntry {
            age: 100,
            qx: 0.30178,
        },
    ]
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

        assert!((find(50) - 0.0009).abs() < 1e-6);
        assert!((find(65) - 0.00729).abs() < 1e-6);
        assert!((find(100) - 0.30178).abs() < 1e-6);
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
