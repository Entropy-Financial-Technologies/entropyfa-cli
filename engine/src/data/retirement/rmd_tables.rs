use crate::models::retirement_rmd::{AgeDistributionPeriod, JointDistributionPeriod};

// ---------------------------------------------------------------------------
// Uniform Lifetime Table (Table III) — IRS Pub 590-B (2026, reviewed artifact)
// ---------------------------------------------------------------------------

pub fn uniform_lifetime() -> Vec<AgeDistributionPeriod> {
    vec![
        AgeDistributionPeriod {
            age: 72,
            distribution_period: 27.4,
        },
        AgeDistributionPeriod {
            age: 73,
            distribution_period: 26.5,
        },
        AgeDistributionPeriod {
            age: 74,
            distribution_period: 25.5,
        },
        AgeDistributionPeriod {
            age: 75,
            distribution_period: 24.6,
        },
        AgeDistributionPeriod {
            age: 76,
            distribution_period: 23.7,
        },
        AgeDistributionPeriod {
            age: 77,
            distribution_period: 22.9,
        },
        AgeDistributionPeriod {
            age: 78,
            distribution_period: 22.0,
        },
        AgeDistributionPeriod {
            age: 79,
            distribution_period: 21.1,
        },
        AgeDistributionPeriod {
            age: 80,
            distribution_period: 20.2,
        },
        AgeDistributionPeriod {
            age: 81,
            distribution_period: 19.4,
        },
        AgeDistributionPeriod {
            age: 82,
            distribution_period: 18.5,
        },
        AgeDistributionPeriod {
            age: 83,
            distribution_period: 17.7,
        },
        AgeDistributionPeriod {
            age: 84,
            distribution_period: 16.8,
        },
        AgeDistributionPeriod {
            age: 85,
            distribution_period: 16.0,
        },
        AgeDistributionPeriod {
            age: 86,
            distribution_period: 15.2,
        },
        AgeDistributionPeriod {
            age: 87,
            distribution_period: 14.4,
        },
        AgeDistributionPeriod {
            age: 88,
            distribution_period: 13.7,
        },
        AgeDistributionPeriod {
            age: 89,
            distribution_period: 12.9,
        },
        AgeDistributionPeriod {
            age: 90,
            distribution_period: 12.2,
        },
        AgeDistributionPeriod {
            age: 91,
            distribution_period: 11.5,
        },
        AgeDistributionPeriod {
            age: 92,
            distribution_period: 10.8,
        },
        AgeDistributionPeriod {
            age: 93,
            distribution_period: 10.1,
        },
        AgeDistributionPeriod {
            age: 94,
            distribution_period: 9.5,
        },
        AgeDistributionPeriod {
            age: 95,
            distribution_period: 8.9,
        },
        AgeDistributionPeriod {
            age: 96,
            distribution_period: 8.4,
        },
        AgeDistributionPeriod {
            age: 97,
            distribution_period: 7.8,
        },
        AgeDistributionPeriod {
            age: 98,
            distribution_period: 7.3,
        },
        AgeDistributionPeriod {
            age: 99,
            distribution_period: 6.8,
        },
        AgeDistributionPeriod {
            age: 100,
            distribution_period: 6.4,
        },
        AgeDistributionPeriod {
            age: 101,
            distribution_period: 6.0,
        },
        AgeDistributionPeriod {
            age: 102,
            distribution_period: 5.6,
        },
        AgeDistributionPeriod {
            age: 103,
            distribution_period: 5.2,
        },
        AgeDistributionPeriod {
            age: 104,
            distribution_period: 4.9,
        },
        AgeDistributionPeriod {
            age: 105,
            distribution_period: 4.6,
        },
        AgeDistributionPeriod {
            age: 106,
            distribution_period: 4.3,
        },
        AgeDistributionPeriod {
            age: 107,
            distribution_period: 4.1,
        },
        AgeDistributionPeriod {
            age: 108,
            distribution_period: 3.9,
        },
        AgeDistributionPeriod {
            age: 109,
            distribution_period: 3.7,
        },
        AgeDistributionPeriod {
            age: 110,
            distribution_period: 3.5,
        },
        AgeDistributionPeriod {
            age: 111,
            distribution_period: 3.4,
        },
        AgeDistributionPeriod {
            age: 112,
            distribution_period: 3.3,
        },
        AgeDistributionPeriod {
            age: 113,
            distribution_period: 3.1,
        },
        AgeDistributionPeriod {
            age: 114,
            distribution_period: 3.0,
        },
        AgeDistributionPeriod {
            age: 115,
            distribution_period: 2.9,
        },
        AgeDistributionPeriod {
            age: 116,
            distribution_period: 2.8,
        },
        AgeDistributionPeriod {
            age: 117,
            distribution_period: 2.7,
        },
        AgeDistributionPeriod {
            age: 118,
            distribution_period: 2.5,
        },
        AgeDistributionPeriod {
            age: 119,
            distribution_period: 2.3,
        },
        AgeDistributionPeriod {
            age: 120,
            distribution_period: 2.0,
        },
    ]
}

// ---------------------------------------------------------------------------
// Single Life Expectancy Table (Table I) — IRS Pub 590-B (2026, reviewed artifact)
// ---------------------------------------------------------------------------

pub fn single_life() -> Vec<AgeDistributionPeriod> {
    vec![
        AgeDistributionPeriod {
            age: 0,
            distribution_period: 84.6,
        },
        AgeDistributionPeriod {
            age: 1,
            distribution_period: 83.7,
        },
        AgeDistributionPeriod {
            age: 2,
            distribution_period: 82.8,
        },
        AgeDistributionPeriod {
            age: 3,
            distribution_period: 81.8,
        },
        AgeDistributionPeriod {
            age: 4,
            distribution_period: 80.8,
        },
        AgeDistributionPeriod {
            age: 5,
            distribution_period: 79.8,
        },
        AgeDistributionPeriod {
            age: 6,
            distribution_period: 78.8,
        },
        AgeDistributionPeriod {
            age: 7,
            distribution_period: 77.9,
        },
        AgeDistributionPeriod {
            age: 8,
            distribution_period: 76.9,
        },
        AgeDistributionPeriod {
            age: 9,
            distribution_period: 75.9,
        },
        AgeDistributionPeriod {
            age: 10,
            distribution_period: 74.9,
        },
        AgeDistributionPeriod {
            age: 11,
            distribution_period: 73.9,
        },
        AgeDistributionPeriod {
            age: 12,
            distribution_period: 72.9,
        },
        AgeDistributionPeriod {
            age: 13,
            distribution_period: 71.9,
        },
        AgeDistributionPeriod {
            age: 14,
            distribution_period: 70.9,
        },
        AgeDistributionPeriod {
            age: 15,
            distribution_period: 69.9,
        },
        AgeDistributionPeriod {
            age: 16,
            distribution_period: 69.0,
        },
        AgeDistributionPeriod {
            age: 17,
            distribution_period: 68.0,
        },
        AgeDistributionPeriod {
            age: 18,
            distribution_period: 67.0,
        },
        AgeDistributionPeriod {
            age: 19,
            distribution_period: 66.0,
        },
        AgeDistributionPeriod {
            age: 20,
            distribution_period: 65.0,
        },
        AgeDistributionPeriod {
            age: 21,
            distribution_period: 64.1,
        },
        AgeDistributionPeriod {
            age: 22,
            distribution_period: 63.1,
        },
        AgeDistributionPeriod {
            age: 23,
            distribution_period: 62.1,
        },
        AgeDistributionPeriod {
            age: 24,
            distribution_period: 61.1,
        },
        AgeDistributionPeriod {
            age: 25,
            distribution_period: 60.2,
        },
        AgeDistributionPeriod {
            age: 26,
            distribution_period: 59.2,
        },
        AgeDistributionPeriod {
            age: 27,
            distribution_period: 58.2,
        },
        AgeDistributionPeriod {
            age: 28,
            distribution_period: 57.3,
        },
        AgeDistributionPeriod {
            age: 29,
            distribution_period: 56.3,
        },
        AgeDistributionPeriod {
            age: 30,
            distribution_period: 55.3,
        },
        AgeDistributionPeriod {
            age: 31,
            distribution_period: 54.4,
        },
        AgeDistributionPeriod {
            age: 32,
            distribution_period: 53.4,
        },
        AgeDistributionPeriod {
            age: 33,
            distribution_period: 52.5,
        },
        AgeDistributionPeriod {
            age: 34,
            distribution_period: 51.5,
        },
        AgeDistributionPeriod {
            age: 35,
            distribution_period: 50.5,
        },
        AgeDistributionPeriod {
            age: 36,
            distribution_period: 49.6,
        },
        AgeDistributionPeriod {
            age: 37,
            distribution_period: 48.6,
        },
        AgeDistributionPeriod {
            age: 38,
            distribution_period: 47.7,
        },
        AgeDistributionPeriod {
            age: 39,
            distribution_period: 46.7,
        },
        AgeDistributionPeriod {
            age: 40,
            distribution_period: 45.7,
        },
        AgeDistributionPeriod {
            age: 41,
            distribution_period: 44.8,
        },
        AgeDistributionPeriod {
            age: 42,
            distribution_period: 43.8,
        },
        AgeDistributionPeriod {
            age: 43,
            distribution_period: 42.9,
        },
        AgeDistributionPeriod {
            age: 44,
            distribution_period: 41.9,
        },
        AgeDistributionPeriod {
            age: 45,
            distribution_period: 41.0,
        },
        AgeDistributionPeriod {
            age: 46,
            distribution_period: 40.0,
        },
        AgeDistributionPeriod {
            age: 47,
            distribution_period: 39.0,
        },
        AgeDistributionPeriod {
            age: 48,
            distribution_period: 38.1,
        },
        AgeDistributionPeriod {
            age: 49,
            distribution_period: 37.1,
        },
        AgeDistributionPeriod {
            age: 50,
            distribution_period: 36.2,
        },
        AgeDistributionPeriod {
            age: 51,
            distribution_period: 35.3,
        },
        AgeDistributionPeriod {
            age: 52,
            distribution_period: 34.3,
        },
        AgeDistributionPeriod {
            age: 53,
            distribution_period: 33.4,
        },
        AgeDistributionPeriod {
            age: 54,
            distribution_period: 32.5,
        },
        AgeDistributionPeriod {
            age: 55,
            distribution_period: 31.6,
        },
        AgeDistributionPeriod {
            age: 56,
            distribution_period: 30.6,
        },
        AgeDistributionPeriod {
            age: 57,
            distribution_period: 29.8,
        },
        AgeDistributionPeriod {
            age: 58,
            distribution_period: 28.9,
        },
        AgeDistributionPeriod {
            age: 59,
            distribution_period: 28.0,
        },
        AgeDistributionPeriod {
            age: 60,
            distribution_period: 27.1,
        },
        AgeDistributionPeriod {
            age: 61,
            distribution_period: 26.2,
        },
        AgeDistributionPeriod {
            age: 62,
            distribution_period: 25.4,
        },
        AgeDistributionPeriod {
            age: 63,
            distribution_period: 24.5,
        },
        AgeDistributionPeriod {
            age: 64,
            distribution_period: 23.7,
        },
        AgeDistributionPeriod {
            age: 65,
            distribution_period: 22.9,
        },
        AgeDistributionPeriod {
            age: 66,
            distribution_period: 22.0,
        },
        AgeDistributionPeriod {
            age: 67,
            distribution_period: 21.2,
        },
        AgeDistributionPeriod {
            age: 68,
            distribution_period: 20.4,
        },
        AgeDistributionPeriod {
            age: 69,
            distribution_period: 19.6,
        },
        AgeDistributionPeriod {
            age: 70,
            distribution_period: 18.8,
        },
        AgeDistributionPeriod {
            age: 71,
            distribution_period: 18.0,
        },
        AgeDistributionPeriod {
            age: 72,
            distribution_period: 17.2,
        },
        AgeDistributionPeriod {
            age: 73,
            distribution_period: 16.4,
        },
        AgeDistributionPeriod {
            age: 74,
            distribution_period: 15.6,
        },
        AgeDistributionPeriod {
            age: 75,
            distribution_period: 14.8,
        },
        AgeDistributionPeriod {
            age: 76,
            distribution_period: 14.1,
        },
        AgeDistributionPeriod {
            age: 77,
            distribution_period: 13.3,
        },
        AgeDistributionPeriod {
            age: 78,
            distribution_period: 12.6,
        },
        AgeDistributionPeriod {
            age: 79,
            distribution_period: 11.9,
        },
        AgeDistributionPeriod {
            age: 80,
            distribution_period: 11.2,
        },
        AgeDistributionPeriod {
            age: 81,
            distribution_period: 10.5,
        },
        AgeDistributionPeriod {
            age: 82,
            distribution_period: 9.9,
        },
        AgeDistributionPeriod {
            age: 83,
            distribution_period: 9.3,
        },
        AgeDistributionPeriod {
            age: 84,
            distribution_period: 8.7,
        },
        AgeDistributionPeriod {
            age: 85,
            distribution_period: 8.1,
        },
        AgeDistributionPeriod {
            age: 86,
            distribution_period: 7.6,
        },
        AgeDistributionPeriod {
            age: 87,
            distribution_period: 7.1,
        },
        AgeDistributionPeriod {
            age: 88,
            distribution_period: 6.6,
        },
        AgeDistributionPeriod {
            age: 89,
            distribution_period: 6.1,
        },
        AgeDistributionPeriod {
            age: 90,
            distribution_period: 5.7,
        },
        AgeDistributionPeriod {
            age: 91,
            distribution_period: 5.3,
        },
        AgeDistributionPeriod {
            age: 92,
            distribution_period: 4.9,
        },
        AgeDistributionPeriod {
            age: 93,
            distribution_period: 4.6,
        },
        AgeDistributionPeriod {
            age: 94,
            distribution_period: 4.3,
        },
        AgeDistributionPeriod {
            age: 95,
            distribution_period: 4.0,
        },
        AgeDistributionPeriod {
            age: 96,
            distribution_period: 3.7,
        },
        AgeDistributionPeriod {
            age: 97,
            distribution_period: 3.4,
        },
        AgeDistributionPeriod {
            age: 98,
            distribution_period: 3.2,
        },
        AgeDistributionPeriod {
            age: 99,
            distribution_period: 3.0,
        },
        AgeDistributionPeriod {
            age: 100,
            distribution_period: 2.8,
        },
        AgeDistributionPeriod {
            age: 101,
            distribution_period: 2.6,
        },
        AgeDistributionPeriod {
            age: 102,
            distribution_period: 2.5,
        },
        AgeDistributionPeriod {
            age: 103,
            distribution_period: 2.3,
        },
        AgeDistributionPeriod {
            age: 104,
            distribution_period: 2.2,
        },
        AgeDistributionPeriod {
            age: 105,
            distribution_period: 2.1,
        },
        AgeDistributionPeriod {
            age: 106,
            distribution_period: 2.1,
        },
        AgeDistributionPeriod {
            age: 107,
            distribution_period: 2.1,
        },
        AgeDistributionPeriod {
            age: 108,
            distribution_period: 2.0,
        },
        AgeDistributionPeriod {
            age: 109,
            distribution_period: 2.0,
        },
        AgeDistributionPeriod {
            age: 110,
            distribution_period: 2.0,
        },
        AgeDistributionPeriod {
            age: 111,
            distribution_period: 2.0,
        },
        AgeDistributionPeriod {
            age: 112,
            distribution_period: 2.0,
        },
        AgeDistributionPeriod {
            age: 113,
            distribution_period: 1.9,
        },
        AgeDistributionPeriod {
            age: 114,
            distribution_period: 1.9,
        },
        AgeDistributionPeriod {
            age: 115,
            distribution_period: 1.8,
        },
        AgeDistributionPeriod {
            age: 116,
            distribution_period: 1.8,
        },
        AgeDistributionPeriod {
            age: 117,
            distribution_period: 1.6,
        },
        AgeDistributionPeriod {
            age: 118,
            distribution_period: 1.4,
        },
        AgeDistributionPeriod {
            age: 119,
            distribution_period: 1.1,
        },
        AgeDistributionPeriod {
            age: 120,
            distribution_period: 1.0,
        },
    ]
}

// ---------------------------------------------------------------------------
// Joint Life and Last Survivor Table (Table II) — IRS Pub 590-B (2026, reviewed artifact)
// ---------------------------------------------------------------------------

