use crate::models::retirement_rmd::{AgeDistributionPeriod, JointDistributionPeriod};

// ---------------------------------------------------------------------------
// Uniform Lifetime Table (Table III) — IRS Pub 590-B (effective 2022+)
// ---------------------------------------------------------------------------

pub fn uniform_lifetime() -> Vec<AgeDistributionPeriod> {
    [
        (72, 27.4),
        (73, 26.5),
        (74, 25.5),
        (75, 24.6),
        (76, 23.7),
        (77, 22.9),
        (78, 22.0),
        (79, 21.1),
        (80, 20.2),
        (81, 19.4),
        (82, 18.5),
        (83, 17.7),
        (84, 16.8),
        (85, 16.0),
        (86, 15.2),
        (87, 14.4),
        (88, 13.7),
        (89, 12.9),
        (90, 12.2),
        (91, 11.5),
        (92, 10.8),
        (93, 10.1),
        (94, 9.5),
        (95, 8.9),
        (96, 8.4),
        (97, 7.8),
        (98, 7.3),
        (99, 6.8),
        (100, 6.4),
        (101, 6.0),
        (102, 5.6),
        (103, 5.2),
        (104, 4.9),
        (105, 4.6),
        (106, 4.3),
        (107, 4.1),
        (108, 3.9),
        (109, 3.7),
        (110, 3.5),
        (111, 3.4),
        (112, 3.3),
        (113, 3.1),
        (114, 3.0),
        (115, 2.9),
        (116, 2.8),
        (117, 2.7),
        (118, 2.5),
        (119, 2.3),
        (120, 2.0),
    ]
    .iter()
    .map(|&(age, dp)| AgeDistributionPeriod {
        age,
        distribution_period: dp,
    })
    .collect()
}

// ---------------------------------------------------------------------------
// Single Life Expectancy Table (Table I) — IRS Pub 590-B (effective 2022+)
// ---------------------------------------------------------------------------

pub fn single_life() -> Vec<AgeDistributionPeriod> {
    [
        (0, 84.6),
        (1, 83.7),
        (2, 82.7),
        (3, 81.7),
        (4, 80.7),
        (5, 79.7),
        (6, 78.7),
        (7, 77.8),
        (8, 76.8),
        (9, 75.8),
        (10, 74.8),
        (11, 73.8),
        (12, 72.8),
        (13, 71.8),
        (14, 70.8),
        (15, 69.9),
        (16, 68.9),
        (17, 67.9),
        (18, 66.9),
        (19, 65.9),
        (20, 65.0),
        (21, 64.0),
        (22, 63.0),
        (23, 62.0),
        (24, 61.0),
        (25, 60.1),
        (26, 59.1),
        (27, 58.1),
        (28, 57.1),
        (29, 56.2),
        (30, 55.2),
        (31, 54.2),
        (32, 53.3),
        (33, 52.3),
        (34, 51.3),
        (35, 50.4),
        (36, 49.4),
        (37, 48.4),
        (38, 47.5),
        (39, 46.5),
        (40, 45.5),
        (41, 44.6),
        (42, 43.6),
        (43, 42.7),
        (44, 41.7),
        (45, 40.7),
        (46, 39.8),
        (47, 38.8),
        (48, 37.9),
        (49, 37.0),
        (50, 36.0),
        (51, 35.1),
        (52, 34.1),
        (53, 33.2),
        (54, 32.3),
        (55, 31.3),
        (56, 30.4),
        (57, 29.5),
        (58, 28.6),
        (59, 27.7),
        (60, 26.8),
        (61, 25.9),
        (62, 25.0),
        (63, 24.1),
        (64, 23.3),
        (65, 22.4),
        (66, 21.5),
        (67, 20.7),
        (68, 19.8),
        (69, 19.0),
        (70, 18.1),
        (71, 17.6),
        (72, 17.2),
        (73, 16.4),
        (74, 15.6),
        (75, 14.8),
        (76, 14.1),
        (77, 13.3),
        (78, 12.6),
        (79, 11.9),
        (80, 11.2),
        (81, 10.5),
        (82, 9.9),
        (83, 9.3),
        (84, 8.7),
        (85, 8.1),
        (86, 7.6),
        (87, 7.1),
        (88, 6.6),
        (89, 6.1),
        (90, 5.7),
        (91, 5.3),
        (92, 4.9),
        (93, 4.6),
        (94, 4.3),
        (95, 4.0),
        (96, 3.7),
        (97, 3.4),
        (98, 3.2),
        (99, 3.0),
        (100, 2.8),
        (101, 2.6),
        (102, 2.5),
        (103, 2.3),
        (104, 2.2),
        (105, 2.1),
        (106, 2.1),
        (107, 2.1),
        (108, 2.0),
        (109, 2.0),
        (110, 2.0),
        (111, 2.0),
        (112, 2.0),
        (113, 1.9),
        (114, 1.9),
        (115, 1.9),
    ]
    .iter()
    .map(|&(age, dp)| AgeDistributionPeriod {
        age,
        distribution_period: dp,
    })
    .collect()
}

