

#[cfg(test)]
pub mod test_runner {
    use crate::experiments::{ex_vars, ex_rand, ex_condition};


    #[test]
    pub fn test_assert() {
        assert!(true);
    }

    #[test]
    #[ignore]
    pub fn test_assert_eq() {
        assert_eq!(1, 102);
    }

    #[test]
    #[should_panic]
    pub fn test_should_panic() {
        assert!(true);
        assert_eq!(1, 1);
        panic!();
    }

    // ~~~~~ Real Tests ~~~~

    #[test]
    pub fn test_experiment_variables() {
        let result = ex_vars::experiment_variables();
        assert_eq!(result.gender, 'M');
        assert_eq!(result.name, String::from("Mahmoud"));
    }

    #[test]
    pub fn test_experiment_conditionals() {
        let person = ex_vars::experiment_variables();
        let result = ex_condition::experiment_conditionals(&person, false);

        assert_eq!(result, true);
    }

    #[test]
    pub fn test_experiment_enums() {
        let result = ex_vars::experiment_enums();
        assert!(result);
    }

    #[test]
    pub fn test_experiment_random_person() {
        let result = ex_rand::experiment_random_person();
        assert!((result.gender == 'M') | (result.gender == 'F'));
        assert!(
            (result.dob.day < 32) &&
            (result.dob.month < 13) &&
            (result.dob.year < 2022)
        );
    }

}