pub fn joint_life() -> Vec<JointDistributionPeriod> {
    vec![
        JointDistributionPeriod {
            owner_age: 20,
            spouse_age: 20,
            distribution_period: 72.0,
        },
        JointDistributionPeriod {
            owner_age: 20,
            spouse_age: 21,
            distribution_period: 71.5,
        },
        JointDistributionPeriod {
            owner_age: 20,
            spouse_age: 22,
            distribution_period: 71.0,
        },
        JointDistributionPeriod {
            owner_age: 20,
            spouse_age: 23,
            distribution_period: 70.6,
        },
        JointDistributionPeriod {
            owner_age: 20,
            spouse_age: 24,
            distribution_period: 70.2,
        },
        JointDistributionPeriod {
            owner_age: 20,
            spouse_age: 25,
            distribution_period: 69.8,
        },
        JointDistributionPeriod {
            owner_age: 20,
            spouse_age: 26,
            distribution_period: 69.5,
        },
        JointDistributionPeriod {
            owner_age: 20,
            spouse_age: 27,
            distribution_period: 69.1,
        },
        JointDistributionPeriod {
            owner_age: 20,
            spouse_age: 28,
            distribution_period: 68.8,
        },
        JointDistributionPeriod {
            owner_age: 20,
            spouse_age: 29,
            distribution_period: 68.5,
        },
        JointDistributionPeriod {
            owner_age: 21,
            spouse_age: 20,
            distribution_period: 71.5,
        },
        JointDistributionPeriod {
            owner_age: 21,
            spouse_age: 21,
            distribution_period: 71.0,
        },
        JointDistributionPeriod {
            owner_age: 21,
            spouse_age: 22,
            distribution_period: 70.5,
        },
        JointDistributionPeriod {
            owner_age: 21,
            spouse_age: 23,
            distribution_period: 70.0,
        },
        JointDistributionPeriod {
            owner_age: 21,
            spouse_age: 24,
            distribution_period: 69.6,
        },
        JointDistributionPeriod {
            owner_age: 21,
            spouse_age: 25,
            distribution_period: 69.2,
        },
        JointDistributionPeriod {
            owner_age: 21,
            spouse_age: 26,
            distribution_period: 68.8,
        },
        JointDistributionPeriod {
            owner_age: 21,
            spouse_age: 27,
            distribution_period: 68.5,
        },
        JointDistributionPeriod {
            owner_age: 21,
            spouse_age: 28,
            distribution_period: 68.1,
        },
        JointDistributionPeriod {
            owner_age: 21,
            spouse_age: 29,
            distribution_period: 67.8,
        },
        JointDistributionPeriod {
            owner_age: 22,
            spouse_age: 20,
            distribution_period: 71.0,
        },
        JointDistributionPeriod {
            owner_age: 22,
            spouse_age: 21,
            distribution_period: 70.5,
        },
        JointDistributionPeriod {
            owner_age: 22,
            spouse_age: 22,
            distribution_period: 70.0,
        },
        JointDistributionPeriod {
            owner_age: 22,
            spouse_age: 23,
            distribution_period: 69.5,
        },
        JointDistributionPeriod {
            owner_age: 22,
            spouse_age: 24,
            distribution_period: 69.0,
        },
        JointDistributionPeriod {
            owner_age: 22,
            spouse_age: 25,
            distribution_period: 68.6,
        },
        JointDistributionPeriod {
            owner_age: 22,
            spouse_age: 26,
            distribution_period: 68.2,
        },
        JointDistributionPeriod {
            owner_age: 22,
            spouse_age: 27,
            distribution_period: 67.8,
        },
        JointDistributionPeriod {
            owner_age: 22,
            spouse_age: 28,
            distribution_period: 67.5,
        },
        JointDistributionPeriod {
            owner_age: 22,
            spouse_age: 29,
            distribution_period: 67.1,
        },
        JointDistributionPeriod {
            owner_age: 23,
            spouse_age: 20,
            distribution_period: 70.6,
        },
        JointDistributionPeriod {
            owner_age: 23,
            spouse_age: 21,
            distribution_period: 70.0,
        },
        JointDistributionPeriod {
            owner_age: 23,
            spouse_age: 22,
            distribution_period: 69.5,
        },
        JointDistributionPeriod {
            owner_age: 23,
            spouse_age: 23,
            distribution_period: 69.0,
        },
        JointDistributionPeriod {
            owner_age: 23,
            spouse_age: 24,
            distribution_period: 68.5,
        },
        JointDistributionPeriod {
            owner_age: 23,
            spouse_age: 25,
            distribution_period: 68.0,
        },
        JointDistributionPeriod {
            owner_age: 23,
            spouse_age: 26,
            distribution_period: 67.6,
        },
        JointDistributionPeriod {
            owner_age: 23,
            spouse_age: 27,
            distribution_period: 67.2,
        },
        JointDistributionPeriod {
            owner_age: 23,
            spouse_age: 28,
            distribution_period: 66.8,
        },
        JointDistributionPeriod {
            owner_age: 23,
            spouse_age: 29,
            distribution_period: 66.5,
        },
        JointDistributionPeriod {
            owner_age: 24,
            spouse_age: 20,
            distribution_period: 70.2,
        },
        JointDistributionPeriod {
            owner_age: 24,
            spouse_age: 21,
            distribution_period: 69.6,
        },
        JointDistributionPeriod {
            owner_age: 24,
            spouse_age: 22,
            distribution_period: 69.0,
        },
        JointDistributionPeriod {
            owner_age: 24,
            spouse_age: 23,
            distribution_period: 68.5,
        },
        JointDistributionPeriod {
            owner_age: 24,
            spouse_age: 24,
            distribution_period: 68.0,
        },
        JointDistributionPeriod {
            owner_age: 24,
            spouse_age: 25,
            distribution_period: 67.5,
        },
        JointDistributionPeriod {
            owner_age: 24,
            spouse_age: 26,
            distribution_period: 67.1,
        },
        JointDistributionPeriod {
            owner_age: 24,
            spouse_age: 27,
            distribution_period: 66.6,
        },
        JointDistributionPeriod {
            owner_age: 24,
            spouse_age: 28,
            distribution_period: 66.2,
        },
        JointDistributionPeriod {
            owner_age: 24,
            spouse_age: 29,
            distribution_period: 65.8,
        },
        JointDistributionPeriod {
            owner_age: 25,
            spouse_age: 20,
            distribution_period: 69.8,
        },
        JointDistributionPeriod {
            owner_age: 25,
            spouse_age: 21,
            distribution_period: 69.2,
        },
        JointDistributionPeriod {
            owner_age: 25,
            spouse_age: 22,
            distribution_period: 68.6,
        },
        JointDistributionPeriod {
            owner_age: 25,
            spouse_age: 23,
            distribution_period: 68.0,
        },
        JointDistributionPeriod {
            owner_age: 25,
            spouse_age: 24,
            distribution_period: 67.5,
        },
        JointDistributionPeriod {
            owner_age: 25,
            spouse_age: 25,
            distribution_period: 67.0,
        },
        JointDistributionPeriod {
            owner_age: 25,
            spouse_age: 26,
            distribution_period: 66.5,
        },
        JointDistributionPeriod {
            owner_age: 25,
            spouse_age: 27,
            distribution_period: 66.1,
        },
        JointDistributionPeriod {
            owner_age: 25,
            spouse_age: 28,
            distribution_period: 65.6,
        },
        JointDistributionPeriod {
            owner_age: 25,
            spouse_age: 29,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 26,
            spouse_age: 20,
            distribution_period: 69.5,
        },
        JointDistributionPeriod {
            owner_age: 26,
            spouse_age: 21,
            distribution_period: 68.8,
        },
        JointDistributionPeriod {
            owner_age: 26,
            spouse_age: 22,
            distribution_period: 68.2,
        },
        JointDistributionPeriod {
            owner_age: 26,
            spouse_age: 23,
            distribution_period: 67.6,
        },
        JointDistributionPeriod {
            owner_age: 26,
            spouse_age: 24,
            distribution_period: 67.1,
        },
        JointDistributionPeriod {
            owner_age: 26,
            spouse_age: 25,
            distribution_period: 66.5,
        },
        JointDistributionPeriod {
            owner_age: 26,
            spouse_age: 26,
            distribution_period: 66.0,
        },
        JointDistributionPeriod {
            owner_age: 26,
            spouse_age: 27,
            distribution_period: 65.5,
        },
        JointDistributionPeriod {
            owner_age: 26,
            spouse_age: 28,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 26,
            spouse_age: 29,
            distribution_period: 64.6,
        },
        JointDistributionPeriod {
            owner_age: 27,
            spouse_age: 20,
            distribution_period: 69.1,
        },
        JointDistributionPeriod {
            owner_age: 27,
            spouse_age: 21,
            distribution_period: 68.5,
        },
        JointDistributionPeriod {
            owner_age: 27,
            spouse_age: 22,
            distribution_period: 67.8,
        },
        JointDistributionPeriod {
            owner_age: 27,
            spouse_age: 23,
            distribution_period: 67.2,
        },
        JointDistributionPeriod {
            owner_age: 27,
            spouse_age: 24,
            distribution_period: 66.6,
        },
        JointDistributionPeriod {
            owner_age: 27,
            spouse_age: 25,
            distribution_period: 66.1,
        },
        JointDistributionPeriod {
            owner_age: 27,
            spouse_age: 26,
            distribution_period: 65.5,
        },
        JointDistributionPeriod {
            owner_age: 27,
            spouse_age: 27,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 27,
            spouse_age: 28,
            distribution_period: 64.5,
        },
        JointDistributionPeriod {
            owner_age: 27,
            spouse_age: 29,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 28,
            spouse_age: 20,
            distribution_period: 68.8,
        },
        JointDistributionPeriod {
            owner_age: 28,
            spouse_age: 21,
            distribution_period: 68.1,
        },
        JointDistributionPeriod {
            owner_age: 28,
            spouse_age: 22,
            distribution_period: 67.5,
        },
        JointDistributionPeriod {
            owner_age: 28,
            spouse_age: 23,
            distribution_period: 66.8,
        },
        JointDistributionPeriod {
            owner_age: 28,
            spouse_age: 24,
            distribution_period: 66.2,
        },
        JointDistributionPeriod {
            owner_age: 28,
            spouse_age: 25,
            distribution_period: 65.6,
        },
        JointDistributionPeriod {
            owner_age: 28,
            spouse_age: 26,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 28,
            spouse_age: 27,
            distribution_period: 64.5,
        },
        JointDistributionPeriod {
            owner_age: 28,
            spouse_age: 28,
            distribution_period: 64.0,
        },
        JointDistributionPeriod {
            owner_age: 28,
            spouse_age: 29,
            distribution_period: 63.5,
        },
        JointDistributionPeriod {
            owner_age: 29,
            spouse_age: 20,
            distribution_period: 68.5,
        },
        JointDistributionPeriod {
            owner_age: 29,
            spouse_age: 21,
            distribution_period: 67.8,
        },
        JointDistributionPeriod {
            owner_age: 29,
            spouse_age: 22,
            distribution_period: 67.1,
        },
        JointDistributionPeriod {
            owner_age: 29,
            spouse_age: 23,
            distribution_period: 66.5,
        },
        JointDistributionPeriod {
            owner_age: 29,
            spouse_age: 24,
            distribution_period: 65.8,
        },
        JointDistributionPeriod {
            owner_age: 29,
            spouse_age: 25,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 29,
            spouse_age: 26,
            distribution_period: 64.6,
        },
        JointDistributionPeriod {
            owner_age: 29,
            spouse_age: 27,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 29,
            spouse_age: 28,
            distribution_period: 63.5,
        },
        JointDistributionPeriod {
            owner_age: 29,
            spouse_age: 29,
            distribution_period: 63.0,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 20,
            distribution_period: 68.3,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 21,
            distribution_period: 67.5,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 22,
            distribution_period: 66.8,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 23,
            distribution_period: 66.2,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 24,
            distribution_period: 65.5,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 25,
            distribution_period: 64.9,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 26,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 27,
            distribution_period: 63.7,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 28,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 29,
            distribution_period: 62.6,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 30,
            distribution_period: 62.0,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 31,
            distribution_period: 61.6,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 32,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 33,
            distribution_period: 60.7,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 34,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 35,
            distribution_period: 59.9,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 36,
            distribution_period: 59.5,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 37,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 38,
            distribution_period: 58.9,
        },
        JointDistributionPeriod {
            owner_age: 30,
            spouse_age: 39,
            distribution_period: 58.6,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 20,
            distribution_period: 68.0,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 21,
            distribution_period: 67.3,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 22,
            distribution_period: 66.6,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 23,
            distribution_period: 65.8,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 24,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 25,
            distribution_period: 64.5,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 26,
            distribution_period: 63.9,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 27,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 28,
            distribution_period: 62.7,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 29,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 30,
            distribution_period: 61.6,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 31,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 32,
            distribution_period: 60.6,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 33,
            distribution_period: 60.1,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 34,
            distribution_period: 59.7,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 35,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 36,
            distribution_period: 58.9,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 37,
            distribution_period: 58.6,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 38,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 31,
            spouse_age: 39,
            distribution_period: 57.9,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 20,
            distribution_period: 67.8,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 21,
            distribution_period: 67.0,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 22,
            distribution_period: 66.3,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 23,
            distribution_period: 65.6,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 24,
            distribution_period: 64.9,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 25,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 26,
            distribution_period: 63.5,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 27,
            distribution_period: 62.9,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 28,
            distribution_period: 62.3,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 29,
            distribution_period: 61.7,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 30,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 31,
            distribution_period: 60.6,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 32,
            distribution_period: 60.1,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 33,
            distribution_period: 59.6,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 34,
            distribution_period: 59.1,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 35,
            distribution_period: 58.7,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 36,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 37,
            distribution_period: 57.9,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 38,
            distribution_period: 57.6,
        },
        JointDistributionPeriod {
            owner_age: 32,
            spouse_age: 39,
            distribution_period: 57.2,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 20,
            distribution_period: 67.6,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 21,
            distribution_period: 66.8,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 22,
            distribution_period: 66.0,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 23,
            distribution_period: 65.3,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 24,
            distribution_period: 64.6,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 25,
            distribution_period: 63.9,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 26,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 27,
            distribution_period: 62.5,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 28,
            distribution_period: 61.9,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 29,
            distribution_period: 61.3,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 30,
            distribution_period: 60.7,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 31,
            distribution_period: 60.1,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 32,
            distribution_period: 59.6,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 33,
            distribution_period: 59.1,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 34,
            distribution_period: 58.6,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 35,
            distribution_period: 58.1,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 36,
            distribution_period: 57.7,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 37,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 38,
            distribution_period: 56.9,
        },
        JointDistributionPeriod {
            owner_age: 33,
            spouse_age: 39,
            distribution_period: 56.6,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 20,
            distribution_period: 67.4,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 21,
            distribution_period: 66.6,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 22,
            distribution_period: 65.8,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 23,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 24,
            distribution_period: 64.3,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 25,
            distribution_period: 63.6,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 26,
            distribution_period: 62.9,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 27,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 28,
            distribution_period: 61.5,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 29,
            distribution_period: 60.9,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 30,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 31,
            distribution_period: 59.7,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 32,
            distribution_period: 59.1,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 33,
            distribution_period: 58.6,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 34,
            distribution_period: 58.1,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 35,
            distribution_period: 57.6,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 36,
            distribution_period: 57.2,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 37,
            distribution_period: 56.7,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 38,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 34,
            spouse_age: 39,
            distribution_period: 55.9,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 20,
            distribution_period: 67.2,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 21,
            distribution_period: 66.4,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 22,
            distribution_period: 65.6,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 23,
            distribution_period: 64.8,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 24,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 25,
            distribution_period: 63.3,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 26,
            distribution_period: 62.6,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 27,
            distribution_period: 61.9,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 28,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 29,
            distribution_period: 60.5,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 30,
            distribution_period: 59.9,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 31,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 32,
            distribution_period: 58.7,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 33,
            distribution_period: 58.1,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 34,
            distribution_period: 57.6,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 35,
            distribution_period: 57.1,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 36,
            distribution_period: 56.6,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 37,
            distribution_period: 56.2,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 38,
            distribution_period: 55.7,
        },
        JointDistributionPeriod {
            owner_age: 35,
            spouse_age: 39,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 20,
            distribution_period: 67.1,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 21,
            distribution_period: 66.2,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 22,
            distribution_period: 65.4,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 23,
            distribution_period: 64.6,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 24,
            distribution_period: 63.8,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 25,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 26,
            distribution_period: 62.3,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 27,
            distribution_period: 61.6,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 28,
            distribution_period: 60.9,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 29,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 30,
            distribution_period: 59.5,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 31,
            distribution_period: 58.9,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 32,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 33,
            distribution_period: 57.7,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 34,
            distribution_period: 57.2,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 35,
            distribution_period: 56.6,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 36,
            distribution_period: 56.1,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 37,
            distribution_period: 55.6,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 38,
            distribution_period: 55.2,
        },
        JointDistributionPeriod {
            owner_age: 36,
            spouse_age: 39,
            distribution_period: 54.7,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 20,
            distribution_period: 66.9,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 21,
            distribution_period: 66.1,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 22,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 23,
            distribution_period: 64.4,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 24,
            distribution_period: 63.6,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 25,
            distribution_period: 62.8,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 26,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 27,
            distribution_period: 61.3,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 28,
            distribution_period: 60.6,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 29,
            distribution_period: 59.9,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 30,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 31,
            distribution_period: 58.6,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 32,
            distribution_period: 57.9,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 33,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 34,
            distribution_period: 56.7,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 35,
            distribution_period: 56.2,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 36,
            distribution_period: 55.6,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 37,
            distribution_period: 55.1,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 38,
            distribution_period: 54.6,
        },
        JointDistributionPeriod {
            owner_age: 37,
            spouse_age: 39,
            distribution_period: 54.2,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 20,
            distribution_period: 66.8,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 21,
            distribution_period: 65.9,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 22,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 23,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 24,
            distribution_period: 63.4,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 25,
            distribution_period: 62.6,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 26,
            distribution_period: 61.9,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 27,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 28,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 29,
            distribution_period: 59.6,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 30,
            distribution_period: 58.9,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 31,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 32,
            distribution_period: 57.6,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 33,
            distribution_period: 56.9,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 34,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 35,
            distribution_period: 55.7,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 36,
            distribution_period: 55.2,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 37,
            distribution_period: 54.6,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 38,
            distribution_period: 54.1,
        },
        JointDistributionPeriod {
            owner_age: 38,
            spouse_age: 39,
            distribution_period: 53.6,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 20,
            distribution_period: 66.6,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 21,
            distribution_period: 65.8,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 22,
            distribution_period: 64.9,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 23,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 24,
            distribution_period: 63.3,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 25,
            distribution_period: 62.4,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 26,
            distribution_period: 61.6,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 27,
            distribution_period: 60.9,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 28,
            distribution_period: 60.1,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 29,
            distribution_period: 59.4,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 30,
            distribution_period: 58.6,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 31,
            distribution_period: 57.9,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 32,
            distribution_period: 57.2,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 33,
            distribution_period: 56.6,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 34,
            distribution_period: 55.9,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 35,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 36,
            distribution_period: 54.7,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 37,
            distribution_period: 54.2,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 38,
            distribution_period: 53.6,
        },
        JointDistributionPeriod {
            owner_age: 39,
            spouse_age: 39,
            distribution_period: 53.1,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 20,
            distribution_period: 66.5,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 21,
            distribution_period: 65.6,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 22,
            distribution_period: 64.8,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 23,
            distribution_period: 63.9,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 24,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 25,
            distribution_period: 62.3,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 26,
            distribution_period: 61.5,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 27,
            distribution_period: 60.7,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 28,
            distribution_period: 59.9,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 29,
            distribution_period: 59.1,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 30,
            distribution_period: 58.4,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 31,
            distribution_period: 57.6,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 32,
            distribution_period: 56.9,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 33,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 34,
            distribution_period: 55.6,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 35,
            distribution_period: 55.0,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 36,
            distribution_period: 54.3,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 37,
            distribution_period: 53.8,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 38,
            distribution_period: 53.2,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 39,
            distribution_period: 52.7,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 40,
            distribution_period: 52.2,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 41,
            distribution_period: 51.7,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 42,
            distribution_period: 51.2,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 43,
            distribution_period: 50.8,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 44,
            distribution_period: 50.4,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 45,
            distribution_period: 50.0,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 46,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 47,
            distribution_period: 49.3,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 48,
            distribution_period: 49.0,
        },
        JointDistributionPeriod {
            owner_age: 40,
            spouse_age: 49,
            distribution_period: 48.8,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 20,
            distribution_period: 66.4,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 21,
            distribution_period: 65.5,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 22,
            distribution_period: 64.6,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 23,
            distribution_period: 63.8,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 24,
            distribution_period: 62.9,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 25,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 26,
            distribution_period: 61.3,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 27,
            distribution_period: 60.5,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 28,
            distribution_period: 59.7,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 29,
            distribution_period: 58.9,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 30,
            distribution_period: 58.1,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 31,
            distribution_period: 57.4,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 32,
            distribution_period: 56.7,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 33,
            distribution_period: 56.0,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 34,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 35,
            distribution_period: 54.6,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 36,
            distribution_period: 54.0,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 37,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 38,
            distribution_period: 52.8,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 39,
            distribution_period: 52.2,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 40,
            distribution_period: 51.7,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 41,
            distribution_period: 51.2,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 42,
            distribution_period: 50.7,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 43,
            distribution_period: 50.2,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 44,
            distribution_period: 49.8,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 45,
            distribution_period: 49.4,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 46,
            distribution_period: 49.0,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 47,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 48,
            distribution_period: 48.4,
        },
        JointDistributionPeriod {
            owner_age: 41,
            spouse_age: 49,
            distribution_period: 48.1,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 20,
            distribution_period: 66.3,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 21,
            distribution_period: 65.4,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 22,
            distribution_period: 64.5,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 23,
            distribution_period: 63.6,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 24,
            distribution_period: 62.8,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 25,
            distribution_period: 61.9,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 26,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 27,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 28,
            distribution_period: 59.5,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 29,
            distribution_period: 58.7,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 30,
            distribution_period: 57.9,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 31,
            distribution_period: 57.1,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 32,
            distribution_period: 56.4,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 33,
            distribution_period: 55.7,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 34,
            distribution_period: 55.0,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 35,
            distribution_period: 54.3,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 36,
            distribution_period: 53.6,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 37,
            distribution_period: 53.0,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 38,
            distribution_period: 52.4,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 39,
            distribution_period: 51.8,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 40,
            distribution_period: 51.2,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 41,
            distribution_period: 50.7,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 42,
            distribution_period: 50.2,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 43,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 44,
            distribution_period: 49.2,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 45,
            distribution_period: 48.8,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 46,
            distribution_period: 48.4,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 47,
            distribution_period: 48.0,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 48,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 42,
            spouse_age: 49,
            distribution_period: 47.4,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 20,
            distribution_period: 66.2,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 21,
            distribution_period: 65.3,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 22,
            distribution_period: 64.4,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 23,
            distribution_period: 63.5,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 24,
            distribution_period: 62.7,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 25,
            distribution_period: 61.8,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 26,
            distribution_period: 61.0,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 27,
            distribution_period: 60.1,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 28,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 29,
            distribution_period: 58.5,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 30,
            distribution_period: 57.7,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 31,
            distribution_period: 56.9,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 32,
            distribution_period: 56.2,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 33,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 34,
            distribution_period: 54.7,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 35,
            distribution_period: 54.0,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 36,
            distribution_period: 53.3,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 37,
            distribution_period: 52.6,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 38,
            distribution_period: 52.0,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 39,
            distribution_period: 51.4,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 40,
            distribution_period: 50.8,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 41,
            distribution_period: 50.2,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 42,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 43,
            distribution_period: 49.2,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 44,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 45,
            distribution_period: 48.3,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 46,
            distribution_period: 47.8,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 47,
            distribution_period: 47.4,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 48,
            distribution_period: 47.1,
        },
        JointDistributionPeriod {
            owner_age: 43,
            spouse_age: 49,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 20,
            distribution_period: 66.1,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 21,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 22,
            distribution_period: 64.3,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 23,
            distribution_period: 63.4,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 24,
            distribution_period: 62.5,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 25,
            distribution_period: 61.7,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 26,
            distribution_period: 60.8,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 27,
            distribution_period: 60.0,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 28,
            distribution_period: 59.1,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 29,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 30,
            distribution_period: 57.5,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 31,
            distribution_period: 56.7,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 32,
            distribution_period: 55.9,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 33,
            distribution_period: 55.2,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 34,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 35,
            distribution_period: 53.7,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 36,
            distribution_period: 53.0,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 37,
            distribution_period: 52.3,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 38,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 39,
            distribution_period: 51.0,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 40,
            distribution_period: 50.4,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 41,
            distribution_period: 49.8,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 42,
            distribution_period: 49.2,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 43,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 44,
            distribution_period: 48.2,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 45,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 46,
            distribution_period: 47.3,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 47,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 48,
            distribution_period: 46.4,
        },
        JointDistributionPeriod {
            owner_age: 44,
            spouse_age: 49,
            distribution_period: 46.1,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 20,
            distribution_period: 66.0,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 21,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 22,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 23,
            distribution_period: 63.3,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 24,
            distribution_period: 62.4,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 25,
            distribution_period: 61.5,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 26,
            distribution_period: 60.7,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 27,
            distribution_period: 59.8,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 28,
            distribution_period: 59.0,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 29,
            distribution_period: 58.1,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 30,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 31,
            distribution_period: 56.5,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 32,
            distribution_period: 55.7,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 33,
            distribution_period: 54.9,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 34,
            distribution_period: 54.2,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 35,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 36,
            distribution_period: 52.7,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 37,
            distribution_period: 52.0,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 38,
            distribution_period: 51.3,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 39,
            distribution_period: 50.7,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 40,
            distribution_period: 50.0,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 41,
            distribution_period: 49.4,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 42,
            distribution_period: 48.8,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 43,
            distribution_period: 48.3,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 44,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 45,
            distribution_period: 47.2,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 46,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 47,
            distribution_period: 46.3,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 48,
            distribution_period: 45.9,
        },
        JointDistributionPeriod {
            owner_age: 45,
            spouse_age: 49,
            distribution_period: 45.5,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 20,
            distribution_period: 65.9,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 21,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 22,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 23,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 24,
            distribution_period: 62.3,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 25,
            distribution_period: 61.4,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 26,
            distribution_period: 60.6,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 27,
            distribution_period: 59.7,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 28,
            distribution_period: 58.8,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 29,
            distribution_period: 58.0,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 30,
            distribution_period: 57.2,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 31,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 32,
            distribution_period: 55.5,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 33,
            distribution_period: 54.7,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 34,
            distribution_period: 54.0,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 35,
            distribution_period: 53.2,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 36,
            distribution_period: 52.4,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 37,
            distribution_period: 51.7,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 38,
            distribution_period: 51.0,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 39,
            distribution_period: 50.3,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 40,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 41,
            distribution_period: 49.0,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 42,
            distribution_period: 48.4,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 43,
            distribution_period: 47.8,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 44,
            distribution_period: 47.3,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 45,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 46,
            distribution_period: 46.2,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 47,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 48,
            distribution_period: 45.3,
        },
        JointDistributionPeriod {
            owner_age: 46,
            spouse_age: 49,
            distribution_period: 44.9,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 20,
            distribution_period: 65.9,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 21,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 22,
            distribution_period: 64.0,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 23,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 24,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 25,
            distribution_period: 61.3,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 26,
            distribution_period: 60.5,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 27,
            distribution_period: 59.6,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 28,
            distribution_period: 58.7,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 29,
            distribution_period: 57.9,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 30,
            distribution_period: 57.0,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 31,
            distribution_period: 56.2,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 32,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 33,
            distribution_period: 54.5,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 34,
            distribution_period: 53.7,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 35,
            distribution_period: 53.0,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 36,
            distribution_period: 52.2,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 37,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 38,
            distribution_period: 50.7,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 39,
            distribution_period: 50.0,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 40,
            distribution_period: 49.3,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 41,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 42,
            distribution_period: 48.0,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 43,
            distribution_period: 47.4,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 44,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 45,
            distribution_period: 46.3,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 46,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 47,
            distribution_period: 45.2,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 48,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 47,
            spouse_age: 49,
            distribution_period: 44.3,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 20,
            distribution_period: 65.8,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 21,
            distribution_period: 64.9,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 22,
            distribution_period: 64.0,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 23,
            distribution_period: 63.0,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 24,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 25,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 26,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 27,
            distribution_period: 59.5,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 28,
            distribution_period: 58.6,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 29,
            distribution_period: 57.7,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 30,
            distribution_period: 56.9,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 31,
            distribution_period: 56.0,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 32,
            distribution_period: 55.2,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 33,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 34,
            distribution_period: 53.6,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 35,
            distribution_period: 52.8,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 36,
            distribution_period: 52.0,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 37,
            distribution_period: 51.2,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 38,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 39,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 40,
            distribution_period: 49.0,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 41,
            distribution_period: 48.4,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 42,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 43,
            distribution_period: 47.1,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 44,
            distribution_period: 46.4,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 45,
            distribution_period: 45.9,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 46,
            distribution_period: 45.3,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 47,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 48,
            distribution_period: 44.3,
        },
        JointDistributionPeriod {
            owner_age: 48,
            spouse_age: 49,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 20,
            distribution_period: 65.7,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 21,
            distribution_period: 64.8,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 22,
            distribution_period: 63.9,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 23,
            distribution_period: 63.0,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 24,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 25,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 26,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 27,
            distribution_period: 59.4,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 28,
            distribution_period: 58.5,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 29,
            distribution_period: 57.6,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 30,
            distribution_period: 56.7,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 31,
            distribution_period: 55.9,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 32,
            distribution_period: 55.0,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 33,
            distribution_period: 54.2,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 34,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 35,
            distribution_period: 52.6,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 36,
            distribution_period: 51.8,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 37,
            distribution_period: 51.0,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 38,
            distribution_period: 50.2,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 39,
            distribution_period: 49.5,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 40,
            distribution_period: 48.8,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 41,
            distribution_period: 48.1,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 42,
            distribution_period: 47.4,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 43,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 44,
            distribution_period: 46.1,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 45,
            distribution_period: 45.5,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 46,
            distribution_period: 44.9,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 47,
            distribution_period: 44.3,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 48,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 49,
            spouse_age: 49,
            distribution_period: 43.3,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 20,
            distribution_period: 65.7,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 21,
            distribution_period: 64.8,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 22,
            distribution_period: 63.8,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 23,
            distribution_period: 62.9,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 24,
            distribution_period: 62.0,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 25,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 26,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 27,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 28,
            distribution_period: 58.4,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 29,
            distribution_period: 57.5,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 30,
            distribution_period: 56.6,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 31,
            distribution_period: 55.8,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 32,
            distribution_period: 54.9,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 33,
            distribution_period: 54.1,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 34,
            distribution_period: 53.2,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 35,
            distribution_period: 52.4,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 36,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 37,
            distribution_period: 50.8,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 38,
            distribution_period: 50.0,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 39,
            distribution_period: 49.2,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 40,
            distribution_period: 48.5,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 41,
            distribution_period: 47.8,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 42,
            distribution_period: 47.1,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 43,
            distribution_period: 46.4,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 44,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 45,
            distribution_period: 45.1,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 46,
            distribution_period: 44.5,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 47,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 48,
            distribution_period: 43.3,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 49,
            distribution_period: 42.8,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 50,
            distribution_period: 42.3,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 51,
            distribution_period: 41.8,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 52,
            distribution_period: 41.4,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 53,
            distribution_period: 40.9,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 54,
            distribution_period: 40.6,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 55,
            distribution_period: 40.2,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 56,
            distribution_period: 39.8,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 57,
            distribution_period: 39.5,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 58,
            distribution_period: 39.2,
        },
        JointDistributionPeriod {
            owner_age: 50,
            spouse_age: 59,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 20,
            distribution_period: 65.6,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 21,
            distribution_period: 64.7,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 22,
            distribution_period: 63.8,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 23,
            distribution_period: 62.8,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 24,
            distribution_period: 61.9,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 25,
            distribution_period: 61.0,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 26,
            distribution_period: 60.1,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 27,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 28,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 29,
            distribution_period: 57.4,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 30,
            distribution_period: 56.5,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 31,
            distribution_period: 55.6,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 32,
            distribution_period: 54.8,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 33,
            distribution_period: 53.9,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 34,
            distribution_period: 53.1,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 35,
            distribution_period: 52.2,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 36,
            distribution_period: 51.4,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 37,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 38,
            distribution_period: 49.8,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 39,
            distribution_period: 49.0,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 40,
            distribution_period: 48.3,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 41,
            distribution_period: 47.5,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 42,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 43,
            distribution_period: 46.1,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 44,
            distribution_period: 45.4,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 45,
            distribution_period: 44.7,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 46,
            distribution_period: 44.1,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 47,
            distribution_period: 43.5,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 48,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 49,
            distribution_period: 42.3,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 50,
            distribution_period: 41.8,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 51,
            distribution_period: 41.3,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 52,
            distribution_period: 40.8,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 53,
            distribution_period: 40.4,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 54,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 55,
            distribution_period: 39.6,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 56,
            distribution_period: 39.2,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 57,
            distribution_period: 38.9,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 58,
            distribution_period: 38.6,
        },
        JointDistributionPeriod {
            owner_age: 51,
            spouse_age: 59,
            distribution_period: 38.3,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 20,
            distribution_period: 65.6,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 21,
            distribution_period: 64.7,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 22,
            distribution_period: 63.7,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 23,
            distribution_period: 62.8,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 24,
            distribution_period: 61.9,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 25,
            distribution_period: 60.9,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 26,
            distribution_period: 60.0,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 27,
            distribution_period: 59.1,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 28,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 29,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 30,
            distribution_period: 56.4,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 31,
            distribution_period: 55.5,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 32,
            distribution_period: 54.7,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 33,
            distribution_period: 53.8,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 34,
            distribution_period: 52.9,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 35,
            distribution_period: 52.1,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 36,
            distribution_period: 51.3,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 37,
            distribution_period: 50.4,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 38,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 39,
            distribution_period: 48.8,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 40,
            distribution_period: 48.0,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 41,
            distribution_period: 47.3,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 42,
            distribution_period: 46.5,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 43,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 44,
            distribution_period: 45.1,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 45,
            distribution_period: 44.4,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 46,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 47,
            distribution_period: 43.1,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 48,
            distribution_period: 42.5,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 49,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 50,
            distribution_period: 41.4,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 51,
            distribution_period: 40.8,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 52,
            distribution_period: 40.3,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 53,
            distribution_period: 39.9,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 54,
            distribution_period: 39.4,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 55,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 56,
            distribution_period: 38.6,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 57,
            distribution_period: 38.2,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 58,
            distribution_period: 37.9,
        },
        JointDistributionPeriod {
            owner_age: 52,
            spouse_age: 59,
            distribution_period: 37.6,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 20,
            distribution_period: 65.5,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 21,
            distribution_period: 64.6,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 22,
            distribution_period: 63.7,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 23,
            distribution_period: 62.7,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 24,
            distribution_period: 61.8,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 25,
            distribution_period: 60.9,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 26,
            distribution_period: 59.9,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 27,
            distribution_period: 59.0,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 28,
            distribution_period: 58.1,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 29,
            distribution_period: 57.2,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 30,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 31,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 32,
            distribution_period: 54.6,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 33,
            distribution_period: 53.7,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 34,
            distribution_period: 52.8,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 35,
            distribution_period: 52.0,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 36,
            distribution_period: 51.1,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 37,
            distribution_period: 50.3,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 38,
            distribution_period: 49.5,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 39,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 40,
            distribution_period: 47.8,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 41,
            distribution_period: 47.1,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 42,
            distribution_period: 46.3,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 43,
            distribution_period: 45.6,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 44,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 45,
            distribution_period: 44.1,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 46,
            distribution_period: 43.4,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 47,
            distribution_period: 42.8,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 48,
            distribution_period: 42.1,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 49,
            distribution_period: 41.5,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 50,
            distribution_period: 40.9,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 51,
            distribution_period: 40.4,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 52,
            distribution_period: 39.9,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 53,
            distribution_period: 39.4,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 54,
            distribution_period: 38.9,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 55,
            distribution_period: 38.4,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 56,
            distribution_period: 38.0,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 57,
            distribution_period: 37.6,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 58,
            distribution_period: 37.3,
        },
        JointDistributionPeriod {
            owner_age: 53,
            spouse_age: 59,
            distribution_period: 36.9,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 20,
            distribution_period: 65.5,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 21,
            distribution_period: 64.6,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 22,
            distribution_period: 63.6,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 23,
            distribution_period: 62.7,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 24,
            distribution_period: 61.7,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 25,
            distribution_period: 60.8,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 26,
            distribution_period: 59.9,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 27,
            distribution_period: 59.0,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 28,
            distribution_period: 58.0,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 29,
            distribution_period: 57.1,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 30,
            distribution_period: 56.2,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 31,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 32,
            distribution_period: 54.5,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 33,
            distribution_period: 53.6,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 34,
            distribution_period: 52.7,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 35,
            distribution_period: 51.8,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 36,
            distribution_period: 51.0,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 37,
            distribution_period: 50.1,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 38,
            distribution_period: 49.3,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 39,
            distribution_period: 48.5,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 40,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 41,
            distribution_period: 46.9,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 42,
            distribution_period: 46.1,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 43,
            distribution_period: 45.3,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 44,
            distribution_period: 44.6,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 45,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 46,
            distribution_period: 43.1,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 47,
            distribution_period: 42.5,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 48,
            distribution_period: 41.8,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 49,
            distribution_period: 41.2,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 50,
            distribution_period: 40.6,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 51,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 52,
            distribution_period: 39.4,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 53,
            distribution_period: 38.9,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 54,
            distribution_period: 38.4,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 55,
            distribution_period: 37.9,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 56,
            distribution_period: 37.5,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 57,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 58,
            distribution_period: 36.7,
        },
        JointDistributionPeriod {
            owner_age: 54,
            spouse_age: 59,
            distribution_period: 36.3,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 20,
            distribution_period: 65.5,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 21,
            distribution_period: 64.5,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 22,
            distribution_period: 63.6,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 23,
            distribution_period: 62.6,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 24,
            distribution_period: 61.7,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 25,
            distribution_period: 60.8,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 26,
            distribution_period: 59.8,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 27,
            distribution_period: 58.9,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 28,
            distribution_period: 58.0,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 29,
            distribution_period: 57.1,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 30,
            distribution_period: 56.2,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 31,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 32,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 33,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 34,
            distribution_period: 52.6,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 35,
            distribution_period: 51.7,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 36,
            distribution_period: 50.9,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 37,
            distribution_period: 50.0,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 38,
            distribution_period: 49.1,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 39,
            distribution_period: 48.3,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 40,
            distribution_period: 47.5,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 41,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 42,
            distribution_period: 45.9,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 43,
            distribution_period: 45.1,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 44,
            distribution_period: 44.3,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 45,
            distribution_period: 43.6,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 46,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 47,
            distribution_period: 42.2,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 48,
            distribution_period: 41.5,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 49,
            distribution_period: 40.8,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 50,
            distribution_period: 40.2,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 51,
            distribution_period: 39.6,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 52,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 53,
            distribution_period: 38.4,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 54,
            distribution_period: 37.9,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 55,
            distribution_period: 37.4,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 56,
            distribution_period: 36.9,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 57,
            distribution_period: 36.5,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 58,
            distribution_period: 36.1,
        },
        JointDistributionPeriod {
            owner_age: 55,
            spouse_age: 59,
            distribution_period: 35.7,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 20,
            distribution_period: 65.4,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 21,
            distribution_period: 64.5,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 22,
            distribution_period: 63.5,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 23,
            distribution_period: 62.6,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 24,
            distribution_period: 61.6,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 25,
            distribution_period: 60.7,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 26,
            distribution_period: 59.8,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 27,
            distribution_period: 58.8,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 28,
            distribution_period: 57.9,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 29,
            distribution_period: 57.0,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 30,
            distribution_period: 56.1,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 31,
            distribution_period: 55.2,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 32,
            distribution_period: 54.3,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 33,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 34,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 35,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 36,
            distribution_period: 50.7,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 37,
            distribution_period: 49.9,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 38,
            distribution_period: 49.0,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 39,
            distribution_period: 48.2,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 40,
            distribution_period: 47.3,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 41,
            distribution_period: 46.5,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 42,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 43,
            distribution_period: 44.9,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 44,
            distribution_period: 44.1,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 45,
            distribution_period: 43.4,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 46,
            distribution_period: 42.6,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 47,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 48,
            distribution_period: 41.2,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 49,
            distribution_period: 40.5,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 50,
            distribution_period: 39.8,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 51,
            distribution_period: 39.2,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 52,
            distribution_period: 38.6,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 53,
            distribution_period: 38.0,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 54,
            distribution_period: 37.5,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 55,
            distribution_period: 36.9,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 56,
            distribution_period: 36.5,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 57,
            distribution_period: 36.0,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 58,
            distribution_period: 35.5,
        },
        JointDistributionPeriod {
            owner_age: 56,
            spouse_age: 59,
            distribution_period: 35.1,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 20,
            distribution_period: 65.4,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 21,
            distribution_period: 64.5,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 22,
            distribution_period: 63.5,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 23,
            distribution_period: 62.5,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 24,
            distribution_period: 61.6,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 25,
            distribution_period: 60.7,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 26,
            distribution_period: 59.7,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 27,
            distribution_period: 58.8,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 28,
            distribution_period: 57.9,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 29,
            distribution_period: 56.9,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 30,
            distribution_period: 56.0,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 31,
            distribution_period: 55.1,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 32,
            distribution_period: 54.2,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 33,
            distribution_period: 53.3,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 34,
            distribution_period: 52.4,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 35,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 36,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 37,
            distribution_period: 49.8,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 38,
            distribution_period: 48.9,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 39,
            distribution_period: 48.0,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 40,
            distribution_period: 47.2,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 41,
            distribution_period: 46.3,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 42,
            distribution_period: 45.5,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 43,
            distribution_period: 44.7,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 44,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 45,
            distribution_period: 43.1,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 46,
            distribution_period: 42.4,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 47,
            distribution_period: 41.6,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 48,
            distribution_period: 40.9,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 49,
            distribution_period: 40.2,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 50,
            distribution_period: 39.5,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 51,
            distribution_period: 38.9,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 52,
            distribution_period: 38.2,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 53,
            distribution_period: 37.6,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 54,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 55,
            distribution_period: 36.5,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 56,
            distribution_period: 36.0,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 57,
            distribution_period: 35.5,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 58,
            distribution_period: 35.0,
        },
        JointDistributionPeriod {
            owner_age: 57,
            spouse_age: 59,
            distribution_period: 34.6,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 20,
            distribution_period: 65.4,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 21,
            distribution_period: 64.4,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 22,
            distribution_period: 63.5,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 23,
            distribution_period: 62.5,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 24,
            distribution_period: 61.6,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 25,
            distribution_period: 60.6,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 26,
            distribution_period: 59.7,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 27,
            distribution_period: 58.7,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 28,
            distribution_period: 57.8,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 29,
            distribution_period: 56.9,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 30,
            distribution_period: 56.0,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 31,
            distribution_period: 55.0,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 32,
            distribution_period: 54.1,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 33,
            distribution_period: 53.2,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 34,
            distribution_period: 52.3,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 35,
            distribution_period: 51.4,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 36,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 37,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 38,
            distribution_period: 48.8,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 39,
            distribution_period: 47.9,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 40,
            distribution_period: 47.1,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 41,
            distribution_period: 46.2,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 42,
            distribution_period: 45.4,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 43,
            distribution_period: 44.5,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 44,
            distribution_period: 43.7,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 45,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 46,
            distribution_period: 42.2,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 47,
            distribution_period: 41.4,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 48,
            distribution_period: 40.7,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 49,
            distribution_period: 39.9,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 50,
            distribution_period: 39.2,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 51,
            distribution_period: 38.6,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 52,
            distribution_period: 37.9,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 53,
            distribution_period: 37.3,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 54,
            distribution_period: 36.7,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 55,
            distribution_period: 36.1,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 56,
            distribution_period: 35.5,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 57,
            distribution_period: 35.0,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 58,
            distribution_period: 34.5,
        },
        JointDistributionPeriod {
            owner_age: 58,
            spouse_age: 59,
            distribution_period: 34.1,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 20,
            distribution_period: 65.4,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 21,
            distribution_period: 64.4,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 22,
            distribution_period: 63.4,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 23,
            distribution_period: 62.5,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 24,
            distribution_period: 61.5,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 25,
            distribution_period: 60.6,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 26,
            distribution_period: 59.6,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 27,
            distribution_period: 58.7,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 28,
            distribution_period: 57.8,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 29,
            distribution_period: 56.8,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 30,
            distribution_period: 55.9,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 31,
            distribution_period: 55.0,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 32,
            distribution_period: 54.1,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 33,
            distribution_period: 53.2,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 34,
            distribution_period: 52.2,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 35,
            distribution_period: 51.3,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 36,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 37,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 38,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 39,
            distribution_period: 47.8,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 40,
            distribution_period: 46.9,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 41,
            distribution_period: 46.1,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 42,
            distribution_period: 45.2,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 43,
            distribution_period: 44.4,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 44,
            distribution_period: 43.6,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 45,
            distribution_period: 42.8,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 46,
            distribution_period: 42.0,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 47,
            distribution_period: 41.2,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 48,
            distribution_period: 40.4,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 49,
            distribution_period: 39.7,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 50,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 51,
            distribution_period: 38.3,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 52,
            distribution_period: 37.6,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 53,
            distribution_period: 36.9,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 54,
            distribution_period: 36.3,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 55,
            distribution_period: 35.7,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 56,
            distribution_period: 35.1,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 57,
            distribution_period: 34.6,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 58,
            distribution_period: 34.1,
        },
        JointDistributionPeriod {
            owner_age: 59,
            spouse_age: 59,
            distribution_period: 33.6,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 20,
            distribution_period: 65.3,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 21,
            distribution_period: 64.4,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 22,
            distribution_period: 63.4,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 23,
            distribution_period: 62.4,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 24,
            distribution_period: 61.5,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 25,
            distribution_period: 60.5,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 26,
            distribution_period: 59.6,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 27,
            distribution_period: 58.7,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 28,
            distribution_period: 57.7,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 29,
            distribution_period: 56.8,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 30,
            distribution_period: 55.9,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 31,
            distribution_period: 54.9,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 32,
            distribution_period: 54.0,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 33,
            distribution_period: 53.1,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 34,
            distribution_period: 52.2,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 35,
            distribution_period: 51.3,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 36,
            distribution_period: 50.4,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 37,
            distribution_period: 49.5,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 38,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 39,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 40,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 41,
            distribution_period: 46.0,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 42,
            distribution_period: 45.1,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 43,
            distribution_period: 44.3,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 44,
            distribution_period: 43.4,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 45,
            distribution_period: 42.6,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 46,
            distribution_period: 41.8,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 47,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 48,
            distribution_period: 40.2,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 49,
            distribution_period: 39.5,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 50,
            distribution_period: 38.7,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 51,
            distribution_period: 38.0,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 52,
            distribution_period: 37.3,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 53,
            distribution_period: 36.6,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 54,
            distribution_period: 36.0,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 55,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 56,
            distribution_period: 34.8,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 57,
            distribution_period: 34.2,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 58,
            distribution_period: 33.6,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 59,
            distribution_period: 33.1,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 60,
            distribution_period: 32.6,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 61,
            distribution_period: 32.2,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 62,
            distribution_period: 31.7,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 63,
            distribution_period: 31.3,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 64,
            distribution_period: 31.0,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 65,
            distribution_period: 30.6,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 66,
            distribution_period: 30.3,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 67,
            distribution_period: 30.0,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 68,
            distribution_period: 29.7,
        },
        JointDistributionPeriod {
            owner_age: 60,
            spouse_age: 69,
            distribution_period: 29.4,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 20,
            distribution_period: 65.3,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 21,
            distribution_period: 64.3,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 22,
            distribution_period: 63.4,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 23,
            distribution_period: 62.4,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 24,
            distribution_period: 61.5,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 25,
            distribution_period: 60.5,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 26,
            distribution_period: 59.6,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 27,
            distribution_period: 58.6,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 28,
            distribution_period: 57.7,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 29,
            distribution_period: 56.7,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 30,
            distribution_period: 55.8,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 31,
            distribution_period: 54.9,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 32,
            distribution_period: 54.0,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 33,
            distribution_period: 53.0,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 34,
            distribution_period: 52.1,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 35,
            distribution_period: 51.2,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 36,
            distribution_period: 50.3,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 37,
            distribution_period: 49.4,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 38,
            distribution_period: 48.5,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 39,
            distribution_period: 47.6,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 40,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 41,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 42,
            distribution_period: 45.0,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 43,
            distribution_period: 44.1,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 44,
            distribution_period: 43.3,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 45,
            distribution_period: 42.4,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 46,
            distribution_period: 41.6,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 47,
            distribution_period: 40.8,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 48,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 49,
            distribution_period: 39.2,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 50,
            distribution_period: 38.5,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 51,
            distribution_period: 37.7,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 52,
            distribution_period: 37.0,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 53,
            distribution_period: 36.3,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 54,
            distribution_period: 35.7,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 55,
            distribution_period: 35.0,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 56,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 57,
            distribution_period: 33.8,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 58,
            distribution_period: 33.2,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 59,
            distribution_period: 32.7,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 60,
            distribution_period: 32.2,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 61,
            distribution_period: 31.7,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 62,
            distribution_period: 31.2,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 63,
            distribution_period: 30.8,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 64,
            distribution_period: 30.4,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 65,
            distribution_period: 30.0,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 66,
            distribution_period: 29.7,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 67,
            distribution_period: 29.4,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 68,
            distribution_period: 29.1,
        },
        JointDistributionPeriod {
            owner_age: 61,
            spouse_age: 69,
            distribution_period: 28.8,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 20,
            distribution_period: 65.3,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 21,
            distribution_period: 64.3,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 22,
            distribution_period: 63.4,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 23,
            distribution_period: 62.4,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 24,
            distribution_period: 61.4,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 25,
            distribution_period: 60.5,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 26,
            distribution_period: 59.5,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 27,
            distribution_period: 58.6,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 28,
            distribution_period: 57.6,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 29,
            distribution_period: 56.7,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 30,
            distribution_period: 55.8,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 31,
            distribution_period: 54.8,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 32,
            distribution_period: 53.9,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 33,
            distribution_period: 53.0,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 34,
            distribution_period: 52.1,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 35,
            distribution_period: 51.1,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 36,
            distribution_period: 50.2,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 37,
            distribution_period: 49.3,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 38,
            distribution_period: 48.4,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 39,
            distribution_period: 47.5,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 40,
            distribution_period: 46.6,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 41,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 42,
            distribution_period: 44.9,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 43,
            distribution_period: 44.0,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 44,
            distribution_period: 43.1,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 45,
            distribution_period: 42.3,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 46,
            distribution_period: 41.5,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 47,
            distribution_period: 40.6,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 48,
            distribution_period: 39.8,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 49,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 50,
            distribution_period: 38.3,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 51,
            distribution_period: 37.5,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 52,
            distribution_period: 36.8,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 53,
            distribution_period: 36.1,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 54,
            distribution_period: 35.4,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 55,
            distribution_period: 34.7,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 56,
            distribution_period: 34.1,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 57,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 58,
            distribution_period: 32.8,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 59,
            distribution_period: 32.3,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 60,
            distribution_period: 31.7,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 61,
            distribution_period: 31.2,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 62,
            distribution_period: 30.8,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 63,
            distribution_period: 30.3,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 64,
            distribution_period: 29.9,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 65,
            distribution_period: 29.5,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 66,
            distribution_period: 29.1,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 67,
            distribution_period: 28.7,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 68,
            distribution_period: 28.4,
        },
        JointDistributionPeriod {
            owner_age: 62,
            spouse_age: 69,
            distribution_period: 28.1,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 20,
            distribution_period: 65.3,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 21,
            distribution_period: 64.3,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 22,
            distribution_period: 63.3,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 23,
            distribution_period: 62.4,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 24,
            distribution_period: 61.4,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 25,
            distribution_period: 60.5,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 26,
            distribution_period: 59.5,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 27,
            distribution_period: 58.6,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 28,
            distribution_period: 57.6,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 29,
            distribution_period: 56.7,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 30,
            distribution_period: 55.7,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 31,
            distribution_period: 54.8,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 32,
            distribution_period: 53.9,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 33,
            distribution_period: 52.9,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 34,
            distribution_period: 52.0,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 35,
            distribution_period: 51.1,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 36,
            distribution_period: 50.2,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 37,
            distribution_period: 49.3,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 38,
            distribution_period: 48.3,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 39,
            distribution_period: 47.4,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 40,
            distribution_period: 46.5,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 41,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 42,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 43,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 44,
            distribution_period: 43.0,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 45,
            distribution_period: 42.2,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 46,
            distribution_period: 41.3,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 47,
            distribution_period: 40.5,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 48,
            distribution_period: 39.7,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 49,
            distribution_period: 38.9,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 50,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 51,
            distribution_period: 37.3,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 52,
            distribution_period: 36.6,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 53,
            distribution_period: 35.8,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 54,
            distribution_period: 35.1,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 55,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 56,
            distribution_period: 33.8,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 57,
            distribution_period: 33.1,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 58,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 59,
            distribution_period: 31.9,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 60,
            distribution_period: 31.3,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 61,
            distribution_period: 30.8,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 62,
            distribution_period: 30.3,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 63,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 64,
            distribution_period: 29.4,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 65,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 66,
            distribution_period: 28.5,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 67,
            distribution_period: 28.2,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 68,
            distribution_period: 27.8,
        },
        JointDistributionPeriod {
            owner_age: 63,
            spouse_age: 69,
            distribution_period: 27.5,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 20,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 21,
            distribution_period: 64.3,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 22,
            distribution_period: 63.3,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 23,
            distribution_period: 62.3,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 24,
            distribution_period: 61.4,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 25,
            distribution_period: 60.4,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 26,
            distribution_period: 59.5,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 27,
            distribution_period: 58.5,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 28,
            distribution_period: 57.6,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 29,
            distribution_period: 56.6,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 30,
            distribution_period: 55.7,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 31,
            distribution_period: 54.8,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 32,
            distribution_period: 53.8,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 33,
            distribution_period: 52.9,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 34,
            distribution_period: 52.0,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 35,
            distribution_period: 51.0,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 36,
            distribution_period: 50.1,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 37,
            distribution_period: 49.2,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 38,
            distribution_period: 48.3,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 39,
            distribution_period: 47.4,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 40,
            distribution_period: 46.5,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 41,
            distribution_period: 45.6,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 42,
            distribution_period: 44.7,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 43,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 44,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 45,
            distribution_period: 42.1,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 46,
            distribution_period: 41.2,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 47,
            distribution_period: 40.4,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 48,
            distribution_period: 39.5,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 49,
            distribution_period: 38.7,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 50,
            distribution_period: 37.9,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 51,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 52,
            distribution_period: 36.3,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 53,
            distribution_period: 35.6,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 54,
            distribution_period: 34.9,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 55,
            distribution_period: 34.2,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 56,
            distribution_period: 33.5,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 57,
            distribution_period: 32.8,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 58,
            distribution_period: 32.2,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 59,
            distribution_period: 31.5,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 60,
            distribution_period: 31.0,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 61,
            distribution_period: 30.4,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 62,
            distribution_period: 29.9,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 63,
            distribution_period: 29.4,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 64,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 65,
            distribution_period: 28.4,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 66,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 67,
            distribution_period: 27.6,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 68,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 64,
            spouse_age: 69,
            distribution_period: 26.9,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 20,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 21,
            distribution_period: 64.3,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 22,
            distribution_period: 63.3,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 23,
            distribution_period: 62.3,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 24,
            distribution_period: 61.4,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 25,
            distribution_period: 60.4,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 26,
            distribution_period: 59.5,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 27,
            distribution_period: 58.5,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 28,
            distribution_period: 57.5,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 29,
            distribution_period: 56.6,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 30,
            distribution_period: 55.7,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 31,
            distribution_period: 54.7,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 32,
            distribution_period: 53.8,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 33,
            distribution_period: 52.8,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 34,
            distribution_period: 51.9,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 35,
            distribution_period: 51.0,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 36,
            distribution_period: 50.1,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 37,
            distribution_period: 49.1,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 38,
            distribution_period: 48.2,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 39,
            distribution_period: 47.3,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 40,
            distribution_period: 46.4,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 41,
            distribution_period: 45.5,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 42,
            distribution_period: 44.6,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 43,
            distribution_period: 43.7,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 44,
            distribution_period: 42.8,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 45,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 46,
            distribution_period: 41.1,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 47,
            distribution_period: 40.2,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 48,
            distribution_period: 39.4,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 49,
            distribution_period: 38.6,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 50,
            distribution_period: 37.7,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 51,
            distribution_period: 36.9,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 52,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 53,
            distribution_period: 35.4,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 54,
            distribution_period: 34.6,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 55,
            distribution_period: 33.9,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 56,
            distribution_period: 33.2,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 57,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 58,
            distribution_period: 31.9,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 59,
            distribution_period: 31.2,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 60,
            distribution_period: 30.6,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 61,
            distribution_period: 30.0,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 62,
            distribution_period: 29.5,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 63,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 64,
            distribution_period: 28.4,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 65,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 66,
            distribution_period: 27.5,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 67,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 68,
            distribution_period: 26.7,
        },
        JointDistributionPeriod {
            owner_age: 65,
            spouse_age: 69,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 20,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 21,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 22,
            distribution_period: 63.3,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 23,
            distribution_period: 62.3,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 24,
            distribution_period: 61.3,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 25,
            distribution_period: 60.4,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 26,
            distribution_period: 59.4,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 27,
            distribution_period: 58.5,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 28,
            distribution_period: 57.5,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 29,
            distribution_period: 56.6,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 30,
            distribution_period: 55.6,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 31,
            distribution_period: 54.7,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 32,
            distribution_period: 53.7,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 33,
            distribution_period: 52.8,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 34,
            distribution_period: 51.9,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 35,
            distribution_period: 50.9,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 36,
            distribution_period: 50.0,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 37,
            distribution_period: 49.1,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 38,
            distribution_period: 48.2,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 39,
            distribution_period: 47.2,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 40,
            distribution_period: 46.3,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 41,
            distribution_period: 45.4,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 42,
            distribution_period: 44.5,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 43,
            distribution_period: 43.6,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 44,
            distribution_period: 42.7,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 45,
            distribution_period: 41.8,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 46,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 47,
            distribution_period: 40.1,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 48,
            distribution_period: 39.3,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 49,
            distribution_period: 38.4,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 50,
            distribution_period: 37.6,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 51,
            distribution_period: 36.8,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 52,
            distribution_period: 36.0,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 53,
            distribution_period: 35.2,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 54,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 55,
            distribution_period: 33.7,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 56,
            distribution_period: 33.0,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 57,
            distribution_period: 32.3,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 58,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 59,
            distribution_period: 30.9,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 60,
            distribution_period: 30.3,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 61,
            distribution_period: 29.7,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 62,
            distribution_period: 29.1,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 63,
            distribution_period: 28.5,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 64,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 65,
            distribution_period: 27.5,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 66,
            distribution_period: 27.0,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 67,
            distribution_period: 26.6,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 68,
            distribution_period: 26.2,
        },
        JointDistributionPeriod {
            owner_age: 66,
            spouse_age: 69,
            distribution_period: 25.8,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 20,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 21,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 22,
            distribution_period: 63.3,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 23,
            distribution_period: 62.3,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 24,
            distribution_period: 61.3,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 25,
            distribution_period: 60.4,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 26,
            distribution_period: 59.4,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 27,
            distribution_period: 58.5,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 28,
            distribution_period: 57.5,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 29,
            distribution_period: 56.5,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 30,
            distribution_period: 55.6,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 31,
            distribution_period: 54.7,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 32,
            distribution_period: 53.7,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 33,
            distribution_period: 52.8,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 34,
            distribution_period: 51.8,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 35,
            distribution_period: 50.9,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 36,
            distribution_period: 50.0,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 37,
            distribution_period: 49.0,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 38,
            distribution_period: 48.1,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 39,
            distribution_period: 47.2,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 40,
            distribution_period: 46.3,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 41,
            distribution_period: 45.4,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 42,
            distribution_period: 44.4,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 43,
            distribution_period: 43.5,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 44,
            distribution_period: 42.6,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 45,
            distribution_period: 41.8,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 46,
            distribution_period: 40.9,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 47,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 48,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 49,
            distribution_period: 38.3,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 50,
            distribution_period: 37.5,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 51,
            distribution_period: 36.6,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 52,
            distribution_period: 35.8,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 53,
            distribution_period: 35.0,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 54,
            distribution_period: 34.2,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 55,
            distribution_period: 33.5,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 56,
            distribution_period: 32.7,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 57,
            distribution_period: 32.0,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 58,
            distribution_period: 31.3,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 59,
            distribution_period: 30.6,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 60,
            distribution_period: 30.0,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 61,
            distribution_period: 29.4,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 62,
            distribution_period: 28.7,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 63,
            distribution_period: 28.2,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 64,
            distribution_period: 27.6,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 65,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 66,
            distribution_period: 26.6,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 67,
            distribution_period: 26.1,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 68,
            distribution_period: 25.7,
        },
        JointDistributionPeriod {
            owner_age: 67,
            spouse_age: 69,
            distribution_period: 25.3,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 20,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 21,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 22,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 23,
            distribution_period: 62.3,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 24,
            distribution_period: 61.3,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 25,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 26,
            distribution_period: 59.4,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 27,
            distribution_period: 58.4,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 28,
            distribution_period: 57.5,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 29,
            distribution_period: 56.5,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 30,
            distribution_period: 55.6,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 31,
            distribution_period: 54.6,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 32,
            distribution_period: 53.7,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 33,
            distribution_period: 52.7,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 34,
            distribution_period: 51.8,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 35,
            distribution_period: 50.9,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 36,
            distribution_period: 49.9,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 37,
            distribution_period: 49.0,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 38,
            distribution_period: 48.1,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 39,
            distribution_period: 47.1,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 40,
            distribution_period: 46.2,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 41,
            distribution_period: 45.3,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 42,
            distribution_period: 44.4,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 43,
            distribution_period: 43.5,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 44,
            distribution_period: 42.6,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 45,
            distribution_period: 41.7,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 46,
            distribution_period: 40.8,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 47,
            distribution_period: 39.9,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 48,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 49,
            distribution_period: 38.2,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 50,
            distribution_period: 37.3,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 51,
            distribution_period: 36.5,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 52,
            distribution_period: 35.7,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 53,
            distribution_period: 34.9,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 54,
            distribution_period: 34.1,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 55,
            distribution_period: 33.3,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 56,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 57,
            distribution_period: 31.8,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 58,
            distribution_period: 31.1,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 59,
            distribution_period: 30.4,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 60,
            distribution_period: 29.7,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 61,
            distribution_period: 29.1,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 62,
            distribution_period: 28.4,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 63,
            distribution_period: 27.8,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 64,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 65,
            distribution_period: 26.7,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 66,
            distribution_period: 26.2,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 67,
            distribution_period: 25.7,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 68,
            distribution_period: 25.2,
        },
        JointDistributionPeriod {
            owner_age: 68,
            spouse_age: 69,
            distribution_period: 24.8,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 20,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 21,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 22,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 23,
            distribution_period: 62.3,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 24,
            distribution_period: 61.3,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 25,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 26,
            distribution_period: 59.4,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 27,
            distribution_period: 58.4,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 28,
            distribution_period: 57.5,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 29,
            distribution_period: 56.5,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 30,
            distribution_period: 55.6,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 31,
            distribution_period: 54.6,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 32,
            distribution_period: 53.7,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 33,
            distribution_period: 52.7,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 34,
            distribution_period: 51.8,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 35,
            distribution_period: 50.8,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 36,
            distribution_period: 49.9,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 37,
            distribution_period: 49.0,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 38,
            distribution_period: 48.0,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 39,
            distribution_period: 47.1,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 40,
            distribution_period: 46.2,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 41,
            distribution_period: 45.2,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 42,
            distribution_period: 44.3,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 43,
            distribution_period: 43.4,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 44,
            distribution_period: 42.5,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 45,
            distribution_period: 41.6,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 46,
            distribution_period: 40.7,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 47,
            distribution_period: 39.8,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 48,
            distribution_period: 38.9,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 49,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 50,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 51,
            distribution_period: 36.4,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 52,
            distribution_period: 35.5,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 53,
            distribution_period: 34.7,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 54,
            distribution_period: 33.9,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 55,
            distribution_period: 33.1,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 56,
            distribution_period: 32.3,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 57,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 58,
            distribution_period: 30.9,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 59,
            distribution_period: 30.1,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 60,
            distribution_period: 29.4,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 61,
            distribution_period: 28.8,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 62,
            distribution_period: 28.1,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 63,
            distribution_period: 27.5,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 64,
            distribution_period: 26.9,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 65,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 66,
            distribution_period: 25.8,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 67,
            distribution_period: 25.3,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 68,
            distribution_period: 24.8,
        },
        JointDistributionPeriod {
            owner_age: 69,
            spouse_age: 69,
            distribution_period: 24.3,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 20,
            distribution_period: 65.2,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 21,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 22,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 23,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 24,
            distribution_period: 61.3,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 25,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 26,
            distribution_period: 59.4,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 27,
            distribution_period: 58.4,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 28,
            distribution_period: 57.4,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 29,
            distribution_period: 56.5,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 30,
            distribution_period: 55.5,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 31,
            distribution_period: 54.6,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 32,
            distribution_period: 53.6,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 33,
            distribution_period: 52.7,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 34,
            distribution_period: 51.7,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 35,
            distribution_period: 50.8,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 36,
            distribution_period: 49.9,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 37,
            distribution_period: 48.9,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 38,
            distribution_period: 48.0,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 39,
            distribution_period: 47.0,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 40,
            distribution_period: 46.1,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 41,
            distribution_period: 45.2,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 42,
            distribution_period: 44.3,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 43,
            distribution_period: 43.3,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 44,
            distribution_period: 42.4,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 45,
            distribution_period: 41.5,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 46,
            distribution_period: 40.6,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 47,
            distribution_period: 39.7,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 48,
            distribution_period: 38.8,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 49,
            distribution_period: 38.0,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 50,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 51,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 52,
            distribution_period: 35.4,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 53,
            distribution_period: 34.6,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 54,
            distribution_period: 33.8,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 55,
            distribution_period: 33.0,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 56,
            distribution_period: 32.2,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 57,
            distribution_period: 31.4,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 58,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 59,
            distribution_period: 29.9,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 60,
            distribution_period: 29.2,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 61,
            distribution_period: 28.5,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 62,
            distribution_period: 27.9,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 63,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 64,
            distribution_period: 26.6,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 65,
            distribution_period: 26.0,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 66,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 67,
            distribution_period: 24.9,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 68,
            distribution_period: 24.3,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 69,
            distribution_period: 23.9,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 70,
            distribution_period: 23.4,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 71,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 72,
            distribution_period: 22.5,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 73,
            distribution_period: 22.2,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 74,
            distribution_period: 21.8,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 75,
            distribution_period: 21.5,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 76,
            distribution_period: 21.2,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 77,
            distribution_period: 20.9,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 78,
            distribution_period: 20.6,
        },
        JointDistributionPeriod {
            owner_age: 70,
            spouse_age: 79,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 21,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 22,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 23,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 24,
            distribution_period: 61.3,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 25,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 26,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 27,
            distribution_period: 58.4,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 28,
            distribution_period: 57.4,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 29,
            distribution_period: 56.5,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 30,
            distribution_period: 55.5,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 31,
            distribution_period: 54.6,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 32,
            distribution_period: 53.6,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 33,
            distribution_period: 52.7,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 34,
            distribution_period: 51.7,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 35,
            distribution_period: 50.8,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 36,
            distribution_period: 49.8,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 37,
            distribution_period: 48.9,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 38,
            distribution_period: 47.9,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 39,
            distribution_period: 47.0,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 40,
            distribution_period: 46.1,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 41,
            distribution_period: 45.1,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 42,
            distribution_period: 44.2,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 43,
            distribution_period: 43.3,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 44,
            distribution_period: 42.4,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 45,
            distribution_period: 41.5,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 46,
            distribution_period: 40.6,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 47,
            distribution_period: 39.7,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 48,
            distribution_period: 38.8,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 49,
            distribution_period: 37.9,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 50,
            distribution_period: 37.0,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 51,
            distribution_period: 36.1,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 52,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 53,
            distribution_period: 34.5,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 54,
            distribution_period: 33.6,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 55,
            distribution_period: 32.8,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 56,
            distribution_period: 32.0,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 57,
            distribution_period: 31.2,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 58,
            distribution_period: 30.5,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 59,
            distribution_period: 29.7,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 60,
            distribution_period: 29.0,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 61,
            distribution_period: 28.3,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 62,
            distribution_period: 27.6,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 63,
            distribution_period: 26.9,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 64,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 65,
            distribution_period: 25.7,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 66,
            distribution_period: 25.1,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 67,
            distribution_period: 24.5,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 68,
            distribution_period: 24.0,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 69,
            distribution_period: 23.4,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 70,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 71,
            distribution_period: 22.5,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 72,
            distribution_period: 22.0,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 73,
            distribution_period: 21.6,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 74,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 75,
            distribution_period: 20.9,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 76,
            distribution_period: 20.6,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 77,
            distribution_period: 20.3,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 78,
            distribution_period: 20.0,
        },
        JointDistributionPeriod {
            owner_age: 71,
            spouse_age: 79,
            distribution_period: 19.8,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 21,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 22,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 23,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 24,
            distribution_period: 61.3,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 25,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 26,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 27,
            distribution_period: 58.4,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 28,
            distribution_period: 57.4,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 29,
            distribution_period: 56.5,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 30,
            distribution_period: 55.5,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 31,
            distribution_period: 54.5,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 32,
            distribution_period: 53.6,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 33,
            distribution_period: 52.6,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 34,
            distribution_period: 51.7,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 35,
            distribution_period: 50.8,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 36,
            distribution_period: 49.8,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 37,
            distribution_period: 48.9,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 38,
            distribution_period: 47.9,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 39,
            distribution_period: 47.0,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 40,
            distribution_period: 46.0,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 41,
            distribution_period: 45.1,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 42,
            distribution_period: 44.2,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 43,
            distribution_period: 43.2,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 44,
            distribution_period: 42.3,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 45,
            distribution_period: 41.4,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 46,
            distribution_period: 40.5,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 47,
            distribution_period: 39.6,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 48,
            distribution_period: 38.7,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 49,
            distribution_period: 37.8,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 50,
            distribution_period: 36.9,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 51,
            distribution_period: 36.0,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 52,
            distribution_period: 35.2,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 53,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 54,
            distribution_period: 33.5,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 55,
            distribution_period: 32.7,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 56,
            distribution_period: 31.9,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 57,
            distribution_period: 31.1,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 58,
            distribution_period: 30.3,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 59,
            distribution_period: 29.5,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 60,
            distribution_period: 28.8,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 61,
            distribution_period: 28.1,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 62,
            distribution_period: 27.4,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 63,
            distribution_period: 26.7,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 64,
            distribution_period: 26.0,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 65,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 66,
            distribution_period: 24.8,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 67,
            distribution_period: 24.2,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 68,
            distribution_period: 23.6,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 69,
            distribution_period: 23.1,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 70,
            distribution_period: 22.5,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 71,
            distribution_period: 22.0,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 72,
            distribution_period: 21.6,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 73,
            distribution_period: 21.1,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 74,
            distribution_period: 20.7,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 75,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 76,
            distribution_period: 20.0,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 77,
            distribution_period: 19.7,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 78,
            distribution_period: 19.4,
        },
        JointDistributionPeriod {
            owner_age: 72,
            spouse_age: 79,
            distribution_period: 19.2,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 21,
            distribution_period: 64.2,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 22,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 23,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 25,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 26,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 27,
            distribution_period: 58.4,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 28,
            distribution_period: 57.4,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 29,
            distribution_period: 56.4,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 30,
            distribution_period: 55.5,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 31,
            distribution_period: 54.5,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 32,
            distribution_period: 53.6,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 33,
            distribution_period: 52.6,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 34,
            distribution_period: 51.7,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 35,
            distribution_period: 50.7,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 36,
            distribution_period: 49.8,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 37,
            distribution_period: 48.8,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 38,
            distribution_period: 47.9,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 39,
            distribution_period: 46.9,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 40,
            distribution_period: 46.0,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 41,
            distribution_period: 45.1,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 42,
            distribution_period: 44.1,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 43,
            distribution_period: 43.2,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 44,
            distribution_period: 42.3,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 45,
            distribution_period: 41.4,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 46,
            distribution_period: 40.4,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 47,
            distribution_period: 39.5,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 48,
            distribution_period: 38.6,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 49,
            distribution_period: 37.7,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 50,
            distribution_period: 36.8,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 51,
            distribution_period: 36.0,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 52,
            distribution_period: 35.1,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 53,
            distribution_period: 34.2,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 54,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 55,
            distribution_period: 32.6,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 56,
            distribution_period: 31.7,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 57,
            distribution_period: 30.9,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 58,
            distribution_period: 30.1,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 59,
            distribution_period: 29.4,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 60,
            distribution_period: 28.6,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 61,
            distribution_period: 27.9,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 62,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 63,
            distribution_period: 26.5,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 64,
            distribution_period: 25.8,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 65,
            distribution_period: 25.1,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 66,
            distribution_period: 24.5,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 67,
            distribution_period: 23.9,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 68,
            distribution_period: 23.3,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 69,
            distribution_period: 22.7,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 70,
            distribution_period: 22.2,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 71,
            distribution_period: 21.6,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 72,
            distribution_period: 21.1,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 73,
            distribution_period: 20.7,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 74,
            distribution_period: 20.3,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 75,
            distribution_period: 19.9,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 76,
            distribution_period: 19.5,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 77,
            distribution_period: 19.1,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 78,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 73,
            spouse_age: 79,
            distribution_period: 18.6,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 22,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 23,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 25,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 26,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 28,
            distribution_period: 57.4,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 29,
            distribution_period: 56.4,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 30,
            distribution_period: 55.5,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 31,
            distribution_period: 54.5,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 32,
            distribution_period: 53.6,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 33,
            distribution_period: 52.6,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 34,
            distribution_period: 51.7,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 35,
            distribution_period: 50.7,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 36,
            distribution_period: 49.8,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 37,
            distribution_period: 48.8,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 38,
            distribution_period: 47.9,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 39,
            distribution_period: 46.9,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 40,
            distribution_period: 46.0,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 41,
            distribution_period: 45.0,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 42,
            distribution_period: 44.1,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 43,
            distribution_period: 43.2,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 44,
            distribution_period: 42.2,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 45,
            distribution_period: 41.3,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 46,
            distribution_period: 40.4,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 47,
            distribution_period: 39.5,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 48,
            distribution_period: 38.6,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 49,
            distribution_period: 37.7,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 50,
            distribution_period: 36.8,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 51,
            distribution_period: 35.9,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 52,
            distribution_period: 35.0,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 53,
            distribution_period: 34.1,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 54,
            distribution_period: 33.3,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 55,
            distribution_period: 32.4,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 56,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 57,
            distribution_period: 30.8,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 58,
            distribution_period: 30.0,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 59,
            distribution_period: 29.2,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 60,
            distribution_period: 28.4,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 61,
            distribution_period: 27.7,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 62,
            distribution_period: 27.0,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 63,
            distribution_period: 26.2,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 64,
            distribution_period: 25.5,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 65,
            distribution_period: 24.9,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 66,
            distribution_period: 24.2,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 67,
            distribution_period: 23.6,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 68,
            distribution_period: 23.0,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 69,
            distribution_period: 22.4,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 70,
            distribution_period: 21.8,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 71,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 72,
            distribution_period: 20.7,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 73,
            distribution_period: 20.3,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 74,
            distribution_period: 19.8,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 75,
            distribution_period: 19.4,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 76,
            distribution_period: 19.0,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 77,
            distribution_period: 18.6,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 78,
            distribution_period: 18.3,
        },
        JointDistributionPeriod {
            owner_age: 74,
            spouse_age: 79,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 22,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 23,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 25,
            distribution_period: 60.3,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 26,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 28,
            distribution_period: 57.4,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 29,
            distribution_period: 56.4,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 30,
            distribution_period: 55.5,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 31,
            distribution_period: 54.5,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 33,
            distribution_period: 52.6,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 34,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 35,
            distribution_period: 50.7,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 36,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 37,
            distribution_period: 48.8,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 38,
            distribution_period: 47.8,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 39,
            distribution_period: 46.9,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 40,
            distribution_period: 45.9,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 41,
            distribution_period: 45.0,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 42,
            distribution_period: 44.1,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 43,
            distribution_period: 43.1,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 44,
            distribution_period: 42.2,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 45,
            distribution_period: 41.3,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 46,
            distribution_period: 40.3,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 47,
            distribution_period: 39.4,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 48,
            distribution_period: 38.5,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 49,
            distribution_period: 37.6,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 50,
            distribution_period: 36.7,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 51,
            distribution_period: 35.8,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 52,
            distribution_period: 34.9,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 53,
            distribution_period: 34.1,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 54,
            distribution_period: 33.2,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 55,
            distribution_period: 32.4,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 56,
            distribution_period: 31.5,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 57,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 58,
            distribution_period: 29.9,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 59,
            distribution_period: 29.1,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 60,
            distribution_period: 28.3,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 61,
            distribution_period: 27.5,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 62,
            distribution_period: 26.8,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 63,
            distribution_period: 26.1,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 64,
            distribution_period: 25.3,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 65,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 66,
            distribution_period: 24.0,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 67,
            distribution_period: 23.3,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 68,
            distribution_period: 22.7,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 69,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 70,
            distribution_period: 21.5,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 71,
            distribution_period: 20.9,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 72,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 73,
            distribution_period: 19.9,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 74,
            distribution_period: 19.4,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 75,
            distribution_period: 18.9,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 76,
            distribution_period: 18.5,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 77,
            distribution_period: 18.1,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 78,
            distribution_period: 17.8,
        },
        JointDistributionPeriod {
            owner_age: 75,
            spouse_age: 79,
            distribution_period: 17.4,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 22,
            distribution_period: 63.2,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 23,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 26,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 28,
            distribution_period: 57.4,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 29,
            distribution_period: 56.4,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 31,
            distribution_period: 54.5,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 33,
            distribution_period: 52.6,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 34,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 35,
            distribution_period: 50.7,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 36,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 37,
            distribution_period: 48.8,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 38,
            distribution_period: 47.8,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 39,
            distribution_period: 46.9,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 40,
            distribution_period: 45.9,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 41,
            distribution_period: 45.0,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 42,
            distribution_period: 44.0,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 43,
            distribution_period: 43.1,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 44,
            distribution_period: 42.2,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 45,
            distribution_period: 41.2,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 46,
            distribution_period: 40.3,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 47,
            distribution_period: 39.4,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 48,
            distribution_period: 38.5,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 49,
            distribution_period: 37.5,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 50,
            distribution_period: 36.6,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 51,
            distribution_period: 35.7,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 52,
            distribution_period: 34.9,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 53,
            distribution_period: 34.0,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 54,
            distribution_period: 33.1,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 55,
            distribution_period: 32.3,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 56,
            distribution_period: 31.4,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 57,
            distribution_period: 30.6,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 58,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 59,
            distribution_period: 29.0,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 60,
            distribution_period: 28.2,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 61,
            distribution_period: 27.4,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 62,
            distribution_period: 26.6,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 63,
            distribution_period: 25.9,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 64,
            distribution_period: 25.2,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 65,
            distribution_period: 24.4,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 66,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 67,
            distribution_period: 23.1,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 68,
            distribution_period: 22.4,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 69,
            distribution_period: 21.8,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 70,
            distribution_period: 21.2,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 71,
            distribution_period: 20.6,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 72,
            distribution_period: 20.0,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 73,
            distribution_period: 19.5,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 74,
            distribution_period: 19.0,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 75,
            distribution_period: 18.5,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 76,
            distribution_period: 18.1,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 77,
            distribution_period: 17.7,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 78,
            distribution_period: 17.3,
        },
        JointDistributionPeriod {
            owner_age: 76,
            spouse_age: 79,
            distribution_period: 16.9,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 23,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 26,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 29,
            distribution_period: 56.4,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 31,
            distribution_period: 54.5,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 33,
            distribution_period: 52.6,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 34,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 35,
            distribution_period: 50.7,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 36,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 37,
            distribution_period: 48.8,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 38,
            distribution_period: 47.8,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 39,
            distribution_period: 46.9,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 40,
            distribution_period: 45.9,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 41,
            distribution_period: 45.0,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 42,
            distribution_period: 44.0,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 43,
            distribution_period: 43.1,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 44,
            distribution_period: 42.1,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 45,
            distribution_period: 41.2,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 46,
            distribution_period: 40.3,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 47,
            distribution_period: 39.3,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 48,
            distribution_period: 38.4,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 49,
            distribution_period: 37.5,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 50,
            distribution_period: 36.6,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 51,
            distribution_period: 35.7,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 52,
            distribution_period: 34.8,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 53,
            distribution_period: 33.9,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 54,
            distribution_period: 33.0,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 55,
            distribution_period: 32.2,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 56,
            distribution_period: 31.3,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 57,
            distribution_period: 30.5,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 58,
            distribution_period: 29.7,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 59,
            distribution_period: 28.8,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 60,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 61,
            distribution_period: 27.3,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 62,
            distribution_period: 26.5,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 63,
            distribution_period: 25.7,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 64,
            distribution_period: 25.0,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 65,
            distribution_period: 24.3,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 66,
            distribution_period: 23.5,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 67,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 68,
            distribution_period: 22.2,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 69,
            distribution_period: 21.5,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 70,
            distribution_period: 20.9,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 71,
            distribution_period: 20.3,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 72,
            distribution_period: 19.7,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 73,
            distribution_period: 19.1,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 74,
            distribution_period: 18.6,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 75,
            distribution_period: 18.1,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 76,
            distribution_period: 17.7,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 77,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 78,
            distribution_period: 16.8,
        },
        JointDistributionPeriod {
            owner_age: 77,
            spouse_age: 79,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 23,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 26,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 29,
            distribution_period: 56.4,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 31,
            distribution_period: 54.5,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 33,
            distribution_period: 52.6,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 34,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 36,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 38,
            distribution_period: 47.8,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 39,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 40,
            distribution_period: 45.9,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 41,
            distribution_period: 44.9,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 42,
            distribution_period: 44.0,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 43,
            distribution_period: 43.0,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 44,
            distribution_period: 42.1,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 45,
            distribution_period: 41.2,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 46,
            distribution_period: 40.2,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 47,
            distribution_period: 39.3,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 48,
            distribution_period: 38.4,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 49,
            distribution_period: 37.5,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 50,
            distribution_period: 36.5,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 51,
            distribution_period: 35.6,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 52,
            distribution_period: 34.7,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 53,
            distribution_period: 33.9,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 54,
            distribution_period: 33.0,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 55,
            distribution_period: 32.1,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 56,
            distribution_period: 31.2,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 57,
            distribution_period: 30.4,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 58,
            distribution_period: 29.6,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 59,
            distribution_period: 28.7,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 60,
            distribution_period: 27.9,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 61,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 62,
            distribution_period: 26.4,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 63,
            distribution_period: 25.6,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 64,
            distribution_period: 24.8,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 65,
            distribution_period: 24.1,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 66,
            distribution_period: 23.4,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 67,
            distribution_period: 22.7,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 68,
            distribution_period: 22.0,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 69,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 70,
            distribution_period: 20.6,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 71,
            distribution_period: 20.0,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 72,
            distribution_period: 19.4,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 73,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 74,
            distribution_period: 18.3,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 75,
            distribution_period: 17.8,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 76,
            distribution_period: 17.3,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 77,
            distribution_period: 16.8,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 78,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 78,
            spouse_age: 79,
            distribution_period: 16.0,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 23,
            distribution_period: 62.2,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 26,
            distribution_period: 59.3,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 29,
            distribution_period: 56.4,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 31,
            distribution_period: 54.5,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 34,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 36,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 38,
            distribution_period: 47.8,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 39,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 40,
            distribution_period: 45.9,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 41,
            distribution_period: 44.9,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 42,
            distribution_period: 44.0,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 43,
            distribution_period: 43.0,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 44,
            distribution_period: 42.1,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 45,
            distribution_period: 41.1,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 46,
            distribution_period: 40.2,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 47,
            distribution_period: 39.3,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 48,
            distribution_period: 38.3,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 49,
            distribution_period: 37.4,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 50,
            distribution_period: 36.5,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 51,
            distribution_period: 35.6,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 52,
            distribution_period: 34.7,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 53,
            distribution_period: 33.8,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 54,
            distribution_period: 32.9,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 55,
            distribution_period: 32.0,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 56,
            distribution_period: 31.2,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 57,
            distribution_period: 30.3,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 58,
            distribution_period: 29.5,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 59,
            distribution_period: 28.7,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 60,
            distribution_period: 27.8,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 61,
            distribution_period: 27.0,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 62,
            distribution_period: 26.2,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 63,
            distribution_period: 25.5,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 64,
            distribution_period: 24.7,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 65,
            distribution_period: 23.9,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 66,
            distribution_period: 23.2,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 67,
            distribution_period: 22.5,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 68,
            distribution_period: 21.8,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 69,
            distribution_period: 21.1,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 70,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 71,
            distribution_period: 19.8,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 72,
            distribution_period: 19.2,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 73,
            distribution_period: 18.6,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 74,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 75,
            distribution_period: 17.4,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 76,
            distribution_period: 16.9,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 77,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 78,
            distribution_period: 16.0,
        },
        JointDistributionPeriod {
            owner_age: 79,
            spouse_age: 79,
            distribution_period: 15.6,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 29,
            distribution_period: 56.4,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 34,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 36,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 38,
            distribution_period: 47.8,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 39,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 40,
            distribution_period: 45.9,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 41,
            distribution_period: 44.9,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 43,
            distribution_period: 43.0,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 44,
            distribution_period: 42.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 45,
            distribution_period: 41.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 46,
            distribution_period: 40.2,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 47,
            distribution_period: 39.2,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 48,
            distribution_period: 38.3,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 49,
            distribution_period: 37.4,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 50,
            distribution_period: 36.5,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 51,
            distribution_period: 35.5,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 52,
            distribution_period: 34.6,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 53,
            distribution_period: 33.7,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 54,
            distribution_period: 32.9,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 55,
            distribution_period: 32.0,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 56,
            distribution_period: 31.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 57,
            distribution_period: 30.3,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 58,
            distribution_period: 29.4,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 59,
            distribution_period: 28.6,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 60,
            distribution_period: 27.8,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 61,
            distribution_period: 26.9,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 62,
            distribution_period: 26.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 63,
            distribution_period: 25.3,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 64,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 65,
            distribution_period: 23.8,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 66,
            distribution_period: 23.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 67,
            distribution_period: 22.3,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 68,
            distribution_period: 21.6,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 69,
            distribution_period: 20.9,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 70,
            distribution_period: 20.2,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 71,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 72,
            distribution_period: 18.9,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 73,
            distribution_period: 18.3,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 74,
            distribution_period: 17.7,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 75,
            distribution_period: 17.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 76,
            distribution_period: 16.6,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 77,
            distribution_period: 16.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 78,
            distribution_period: 15.6,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 79,
            distribution_period: 15.2,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 80,
            distribution_period: 14.7,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 81,
            distribution_period: 14.4,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 82,
            distribution_period: 14.0,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 83,
            distribution_period: 13.7,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 84,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 85,
            distribution_period: 13.1,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 86,
            distribution_period: 12.9,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 87,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 88,
            distribution_period: 12.5,
        },
        JointDistributionPeriod {
            owner_age: 80,
            spouse_age: 89,
            distribution_period: 12.3,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 29,
            distribution_period: 56.4,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 34,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 36,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 39,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 41,
            distribution_period: 44.9,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 43,
            distribution_period: 43.0,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 44,
            distribution_period: 42.0,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 45,
            distribution_period: 41.1,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 46,
            distribution_period: 40.1,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 47,
            distribution_period: 39.2,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 48,
            distribution_period: 38.3,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 49,
            distribution_period: 37.3,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 50,
            distribution_period: 36.4,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 51,
            distribution_period: 35.5,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 52,
            distribution_period: 34.6,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 53,
            distribution_period: 33.7,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 54,
            distribution_period: 32.8,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 55,
            distribution_period: 31.9,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 56,
            distribution_period: 31.1,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 57,
            distribution_period: 30.2,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 58,
            distribution_period: 29.3,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 59,
            distribution_period: 28.5,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 60,
            distribution_period: 27.7,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 61,
            distribution_period: 26.9,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 62,
            distribution_period: 26.0,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 63,
            distribution_period: 25.2,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 64,
            distribution_period: 24.5,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 65,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 66,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 67,
            distribution_period: 22.2,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 68,
            distribution_period: 21.5,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 69,
            distribution_period: 20.7,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 70,
            distribution_period: 20.0,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 71,
            distribution_period: 19.4,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 72,
            distribution_period: 18.7,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 73,
            distribution_period: 18.1,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 74,
            distribution_period: 17.4,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 75,
            distribution_period: 16.9,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 76,
            distribution_period: 16.3,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 77,
            distribution_period: 15.8,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 78,
            distribution_period: 15.3,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 79,
            distribution_period: 14.8,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 80,
            distribution_period: 14.4,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 81,
            distribution_period: 14.0,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 82,
            distribution_period: 13.6,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 83,
            distribution_period: 13.2,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 84,
            distribution_period: 12.9,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 85,
            distribution_period: 12.6,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 86,
            distribution_period: 12.4,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 87,
            distribution_period: 12.2,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 88,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 81,
            spouse_age: 89,
            distribution_period: 11.8,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 34,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 36,
            distribution_period: 49.7,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 39,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 41,
            distribution_period: 44.9,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 43,
            distribution_period: 43.0,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 44,
            distribution_period: 42.0,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 45,
            distribution_period: 41.1,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 46,
            distribution_period: 40.1,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 47,
            distribution_period: 39.2,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 48,
            distribution_period: 38.3,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 49,
            distribution_period: 37.3,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 50,
            distribution_period: 36.4,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 51,
            distribution_period: 35.5,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 52,
            distribution_period: 34.6,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 53,
            distribution_period: 33.7,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 54,
            distribution_period: 32.8,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 55,
            distribution_period: 31.9,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 56,
            distribution_period: 31.0,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 57,
            distribution_period: 30.1,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 58,
            distribution_period: 29.3,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 59,
            distribution_period: 28.4,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 60,
            distribution_period: 27.6,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 61,
            distribution_period: 26.8,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 62,
            distribution_period: 26.0,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 63,
            distribution_period: 25.2,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 64,
            distribution_period: 24.4,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 65,
            distribution_period: 23.6,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 66,
            distribution_period: 22.8,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 67,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 68,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 69,
            distribution_period: 20.6,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 70,
            distribution_period: 19.9,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 71,
            distribution_period: 19.2,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 72,
            distribution_period: 18.5,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 73,
            distribution_period: 17.9,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 74,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 75,
            distribution_period: 16.6,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 76,
            distribution_period: 16.0,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 77,
            distribution_period: 15.5,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 78,
            distribution_period: 15.0,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 79,
            distribution_period: 14.5,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 80,
            distribution_period: 14.0,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 81,
            distribution_period: 13.6,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 82,
            distribution_period: 13.2,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 83,
            distribution_period: 12.8,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 84,
            distribution_period: 12.5,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 85,
            distribution_period: 12.2,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 86,
            distribution_period: 11.9,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 87,
            distribution_period: 11.7,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 88,
            distribution_period: 11.5,
        },
        JointDistributionPeriod {
            owner_age: 82,
            spouse_age: 89,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 34,
            distribution_period: 51.6,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 39,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 41,
            distribution_period: 44.9,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 43,
            distribution_period: 43.0,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 44,
            distribution_period: 42.0,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 45,
            distribution_period: 41.1,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 46,
            distribution_period: 40.1,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 47,
            distribution_period: 39.2,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 48,
            distribution_period: 38.2,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 49,
            distribution_period: 37.3,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 50,
            distribution_period: 36.4,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 51,
            distribution_period: 35.4,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 52,
            distribution_period: 34.5,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 53,
            distribution_period: 33.6,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 54,
            distribution_period: 32.7,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 55,
            distribution_period: 31.8,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 56,
            distribution_period: 31.0,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 57,
            distribution_period: 30.1,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 58,
            distribution_period: 29.2,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 59,
            distribution_period: 28.4,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 60,
            distribution_period: 27.5,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 61,
            distribution_period: 26.7,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 62,
            distribution_period: 25.9,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 63,
            distribution_period: 25.1,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 64,
            distribution_period: 24.3,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 65,
            distribution_period: 23.5,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 66,
            distribution_period: 22.7,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 67,
            distribution_period: 22.0,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 68,
            distribution_period: 21.2,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 69,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 70,
            distribution_period: 19.7,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 71,
            distribution_period: 19.0,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 72,
            distribution_period: 18.3,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 73,
            distribution_period: 17.7,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 74,
            distribution_period: 17.0,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 75,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 76,
            distribution_period: 15.8,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 77,
            distribution_period: 15.2,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 78,
            distribution_period: 14.7,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 79,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 80,
            distribution_period: 13.7,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 81,
            distribution_period: 13.2,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 82,
            distribution_period: 12.8,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 83,
            distribution_period: 12.4,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 84,
            distribution_period: 12.1,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 85,
            distribution_period: 11.8,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 86,
            distribution_period: 11.5,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 87,
            distribution_period: 11.2,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 88,
            distribution_period: 11.0,
        },
        JointDistributionPeriod {
            owner_age: 83,
            spouse_age: 89,
            distribution_period: 10.8,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 39,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 41,
            distribution_period: 44.9,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 44,
            distribution_period: 42.0,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 46,
            distribution_period: 40.1,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 47,
            distribution_period: 39.2,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 48,
            distribution_period: 38.2,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 49,
            distribution_period: 37.3,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 50,
            distribution_period: 36.3,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 51,
            distribution_period: 35.4,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 52,
            distribution_period: 34.5,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 53,
            distribution_period: 33.6,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 54,
            distribution_period: 32.7,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 55,
            distribution_period: 31.8,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 56,
            distribution_period: 30.9,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 57,
            distribution_period: 30.0,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 58,
            distribution_period: 29.2,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 59,
            distribution_period: 28.3,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 60,
            distribution_period: 27.5,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 61,
            distribution_period: 26.7,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 62,
            distribution_period: 25.8,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 63,
            distribution_period: 25.0,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 64,
            distribution_period: 24.2,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 65,
            distribution_period: 23.4,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 66,
            distribution_period: 22.6,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 67,
            distribution_period: 21.9,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 68,
            distribution_period: 21.1,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 69,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 70,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 71,
            distribution_period: 18.9,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 72,
            distribution_period: 18.2,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 73,
            distribution_period: 17.5,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 74,
            distribution_period: 16.8,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 75,
            distribution_period: 16.2,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 76,
            distribution_period: 15.6,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 77,
            distribution_period: 15.0,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 78,
            distribution_period: 14.4,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 79,
            distribution_period: 13.9,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 80,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 81,
            distribution_period: 12.9,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 82,
            distribution_period: 12.5,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 83,
            distribution_period: 12.1,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 84,
            distribution_period: 11.7,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 85,
            distribution_period: 11.4,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 86,
            distribution_period: 11.1,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 87,
            distribution_period: 10.8,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 88,
            distribution_period: 10.5,
        },
        JointDistributionPeriod {
            owner_age: 84,
            spouse_age: 89,
            distribution_period: 10.3,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 24,
            distribution_period: 61.2,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 27,
            distribution_period: 58.3,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 39,
            distribution_period: 46.8,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 44,
            distribution_period: 42.0,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 46,
            distribution_period: 40.1,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 48,
            distribution_period: 38.2,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 49,
            distribution_period: 37.3,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 50,
            distribution_period: 36.3,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 51,
            distribution_period: 35.4,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 52,
            distribution_period: 34.5,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 53,
            distribution_period: 33.6,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 54,
            distribution_period: 32.7,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 55,
            distribution_period: 31.8,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 56,
            distribution_period: 30.9,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 57,
            distribution_period: 30.0,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 58,
            distribution_period: 29.1,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 59,
            distribution_period: 28.3,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 60,
            distribution_period: 27.4,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 61,
            distribution_period: 26.6,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 62,
            distribution_period: 25.8,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 63,
            distribution_period: 25.0,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 64,
            distribution_period: 24.1,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 65,
            distribution_period: 23.3,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 66,
            distribution_period: 22.6,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 67,
            distribution_period: 21.8,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 68,
            distribution_period: 21.0,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 69,
            distribution_period: 20.3,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 70,
            distribution_period: 19.5,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 71,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 72,
            distribution_period: 18.1,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 73,
            distribution_period: 17.4,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 74,
            distribution_period: 16.7,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 75,
            distribution_period: 16.0,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 76,
            distribution_period: 15.4,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 77,
            distribution_period: 14.8,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 78,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 79,
            distribution_period: 13.6,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 80,
            distribution_period: 13.1,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 81,
            distribution_period: 12.6,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 82,
            distribution_period: 12.2,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 83,
            distribution_period: 11.8,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 84,
            distribution_period: 11.4,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 85,
            distribution_period: 11.0,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 86,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 87,
            distribution_period: 10.4,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 88,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 85,
            spouse_age: 89,
            distribution_period: 9.9,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 20,
            distribution_period: 65.1,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 32,
            distribution_period: 53.5,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 44,
            distribution_period: 42.0,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 46,
            distribution_period: 40.1,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 48,
            distribution_period: 38.2,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 50,
            distribution_period: 36.3,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 51,
            distribution_period: 35.4,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 52,
            distribution_period: 34.5,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 53,
            distribution_period: 33.5,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 54,
            distribution_period: 32.6,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 55,
            distribution_period: 31.7,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 56,
            distribution_period: 30.9,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 57,
            distribution_period: 30.0,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 58,
            distribution_period: 29.1,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 59,
            distribution_period: 28.2,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 60,
            distribution_period: 27.4,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 61,
            distribution_period: 26.6,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 62,
            distribution_period: 25.7,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 63,
            distribution_period: 24.9,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 64,
            distribution_period: 24.1,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 65,
            distribution_period: 23.3,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 66,
            distribution_period: 22.5,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 67,
            distribution_period: 21.7,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 68,
            distribution_period: 20.9,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 69,
            distribution_period: 20.2,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 70,
            distribution_period: 19.4,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 71,
            distribution_period: 18.7,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 72,
            distribution_period: 17.9,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 73,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 74,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 75,
            distribution_period: 15.9,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 76,
            distribution_period: 15.2,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 77,
            distribution_period: 14.6,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 78,
            distribution_period: 14.0,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 79,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 80,
            distribution_period: 12.9,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 81,
            distribution_period: 12.4,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 82,
            distribution_period: 11.9,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 83,
            distribution_period: 11.5,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 84,
            distribution_period: 11.1,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 85,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 86,
            distribution_period: 10.4,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 87,
            distribution_period: 10.0,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 88,
            distribution_period: 9.8,
        },
        JointDistributionPeriod {
            owner_age: 86,
            spouse_age: 89,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 44,
            distribution_period: 42.0,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 46,
            distribution_period: 40.1,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 48,
            distribution_period: 38.2,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 50,
            distribution_period: 36.3,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 51,
            distribution_period: 35.4,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 52,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 53,
            distribution_period: 33.5,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 54,
            distribution_period: 32.6,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 55,
            distribution_period: 31.7,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 56,
            distribution_period: 30.8,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 57,
            distribution_period: 29.9,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 58,
            distribution_period: 29.1,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 59,
            distribution_period: 28.2,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 60,
            distribution_period: 27.4,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 61,
            distribution_period: 26.5,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 62,
            distribution_period: 25.7,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 63,
            distribution_period: 24.9,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 64,
            distribution_period: 24.0,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 65,
            distribution_period: 23.2,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 66,
            distribution_period: 22.4,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 67,
            distribution_period: 21.6,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 68,
            distribution_period: 20.9,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 69,
            distribution_period: 20.1,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 70,
            distribution_period: 19.3,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 71,
            distribution_period: 18.6,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 72,
            distribution_period: 17.8,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 73,
            distribution_period: 17.1,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 74,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 75,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 76,
            distribution_period: 15.1,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 77,
            distribution_period: 14.4,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 78,
            distribution_period: 13.8,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 79,
            distribution_period: 13.2,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 80,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 81,
            distribution_period: 12.2,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 82,
            distribution_period: 11.7,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 83,
            distribution_period: 11.2,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 84,
            distribution_period: 10.8,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 85,
            distribution_period: 10.4,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 86,
            distribution_period: 10.0,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 87,
            distribution_period: 9.7,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 88,
            distribution_period: 9.4,
        },
        JointDistributionPeriod {
            owner_age: 87,
            spouse_age: 89,
            distribution_period: 9.1,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 44,
            distribution_period: 42.0,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 48,
            distribution_period: 38.2,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 50,
            distribution_period: 36.3,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 52,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 53,
            distribution_period: 33.5,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 54,
            distribution_period: 32.6,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 55,
            distribution_period: 31.7,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 56,
            distribution_period: 30.8,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 57,
            distribution_period: 29.9,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 58,
            distribution_period: 29.0,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 59,
            distribution_period: 28.2,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 60,
            distribution_period: 27.3,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 61,
            distribution_period: 26.5,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 62,
            distribution_period: 25.6,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 63,
            distribution_period: 24.8,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 64,
            distribution_period: 24.0,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 65,
            distribution_period: 23.2,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 66,
            distribution_period: 22.4,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 67,
            distribution_period: 21.6,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 68,
            distribution_period: 20.8,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 69,
            distribution_period: 20.0,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 70,
            distribution_period: 19.2,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 71,
            distribution_period: 18.5,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 72,
            distribution_period: 17.7,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 73,
            distribution_period: 17.0,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 74,
            distribution_period: 16.3,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 75,
            distribution_period: 15.6,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 76,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 77,
            distribution_period: 14.3,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 78,
            distribution_period: 13.7,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 79,
            distribution_period: 13.1,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 80,
            distribution_period: 12.5,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 81,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 82,
            distribution_period: 11.5,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 83,
            distribution_period: 11.0,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 84,
            distribution_period: 10.5,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 85,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 86,
            distribution_period: 9.8,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 87,
            distribution_period: 9.4,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 88,
            distribution_period: 9.1,
        },
        JointDistributionPeriod {
            owner_age: 88,
            spouse_age: 89,
            distribution_period: 8.8,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 37,
            distribution_period: 48.7,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 50,
            distribution_period: 36.3,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 52,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 53,
            distribution_period: 33.5,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 54,
            distribution_period: 32.6,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 55,
            distribution_period: 31.7,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 56,
            distribution_period: 30.8,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 57,
            distribution_period: 29.9,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 58,
            distribution_period: 29.0,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 59,
            distribution_period: 28.2,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 60,
            distribution_period: 27.3,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 61,
            distribution_period: 26.4,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 62,
            distribution_period: 25.6,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 63,
            distribution_period: 24.8,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 64,
            distribution_period: 24.0,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 65,
            distribution_period: 23.1,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 66,
            distribution_period: 22.3,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 67,
            distribution_period: 21.5,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 68,
            distribution_period: 20.7,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 69,
            distribution_period: 20.0,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 70,
            distribution_period: 19.2,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 71,
            distribution_period: 18.4,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 72,
            distribution_period: 17.7,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 73,
            distribution_period: 16.9,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 74,
            distribution_period: 16.2,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 75,
            distribution_period: 15.5,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 76,
            distribution_period: 14.8,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 77,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 78,
            distribution_period: 13.5,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 79,
            distribution_period: 12.9,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 80,
            distribution_period: 12.3,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 81,
            distribution_period: 11.8,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 82,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 83,
            distribution_period: 10.8,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 84,
            distribution_period: 10.3,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 85,
            distribution_period: 9.9,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 86,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 87,
            distribution_period: 9.1,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 88,
            distribution_period: 8.8,
        },
        JointDistributionPeriod {
            owner_age: 89,
            spouse_age: 89,
            distribution_period: 8.5,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 30,
            distribution_period: 55.4,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 50,
            distribution_period: 36.3,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 52,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 53,
            distribution_period: 33.5,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 54,
            distribution_period: 32.6,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 55,
            distribution_period: 31.7,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 56,
            distribution_period: 30.8,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 57,
            distribution_period: 29.9,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 58,
            distribution_period: 29.0,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 59,
            distribution_period: 28.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 60,
            distribution_period: 27.3,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 61,
            distribution_period: 26.4,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 62,
            distribution_period: 25.6,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 63,
            distribution_period: 24.7,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 64,
            distribution_period: 23.9,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 65,
            distribution_period: 23.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 66,
            distribution_period: 22.3,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 67,
            distribution_period: 21.5,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 68,
            distribution_period: 20.7,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 69,
            distribution_period: 19.9,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 70,
            distribution_period: 19.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 71,
            distribution_period: 18.4,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 72,
            distribution_period: 17.6,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 73,
            distribution_period: 16.9,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 74,
            distribution_period: 16.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 75,
            distribution_period: 15.4,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 76,
            distribution_period: 14.8,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 77,
            distribution_period: 14.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 78,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 79,
            distribution_period: 12.8,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 80,
            distribution_period: 12.2,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 81,
            distribution_period: 11.6,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 82,
            distribution_period: 11.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 83,
            distribution_period: 10.6,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 84,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 85,
            distribution_period: 9.7,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 86,
            distribution_period: 9.3,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 87,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 88,
            distribution_period: 8.6,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 89,
            distribution_period: 8.3,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 90,
            distribution_period: 8.0,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 91,
            distribution_period: 7.7,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 92,
            distribution_period: 7.5,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 93,
            distribution_period: 7.3,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 94,
            distribution_period: 7.1,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 95,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 96,
            distribution_period: 6.8,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 97,
            distribution_period: 6.7,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 98,
            distribution_period: 6.6,
        },
        JointDistributionPeriod {
            owner_age: 90,
            spouse_age: 99,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 42,
            distribution_period: 43.9,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 52,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 53,
            distribution_period: 33.5,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 57,
            distribution_period: 29.9,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 58,
            distribution_period: 29.0,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 59,
            distribution_period: 28.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 60,
            distribution_period: 27.3,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 61,
            distribution_period: 26.4,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 62,
            distribution_period: 25.6,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 63,
            distribution_period: 24.7,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 64,
            distribution_period: 23.9,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 65,
            distribution_period: 23.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 66,
            distribution_period: 22.3,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 67,
            distribution_period: 21.5,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 68,
            distribution_period: 20.7,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 69,
            distribution_period: 19.9,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 70,
            distribution_period: 19.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 71,
            distribution_period: 18.3,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 72,
            distribution_period: 17.5,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 73,
            distribution_period: 16.8,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 74,
            distribution_period: 16.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 75,
            distribution_period: 15.3,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 76,
            distribution_period: 14.6,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 77,
            distribution_period: 14.0,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 78,
            distribution_period: 13.3,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 79,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 80,
            distribution_period: 12.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 81,
            distribution_period: 11.5,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 82,
            distribution_period: 10.9,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 83,
            distribution_period: 10.4,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 84,
            distribution_period: 9.9,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 85,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 86,
            distribution_period: 9.1,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 87,
            distribution_period: 8.7,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 88,
            distribution_period: 8.3,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 89,
            distribution_period: 8.0,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 90,
            distribution_period: 7.7,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 91,
            distribution_period: 7.5,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 92,
            distribution_period: 7.2,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 93,
            distribution_period: 7.0,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 94,
            distribution_period: 6.8,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 95,
            distribution_period: 6.6,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 96,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 97,
            distribution_period: 6.4,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 98,
            distribution_period: 6.2,
        },
        JointDistributionPeriod {
            owner_age: 91,
            spouse_age: 99,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 52,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 53,
            distribution_period: 33.5,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 58,
            distribution_period: 29.0,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 59,
            distribution_period: 28.1,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 60,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 61,
            distribution_period: 26.4,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 62,
            distribution_period: 25.5,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 63,
            distribution_period: 24.7,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 64,
            distribution_period: 23.9,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 65,
            distribution_period: 23.0,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 66,
            distribution_period: 22.2,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 67,
            distribution_period: 21.4,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 68,
            distribution_period: 20.6,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 69,
            distribution_period: 19.8,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 70,
            distribution_period: 19.0,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 71,
            distribution_period: 18.3,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 72,
            distribution_period: 17.5,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 73,
            distribution_period: 16.7,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 74,
            distribution_period: 16.0,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 75,
            distribution_period: 15.3,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 76,
            distribution_period: 14.6,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 77,
            distribution_period: 13.9,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 78,
            distribution_period: 13.2,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 79,
            distribution_period: 12.6,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 80,
            distribution_period: 11.9,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 81,
            distribution_period: 11.4,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 82,
            distribution_period: 10.8,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 83,
            distribution_period: 10.3,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 84,
            distribution_period: 9.8,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 85,
            distribution_period: 9.3,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 86,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 87,
            distribution_period: 8.5,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 88,
            distribution_period: 8.1,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 89,
            distribution_period: 7.8,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 90,
            distribution_period: 7.5,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 91,
            distribution_period: 7.2,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 92,
            distribution_period: 7.0,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 93,
            distribution_period: 6.7,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 94,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 95,
            distribution_period: 6.4,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 96,
            distribution_period: 6.2,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 97,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 98,
            distribution_period: 5.9,
        },
        JointDistributionPeriod {
            owner_age: 92,
            spouse_age: 99,
            distribution_period: 5.8,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 52,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 58,
            distribution_period: 29.0,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 59,
            distribution_period: 28.1,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 60,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 61,
            distribution_period: 26.4,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 62,
            distribution_period: 25.5,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 63,
            distribution_period: 24.7,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 64,
            distribution_period: 23.8,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 65,
            distribution_period: 23.0,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 66,
            distribution_period: 22.2,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 67,
            distribution_period: 21.4,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 68,
            distribution_period: 20.6,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 69,
            distribution_period: 19.8,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 70,
            distribution_period: 19.0,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 71,
            distribution_period: 18.2,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 72,
            distribution_period: 17.4,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 73,
            distribution_period: 16.7,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 74,
            distribution_period: 15.9,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 75,
            distribution_period: 15.2,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 76,
            distribution_period: 14.5,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 77,
            distribution_period: 13.8,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 78,
            distribution_period: 13.1,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 79,
            distribution_period: 12.5,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 80,
            distribution_period: 11.9,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 81,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 82,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 83,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 84,
            distribution_period: 9.6,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 85,
            distribution_period: 9.2,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 86,
            distribution_period: 8.7,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 87,
            distribution_period: 8.3,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 88,
            distribution_period: 7.9,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 89,
            distribution_period: 7.6,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 90,
            distribution_period: 7.3,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 91,
            distribution_period: 7.0,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 92,
            distribution_period: 6.7,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 93,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 94,
            distribution_period: 6.3,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 95,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 96,
            distribution_period: 5.9,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 97,
            distribution_period: 5.8,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 98,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 93,
            spouse_age: 99,
            distribution_period: 5.5,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 52,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 59,
            distribution_period: 28.1,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 60,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 62,
            distribution_period: 25.5,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 63,
            distribution_period: 24.7,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 64,
            distribution_period: 23.8,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 65,
            distribution_period: 23.0,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 66,
            distribution_period: 22.2,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 67,
            distribution_period: 21.4,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 68,
            distribution_period: 20.6,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 69,
            distribution_period: 19.8,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 70,
            distribution_period: 19.0,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 71,
            distribution_period: 18.2,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 72,
            distribution_period: 17.4,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 73,
            distribution_period: 16.6,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 74,
            distribution_period: 15.9,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 75,
            distribution_period: 15.2,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 76,
            distribution_period: 14.4,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 77,
            distribution_period: 13.7,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 78,
            distribution_period: 13.1,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 79,
            distribution_period: 12.4,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 80,
            distribution_period: 11.8,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 81,
            distribution_period: 11.2,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 82,
            distribution_period: 10.6,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 83,
            distribution_period: 10.0,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 84,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 85,
            distribution_period: 9.0,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 86,
            distribution_period: 8.6,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 87,
            distribution_period: 8.2,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 88,
            distribution_period: 7.8,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 89,
            distribution_period: 7.4,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 90,
            distribution_period: 7.1,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 91,
            distribution_period: 6.8,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 92,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 93,
            distribution_period: 6.3,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 94,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 95,
            distribution_period: 5.9,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 96,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 97,
            distribution_period: 5.5,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 98,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 94,
            spouse_age: 99,
            distribution_period: 5.3,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 52,
            distribution_period: 34.4,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 59,
            distribution_period: 28.1,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 60,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 62,
            distribution_period: 25.5,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 64,
            distribution_period: 23.8,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 65,
            distribution_period: 23.0,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 66,
            distribution_period: 22.2,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 67,
            distribution_period: 21.4,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 68,
            distribution_period: 20.6,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 69,
            distribution_period: 19.7,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 70,
            distribution_period: 18.9,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 71,
            distribution_period: 18.2,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 72,
            distribution_period: 17.4,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 73,
            distribution_period: 16.6,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 74,
            distribution_period: 15.9,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 75,
            distribution_period: 15.1,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 76,
            distribution_period: 14.4,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 77,
            distribution_period: 13.7,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 78,
            distribution_period: 13.0,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 79,
            distribution_period: 12.3,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 80,
            distribution_period: 11.7,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 81,
            distribution_period: 11.1,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 82,
            distribution_period: 10.5,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 83,
            distribution_period: 9.9,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 84,
            distribution_period: 9.4,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 85,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 86,
            distribution_period: 8.5,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 87,
            distribution_period: 8.0,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 88,
            distribution_period: 7.6,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 89,
            distribution_period: 7.3,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 90,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 91,
            distribution_period: 6.6,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 92,
            distribution_period: 6.4,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 93,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 94,
            distribution_period: 5.9,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 95,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 96,
            distribution_period: 5.5,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 97,
            distribution_period: 5.3,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 98,
            distribution_period: 5.2,
        },
        JointDistributionPeriod {
            owner_age: 95,
            spouse_age: 99,
            distribution_period: 5.0,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 60,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 62,
            distribution_period: 25.5,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 64,
            distribution_period: 23.8,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 65,
            distribution_period: 23.0,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 66,
            distribution_period: 22.2,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 69,
            distribution_period: 19.7,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 70,
            distribution_period: 18.9,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 71,
            distribution_period: 18.1,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 72,
            distribution_period: 17.4,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 73,
            distribution_period: 16.6,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 74,
            distribution_period: 15.8,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 75,
            distribution_period: 15.1,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 76,
            distribution_period: 14.3,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 77,
            distribution_period: 13.6,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 78,
            distribution_period: 12.9,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 79,
            distribution_period: 12.3,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 80,
            distribution_period: 11.6,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 81,
            distribution_period: 11.0,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 82,
            distribution_period: 10.4,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 83,
            distribution_period: 9.9,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 84,
            distribution_period: 9.3,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 85,
            distribution_period: 8.8,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 86,
            distribution_period: 8.4,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 87,
            distribution_period: 7.9,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 88,
            distribution_period: 7.5,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 89,
            distribution_period: 7.1,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 90,
            distribution_period: 6.8,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 91,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 92,
            distribution_period: 6.2,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 93,
            distribution_period: 5.9,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 94,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 95,
            distribution_period: 5.5,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 96,
            distribution_period: 5.3,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 97,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 98,
            distribution_period: 5.0,
        },
        JointDistributionPeriod {
            owner_age: 96,
            spouse_age: 99,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 60,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 62,
            distribution_period: 25.5,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 64,
            distribution_period: 23.8,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 65,
            distribution_period: 23.0,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 69,
            distribution_period: 19.7,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 70,
            distribution_period: 18.9,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 71,
            distribution_period: 18.1,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 72,
            distribution_period: 17.3,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 73,
            distribution_period: 16.6,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 74,
            distribution_period: 15.8,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 75,
            distribution_period: 15.0,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 76,
            distribution_period: 14.3,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 77,
            distribution_period: 13.6,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 78,
            distribution_period: 12.9,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 79,
            distribution_period: 12.2,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 80,
            distribution_period: 11.6,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 81,
            distribution_period: 11.0,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 82,
            distribution_period: 10.4,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 83,
            distribution_period: 9.8,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 84,
            distribution_period: 9.2,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 85,
            distribution_period: 8.7,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 86,
            distribution_period: 8.3,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 87,
            distribution_period: 7.8,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 88,
            distribution_period: 7.4,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 89,
            distribution_period: 7.0,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 90,
            distribution_period: 6.7,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 91,
            distribution_period: 6.4,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 92,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 93,
            distribution_period: 5.8,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 94,
            distribution_period: 5.5,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 95,
            distribution_period: 5.3,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 96,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 97,
            distribution_period: 4.9,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 98,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 97,
            spouse_age: 99,
            distribution_period: 4.6,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 60,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 62,
            distribution_period: 25.5,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 64,
            distribution_period: 23.8,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 69,
            distribution_period: 19.7,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 70,
            distribution_period: 18.9,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 71,
            distribution_period: 18.1,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 72,
            distribution_period: 17.3,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 73,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 74,
            distribution_period: 15.8,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 75,
            distribution_period: 15.0,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 76,
            distribution_period: 14.3,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 77,
            distribution_period: 13.6,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 78,
            distribution_period: 12.9,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 79,
            distribution_period: 12.2,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 80,
            distribution_period: 11.5,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 81,
            distribution_period: 10.9,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 82,
            distribution_period: 10.3,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 83,
            distribution_period: 9.7,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 84,
            distribution_period: 9.2,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 85,
            distribution_period: 8.7,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 86,
            distribution_period: 8.2,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 87,
            distribution_period: 7.7,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 88,
            distribution_period: 7.3,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 89,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 90,
            distribution_period: 6.6,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 91,
            distribution_period: 6.2,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 92,
            distribution_period: 5.9,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 93,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 94,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 95,
            distribution_period: 5.2,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 96,
            distribution_period: 5.0,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 97,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 98,
            distribution_period: 4.6,
        },
        JointDistributionPeriod {
            owner_age: 98,
            spouse_age: 99,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 47,
            distribution_period: 39.1,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 49,
            distribution_period: 37.2,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 60,
            distribution_period: 27.2,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 64,
            distribution_period: 23.8,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 69,
            distribution_period: 19.7,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 70,
            distribution_period: 18.9,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 71,
            distribution_period: 18.1,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 72,
            distribution_period: 17.3,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 73,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 75,
            distribution_period: 15.0,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 76,
            distribution_period: 14.3,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 77,
            distribution_period: 13.5,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 78,
            distribution_period: 12.8,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 79,
            distribution_period: 12.2,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 80,
            distribution_period: 11.5,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 81,
            distribution_period: 10.9,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 82,
            distribution_period: 10.2,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 83,
            distribution_period: 9.7,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 84,
            distribution_period: 9.1,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 85,
            distribution_period: 8.6,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 86,
            distribution_period: 8.1,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 87,
            distribution_period: 7.6,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 88,
            distribution_period: 7.2,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 89,
            distribution_period: 6.8,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 90,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 91,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 92,
            distribution_period: 5.8,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 93,
            distribution_period: 5.5,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 94,
            distribution_period: 5.3,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 95,
            distribution_period: 5.0,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 96,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 97,
            distribution_period: 4.6,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 98,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 99,
            spouse_age: 99,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 64,
            distribution_period: 23.8,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 69,
            distribution_period: 19.7,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 70,
            distribution_period: 18.9,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 71,
            distribution_period: 18.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 72,
            distribution_period: 17.3,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 73,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 75,
            distribution_period: 15.0,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 77,
            distribution_period: 13.5,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 78,
            distribution_period: 12.8,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 79,
            distribution_period: 12.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 80,
            distribution_period: 11.5,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 81,
            distribution_period: 10.8,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 82,
            distribution_period: 10.2,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 83,
            distribution_period: 9.6,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 84,
            distribution_period: 9.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 85,
            distribution_period: 8.5,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 86,
            distribution_period: 8.0,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 87,
            distribution_period: 7.6,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 88,
            distribution_period: 7.2,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 89,
            distribution_period: 6.8,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 90,
            distribution_period: 6.4,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 91,
            distribution_period: 6.0,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 92,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 93,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 94,
            distribution_period: 5.2,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 95,
            distribution_period: 4.9,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 96,
            distribution_period: 4.7,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 97,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 98,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 99,
            distribution_period: 4.2,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 100,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 101,
            distribution_period: 3.9,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 102,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 103,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 104,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 105,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 106,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 107,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 108,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 100,
            spouse_age: 109,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 64,
            distribution_period: 23.8,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 69,
            distribution_period: 19.7,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 70,
            distribution_period: 18.9,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 71,
            distribution_period: 18.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 72,
            distribution_period: 17.3,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 73,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 75,
            distribution_period: 15.0,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 77,
            distribution_period: 13.5,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 78,
            distribution_period: 12.8,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 79,
            distribution_period: 12.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 80,
            distribution_period: 11.4,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 81,
            distribution_period: 10.8,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 82,
            distribution_period: 10.2,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 83,
            distribution_period: 9.6,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 84,
            distribution_period: 9.0,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 85,
            distribution_period: 8.5,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 86,
            distribution_period: 8.0,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 87,
            distribution_period: 7.5,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 88,
            distribution_period: 7.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 89,
            distribution_period: 6.7,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 90,
            distribution_period: 6.3,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 91,
            distribution_period: 6.0,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 92,
            distribution_period: 5.6,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 93,
            distribution_period: 5.3,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 94,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 95,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 96,
            distribution_period: 4.6,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 97,
            distribution_period: 4.4,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 98,
            distribution_period: 4.2,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 99,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 100,
            distribution_period: 3.9,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 101,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 102,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 103,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 104,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 105,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 106,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 107,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 108,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 101,
            spouse_age: 109,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 35,
            distribution_period: 50.6,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 69,
            distribution_period: 19.7,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 72,
            distribution_period: 17.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 73,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 77,
            distribution_period: 13.5,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 78,
            distribution_period: 12.8,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 79,
            distribution_period: 12.1,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 80,
            distribution_period: 11.4,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 81,
            distribution_period: 10.8,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 82,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 83,
            distribution_period: 9.6,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 84,
            distribution_period: 9.0,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 85,
            distribution_period: 8.5,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 86,
            distribution_period: 8.0,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 87,
            distribution_period: 7.5,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 88,
            distribution_period: 7.0,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 89,
            distribution_period: 6.6,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 90,
            distribution_period: 6.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 91,
            distribution_period: 5.9,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 92,
            distribution_period: 5.6,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 93,
            distribution_period: 5.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 94,
            distribution_period: 5.0,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 95,
            distribution_period: 4.7,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 96,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 97,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 98,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 99,
            distribution_period: 4.0,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 100,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 101,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 102,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 103,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 104,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 105,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 106,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 107,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 108,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 102,
            spouse_age: 109,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 72,
            distribution_period: 17.3,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 73,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 77,
            distribution_period: 13.5,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 78,
            distribution_period: 12.8,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 79,
            distribution_period: 12.1,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 80,
            distribution_period: 11.4,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 82,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 83,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 84,
            distribution_period: 9.0,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 85,
            distribution_period: 8.4,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 86,
            distribution_period: 7.9,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 87,
            distribution_period: 7.4,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 88,
            distribution_period: 7.0,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 89,
            distribution_period: 6.6,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 90,
            distribution_period: 6.2,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 91,
            distribution_period: 5.9,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 92,
            distribution_period: 5.5,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 93,
            distribution_period: 5.2,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 94,
            distribution_period: 4.9,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 95,
            distribution_period: 4.7,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 96,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 97,
            distribution_period: 4.2,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 98,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 99,
            distribution_period: 3.9,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 100,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 101,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 102,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 103,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 104,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 105,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 106,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 107,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 108,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 103,
            spouse_age: 109,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 40,
            distribution_period: 45.8,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 73,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 77,
            distribution_period: 13.5,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 80,
            distribution_period: 11.4,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 82,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 83,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 84,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 85,
            distribution_period: 8.4,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 86,
            distribution_period: 7.9,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 87,
            distribution_period: 7.4,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 88,
            distribution_period: 7.0,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 89,
            distribution_period: 6.6,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 90,
            distribution_period: 6.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 91,
            distribution_period: 5.8,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 92,
            distribution_period: 5.5,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 93,
            distribution_period: 5.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 94,
            distribution_period: 4.9,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 95,
            distribution_period: 4.6,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 96,
            distribution_period: 4.4,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 97,
            distribution_period: 4.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 98,
            distribution_period: 4.0,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 99,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 100,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 101,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 102,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 103,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 104,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 105,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 106,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 107,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 108,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 104,
            spouse_age: 109,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 73,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 80,
            distribution_period: 11.4,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 82,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 83,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 84,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 85,
            distribution_period: 8.4,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 86,
            distribution_period: 7.9,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 87,
            distribution_period: 7.4,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 88,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 89,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 90,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 91,
            distribution_period: 5.8,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 92,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 93,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 94,
            distribution_period: 4.9,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 95,
            distribution_period: 4.6,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 96,
            distribution_period: 4.4,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 97,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 98,
            distribution_period: 4.0,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 99,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 100,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 101,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 102,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 103,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 104,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 105,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 106,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 107,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 108,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 105,
            spouse_age: 109,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 73,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 80,
            distribution_period: 11.4,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 82,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 83,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 84,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 85,
            distribution_period: 8.4,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 86,
            distribution_period: 7.9,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 87,
            distribution_period: 7.4,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 88,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 89,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 90,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 91,
            distribution_period: 5.8,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 92,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 93,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 94,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 95,
            distribution_period: 4.6,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 96,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 97,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 98,
            distribution_period: 3.9,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 99,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 100,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 101,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 102,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 103,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 104,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 105,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 106,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 107,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 108,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 106,
            spouse_age: 109,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 73,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 80,
            distribution_period: 11.4,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 82,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 83,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 84,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 85,
            distribution_period: 8.4,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 86,
            distribution_period: 7.9,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 87,
            distribution_period: 7.4,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 88,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 89,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 90,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 91,
            distribution_period: 5.8,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 92,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 93,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 94,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 95,
            distribution_period: 4.6,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 96,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 97,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 98,
            distribution_period: 3.9,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 99,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 100,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 101,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 102,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 103,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 104,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 105,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 106,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 107,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 108,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 107,
            spouse_age: 109,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 68,
            distribution_period: 20.5,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 73,
            distribution_period: 16.5,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 80,
            distribution_period: 11.4,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 82,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 83,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 84,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 85,
            distribution_period: 8.4,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 86,
            distribution_period: 7.8,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 87,
            distribution_period: 7.4,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 88,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 89,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 90,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 91,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 92,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 93,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 94,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 95,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 96,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 97,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 98,
            distribution_period: 3.9,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 99,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 100,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 101,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 102,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 103,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 104,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 105,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 106,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 107,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 108,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 108,
            spouse_age: 109,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 80,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 82,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 83,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 84,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 85,
            distribution_period: 8.4,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 86,
            distribution_period: 7.8,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 87,
            distribution_period: 7.4,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 88,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 89,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 90,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 91,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 92,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 93,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 94,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 95,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 96,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 97,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 98,
            distribution_period: 3.9,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 99,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 100,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 101,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 102,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 103,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 104,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 105,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 106,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 107,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 108,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 109,
            spouse_age: 109,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 80,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 82,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 83,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 84,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 85,
            distribution_period: 8.3,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 86,
            distribution_period: 7.8,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 87,
            distribution_period: 7.4,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 88,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 89,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 90,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 91,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 92,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 93,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 94,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 95,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 96,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 97,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 98,
            distribution_period: 3.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 99,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 100,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 101,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 102,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 103,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 104,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 105,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 106,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 107,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 108,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 109,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 110,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 111,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 112,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 113,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 114,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 115,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 116,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 117,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 118,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 119,
            distribution_period: 2.2,
        },
        JointDistributionPeriod {
            owner_age: 110,
            spouse_age: 120,
            distribution_period: 2.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 80,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 82,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 83,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 84,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 85,
            distribution_period: 8.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 86,
            distribution_period: 7.8,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 87,
            distribution_period: 7.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 88,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 89,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 90,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 91,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 92,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 93,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 94,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 95,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 96,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 97,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 98,
            distribution_period: 3.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 99,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 100,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 101,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 102,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 103,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 104,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 105,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 106,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 107,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 108,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 109,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 110,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 111,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 112,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 113,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 114,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 115,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 116,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 117,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 118,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 119,
            distribution_period: 2.2,
        },
        JointDistributionPeriod {
            owner_age: 111,
            spouse_age: 120,
            distribution_period: 2.0,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 80,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 82,
            distribution_period: 10.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 83,
            distribution_period: 9.5,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 84,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 85,
            distribution_period: 8.3,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 86,
            distribution_period: 7.8,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 87,
            distribution_period: 7.3,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 88,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 89,
            distribution_period: 6.5,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 90,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 91,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 92,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 93,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 94,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 95,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 96,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 97,
            distribution_period: 4.0,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 98,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 99,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 100,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 101,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 102,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 103,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 104,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 105,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 106,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 107,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 108,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 109,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 110,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 111,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 112,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 113,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 114,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 115,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 116,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 117,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 118,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 119,
            distribution_period: 2.2,
        },
        JointDistributionPeriod {
            owner_age: 112,
            spouse_age: 120,
            distribution_period: 2.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 76,
            distribution_period: 14.2,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 80,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 82,
            distribution_period: 10.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 83,
            distribution_period: 9.4,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 84,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 85,
            distribution_period: 8.3,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 86,
            distribution_period: 7.8,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 87,
            distribution_period: 7.3,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 88,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 89,
            distribution_period: 6.4,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 90,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 91,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 92,
            distribution_period: 5.3,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 93,
            distribution_period: 5.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 94,
            distribution_period: 4.7,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 95,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 96,
            distribution_period: 4.2,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 97,
            distribution_period: 4.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 98,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 99,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 100,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 101,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 102,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 103,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 104,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 105,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 106,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 107,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 108,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 109,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 110,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 111,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 112,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 113,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 114,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 115,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 116,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 117,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 118,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 119,
            distribution_period: 2.2,
        },
        JointDistributionPeriod {
            owner_age: 113,
            spouse_age: 120,
            distribution_period: 1.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 76,
            distribution_period: 14.1,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 80,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 82,
            distribution_period: 10.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 83,
            distribution_period: 9.4,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 84,
            distribution_period: 8.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 85,
            distribution_period: 8.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 86,
            distribution_period: 7.8,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 87,
            distribution_period: 7.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 88,
            distribution_period: 6.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 89,
            distribution_period: 6.4,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 90,
            distribution_period: 6.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 91,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 92,
            distribution_period: 5.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 93,
            distribution_period: 5.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 94,
            distribution_period: 4.7,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 95,
            distribution_period: 4.4,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 96,
            distribution_period: 4.2,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 97,
            distribution_period: 4.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 98,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 99,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 100,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 101,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 102,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 103,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 104,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 105,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 106,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 107,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 108,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 109,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 110,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 111,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 112,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 113,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 114,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 115,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 116,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 117,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 118,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 119,
            distribution_period: 2.1,
        },
        JointDistributionPeriod {
            owner_age: 114,
            spouse_age: 120,
            distribution_period: 1.9,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 74,
            distribution_period: 15.7,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 76,
            distribution_period: 14.1,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 80,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 81,
            distribution_period: 10.7,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 82,
            distribution_period: 10.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 83,
            distribution_period: 9.4,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 84,
            distribution_period: 8.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 85,
            distribution_period: 8.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 86,
            distribution_period: 7.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 87,
            distribution_period: 7.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 88,
            distribution_period: 6.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 89,
            distribution_period: 6.4,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 90,
            distribution_period: 6.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 91,
            distribution_period: 5.6,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 92,
            distribution_period: 5.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 93,
            distribution_period: 5.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 94,
            distribution_period: 4.7,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 95,
            distribution_period: 4.4,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 96,
            distribution_period: 4.2,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 97,
            distribution_period: 4.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 98,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 99,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 100,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 101,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 102,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 103,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 104,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 105,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 106,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 107,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 108,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 109,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 110,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 111,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 112,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 113,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 114,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 115,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 116,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 117,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 118,
            distribution_period: 2.3,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 119,
            distribution_period: 2.1,
        },
        JointDistributionPeriod {
            owner_age: 115,
            spouse_age: 120,
            distribution_period: 1.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 67,
            distribution_period: 21.3,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 74,
            distribution_period: 15.6,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 76,
            distribution_period: 14.1,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 80,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 81,
            distribution_period: 10.6,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 82,
            distribution_period: 10.0,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 83,
            distribution_period: 9.4,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 84,
            distribution_period: 8.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 85,
            distribution_period: 8.3,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 86,
            distribution_period: 7.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 87,
            distribution_period: 7.3,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 88,
            distribution_period: 6.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 89,
            distribution_period: 6.4,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 90,
            distribution_period: 6.0,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 91,
            distribution_period: 5.6,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 92,
            distribution_period: 5.2,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 93,
            distribution_period: 4.9,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 94,
            distribution_period: 4.6,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 95,
            distribution_period: 4.4,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 96,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 97,
            distribution_period: 3.9,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 98,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 99,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 100,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 101,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 102,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 103,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 104,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 105,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 106,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 107,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 108,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 109,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 110,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 111,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 112,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 113,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 114,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 115,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 116,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 117,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 118,
            distribution_period: 2.2,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 119,
            distribution_period: 2.0,
        },
        JointDistributionPeriod {
            owner_age: 116,
            spouse_age: 120,
            distribution_period: 1.8,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 63,
            distribution_period: 24.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 67,
            distribution_period: 21.2,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 74,
            distribution_period: 15.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 76,
            distribution_period: 14.1,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 78,
            distribution_period: 12.7,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 79,
            distribution_period: 12.0,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 80,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 81,
            distribution_period: 10.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 82,
            distribution_period: 10.0,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 83,
            distribution_period: 9.4,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 84,
            distribution_period: 8.8,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 85,
            distribution_period: 8.2,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 86,
            distribution_period: 7.7,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 87,
            distribution_period: 7.2,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 88,
            distribution_period: 6.8,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 89,
            distribution_period: 6.3,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 90,
            distribution_period: 5.9,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 91,
            distribution_period: 5.5,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 92,
            distribution_period: 5.2,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 93,
            distribution_period: 4.9,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 94,
            distribution_period: 4.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 95,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 96,
            distribution_period: 4.0,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 97,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 98,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 99,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 100,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 101,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 102,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 103,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 104,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 105,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 106,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 107,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 108,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 109,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 110,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 111,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 112,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 113,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 114,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 115,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 116,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 117,
            distribution_period: 2.3,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 118,
            distribution_period: 2.1,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 119,
            distribution_period: 1.9,
        },
        JointDistributionPeriod {
            owner_age: 117,
            spouse_age: 120,
            distribution_period: 1.6,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 61,
            distribution_period: 26.3,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 63,
            distribution_period: 24.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 67,
            distribution_period: 21.2,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 74,
            distribution_period: 15.6,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 75,
            distribution_period: 14.9,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 76,
            distribution_period: 14.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 78,
            distribution_period: 12.6,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 79,
            distribution_period: 11.9,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 80,
            distribution_period: 11.3,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 81,
            distribution_period: 10.6,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 82,
            distribution_period: 10.0,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 83,
            distribution_period: 9.3,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 84,
            distribution_period: 8.8,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 85,
            distribution_period: 8.2,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 86,
            distribution_period: 7.7,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 87,
            distribution_period: 7.2,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 88,
            distribution_period: 6.7,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 89,
            distribution_period: 6.3,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 90,
            distribution_period: 5.8,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 91,
            distribution_period: 5.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 92,
            distribution_period: 5.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 93,
            distribution_period: 4.8,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 94,
            distribution_period: 4.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 95,
            distribution_period: 4.2,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 96,
            distribution_period: 3.9,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 97,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 98,
            distribution_period: 3.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 99,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 100,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 101,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 102,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 103,
            distribution_period: 2.7,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 104,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 105,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 106,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 107,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 108,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 109,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 110,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 111,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 112,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 113,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 114,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 115,
            distribution_period: 2.3,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 116,
            distribution_period: 2.2,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 117,
            distribution_period: 2.1,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 118,
            distribution_period: 1.9,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 119,
            distribution_period: 1.7,
        },
        JointDistributionPeriod {
            owner_age: 118,
            spouse_age: 120,
            distribution_period: 1.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 56,
            distribution_period: 30.7,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 61,
            distribution_period: 26.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 63,
            distribution_period: 24.5,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 66,
            distribution_period: 22.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 67,
            distribution_period: 21.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 74,
            distribution_period: 15.6,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 75,
            distribution_period: 14.8,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 76,
            distribution_period: 14.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 77,
            distribution_period: 13.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 78,
            distribution_period: 12.6,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 79,
            distribution_period: 11.9,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 80,
            distribution_period: 11.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 81,
            distribution_period: 10.6,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 82,
            distribution_period: 9.9,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 83,
            distribution_period: 9.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 84,
            distribution_period: 8.7,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 85,
            distribution_period: 8.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 86,
            distribution_period: 7.6,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 87,
            distribution_period: 7.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 88,
            distribution_period: 6.6,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 89,
            distribution_period: 6.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 90,
            distribution_period: 5.8,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 91,
            distribution_period: 5.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 92,
            distribution_period: 5.0,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 93,
            distribution_period: 4.7,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 94,
            distribution_period: 4.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 95,
            distribution_period: 4.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 96,
            distribution_period: 3.8,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 97,
            distribution_period: 3.6,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 98,
            distribution_period: 3.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 99,
            distribution_period: 3.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 100,
            distribution_period: 2.9,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 101,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 102,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 103,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 104,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 105,
            distribution_period: 2.4,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 106,
            distribution_period: 2.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 107,
            distribution_period: 2.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 108,
            distribution_period: 2.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 109,
            distribution_period: 2.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 110,
            distribution_period: 2.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 111,
            distribution_period: 2.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 112,
            distribution_period: 2.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 113,
            distribution_period: 2.2,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 114,
            distribution_period: 2.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 115,
            distribution_period: 2.1,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 116,
            distribution_period: 2.0,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 117,
            distribution_period: 1.9,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 118,
            distribution_period: 1.7,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 119,
            distribution_period: 1.3,
        },
        JointDistributionPeriod {
            owner_age: 119,
            spouse_age: 120,
            distribution_period: 1.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 20,
            distribution_period: 65.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 21,
            distribution_period: 64.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 22,
            distribution_period: 63.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 23,
            distribution_period: 62.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 24,
            distribution_period: 61.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 25,
            distribution_period: 60.2,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 26,
            distribution_period: 59.2,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 27,
            distribution_period: 58.2,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 28,
            distribution_period: 57.3,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 29,
            distribution_period: 56.3,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 30,
            distribution_period: 55.3,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 31,
            distribution_period: 54.4,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 32,
            distribution_period: 53.4,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 33,
            distribution_period: 52.5,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 34,
            distribution_period: 51.5,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 35,
            distribution_period: 50.5,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 36,
            distribution_period: 49.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 37,
            distribution_period: 48.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 38,
            distribution_period: 47.7,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 39,
            distribution_period: 46.7,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 40,
            distribution_period: 45.7,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 41,
            distribution_period: 44.8,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 42,
            distribution_period: 43.8,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 43,
            distribution_period: 42.9,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 44,
            distribution_period: 41.9,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 45,
            distribution_period: 41.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 46,
            distribution_period: 40.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 47,
            distribution_period: 39.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 48,
            distribution_period: 38.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 49,
            distribution_period: 37.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 50,
            distribution_period: 36.2,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 51,
            distribution_period: 35.3,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 52,
            distribution_period: 34.3,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 53,
            distribution_period: 33.4,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 54,
            distribution_period: 32.5,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 55,
            distribution_period: 31.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 56,
            distribution_period: 30.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 57,
            distribution_period: 29.8,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 58,
            distribution_period: 28.9,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 59,
            distribution_period: 28.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 60,
            distribution_period: 27.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 61,
            distribution_period: 26.2,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 62,
            distribution_period: 25.4,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 63,
            distribution_period: 24.5,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 64,
            distribution_period: 23.7,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 65,
            distribution_period: 22.9,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 66,
            distribution_period: 22.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 67,
            distribution_period: 21.2,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 68,
            distribution_period: 20.4,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 69,
            distribution_period: 19.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 70,
            distribution_period: 18.8,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 71,
            distribution_period: 18.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 72,
            distribution_period: 17.2,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 73,
            distribution_period: 16.4,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 74,
            distribution_period: 15.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 75,
            distribution_period: 14.8,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 76,
            distribution_period: 14.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 77,
            distribution_period: 13.3,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 78,
            distribution_period: 12.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 79,
            distribution_period: 11.9,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 80,
            distribution_period: 11.2,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 81,
            distribution_period: 10.5,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 82,
            distribution_period: 9.9,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 83,
            distribution_period: 9.3,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 84,
            distribution_period: 8.7,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 85,
            distribution_period: 8.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 86,
            distribution_period: 7.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 87,
            distribution_period: 7.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 88,
            distribution_period: 6.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 89,
            distribution_period: 6.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 90,
            distribution_period: 5.7,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 91,
            distribution_period: 5.3,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 92,
            distribution_period: 4.9,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 93,
            distribution_period: 4.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 94,
            distribution_period: 4.3,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 95,
            distribution_period: 4.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 96,
            distribution_period: 3.7,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 97,
            distribution_period: 3.4,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 98,
            distribution_period: 3.2,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 99,
            distribution_period: 3.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 100,
            distribution_period: 2.8,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 101,
            distribution_period: 2.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 102,
            distribution_period: 2.5,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 103,
            distribution_period: 2.3,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 104,
            distribution_period: 2.2,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 105,
            distribution_period: 2.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 106,
            distribution_period: 2.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 107,
            distribution_period: 2.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 108,
            distribution_period: 2.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 109,
            distribution_period: 2.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 110,
            distribution_period: 2.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 111,
            distribution_period: 2.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 112,
            distribution_period: 2.0,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 113,
            distribution_period: 1.9,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 114,
            distribution_period: 1.9,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 115,
            distribution_period: 1.8,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 116,
            distribution_period: 1.8,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 117,
            distribution_period: 1.6,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 118,
            distribution_period: 1.4,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 119,
            distribution_period: 1.1,
        },
        JointDistributionPeriod {
            owner_age: 120,
            spouse_age: 120,
            distribution_period: 1.0,
        },
    ]
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
    fn uniform_lifetime_last_age() {
        let table = uniform_lifetime();
        let entry = table.last().unwrap();
        assert_eq!(entry.age, 120);
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
    fn single_life_ends_at_last_age() {
        let table = single_life();
        let last = table.last().unwrap();
        assert_eq!(last.age, 120);
        assert_eq!(last.distribution_period, 1.0);
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
        assert_eq!(entry.distribution_period, 20.2);
    }
}