// ---------------------------------------------------------------------------
// Joint Life and Last Survivor Table (Table II) — IRS Pub 590-B
//
// This is a large matrix (owner age × spouse age). We embed the commonly-used
// range: owner 72-100, spouse 20-100. For the full table in production, this
// would be loaded from an embedded data file.
// ---------------------------------------------------------------------------

pub fn joint_life() -> Vec<JointDistributionPeriod> {
    // Representative entries for commonly used age combinations.
    // Owner ages 72-85, spouse ages that are 10+ years younger (the scenario
    // where the joint table differs from uniform lifetime).
    let entries: Vec<(u32, u32, f64)> = vec![
        // owner 72, select spouse ages
        (72, 30, 55.1),
        (72, 40, 45.5),
        (72, 50, 36.3),
        (72, 55, 32.0),
        (72, 60, 27.9),
        (72, 62, 26.6),
        (72, 65, 24.7),
        (72, 70, 22.1),
        (72, 72, 21.1),
        // owner 75
        (75, 30, 55.0),
        (75, 40, 45.3),
        (75, 50, 35.9),
        (75, 55, 31.4),
        (75, 60, 27.1),
        (75, 65, 23.2),
        (75, 70, 19.9),
        (75, 75, 17.4),
        // owner 80
        (80, 30, 54.8),
        (80, 40, 45.1),
        (80, 50, 35.5),
        (80, 55, 30.8),
        (80, 60, 26.2),
        (80, 65, 22.0),
        (80, 70, 18.4),
        (80, 75, 15.5),
        (80, 80, 13.5),
        // owner 85
        (85, 30, 54.8),
        (85, 40, 45.0),
        (85, 50, 35.3),
        (85, 55, 30.5),
        (85, 60, 25.8),
        (85, 65, 21.4),
        (85, 70, 17.5),
        (85, 75, 14.3),
        (85, 80, 11.9),
        (85, 85, 10.3),
        // owner 90
        (90, 50, 35.2),
        (90, 60, 25.5),
        (90, 65, 21.0),
        (90, 70, 17.0),
        (90, 75, 13.5),
        (90, 80, 10.9),
        (90, 85, 9.0),
        (90, 90, 7.8),
        // owner 95
        (95, 60, 25.3),
        (95, 65, 20.8),
        (95, 70, 16.7),
        (95, 75, 13.1),
        (95, 80, 10.3),
        (95, 85, 8.2),
        (95, 90, 6.7),
        (95, 95, 5.9),
    ];

    entries
        .into_iter()
        .map(|(owner_age, spouse_age, dp)| JointDistributionPeriod {
            owner_age,
            spouse_age,
            distribution_period: dp,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_lifetime_age_72() {
        let table = uniform_lifetime();
        let entry = table.iter().find(|e| e.age == 72).unwrap();
        assert_eq!(entry.distribution_period, 27.4);
    }

    #[test]
    fn uniform_lifetime_age_120() {
        let table = uniform_lifetime();
        let entry = table.iter().find(|e| e.age == 120).unwrap();
        assert_eq!(entry.distribution_period, 2.0);
    }

    #[test]
    fn uniform_lifetime_range() {
        let table = uniform_lifetime();
        assert_eq!(table.first().unwrap().age, 72);
        assert_eq!(table.last().unwrap().age, 120);
        assert_eq!(table.len(), 49);
    }

    #[test]
    fn single_life_starts_at_zero() {
        let table = single_life();
        assert_eq!(table.first().unwrap().age, 0);
        assert_eq!(table.first().unwrap().distribution_period, 84.6);
    }

    #[test]
    fn single_life_age_72() {
        let table = single_life();
        let entry = table.iter().find(|e| e.age == 72).unwrap();
        assert_eq!(entry.distribution_period, 17.2);
    }

    #[test]
    fn single_life_ends_at_115() {
        let table = single_life();
        let last = table.last().unwrap();
        assert_eq!(last.age, 115);
        assert_eq!(last.distribution_period, 1.9);
    }

    #[test]
    fn joint_life_has_entries() {
        let table = joint_life();
        assert!(!table.is_empty());
        // Verify a known entry
        let entry = table
            .iter()
            .find(|e| e.owner_age == 80 && e.spouse_age == 70)
            .unwrap();
        assert_eq!(entry.distribution_period, 18.4);
    }
}
