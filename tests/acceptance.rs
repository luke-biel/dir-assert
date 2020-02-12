use std::path::PathBuf;

fn test_root() -> PathBuf {
    PathBuf::from("tests").join("data")
}

mod when_dir_contents_match {
    use dir_assert::assert_paths;
    use std::path::PathBuf;
    use test_case::test_case;

    fn test_root() -> PathBuf {
        super::test_root().join("when_dir_contents_match")
    }

    #[test_case("file"         ; "when comparing single file")]
    #[test_case("simple"       ; "when directories contain one file each")]
    #[test_case("deep"         ; "when files are stored inside subdirectories")]
    #[test_case("complex"      ; "when files are scattered inside dir")]
    #[test_case("file_symlink" ; "when comparing symlinked files")]
    fn is_ok(dir: &str) {
        assert_paths(
            test_root().join(dir).join("actual"),
            test_root().join(dir).join("expected"),
        )
        .unwrap()
    }

    #[test]
    fn does_not_panic() {
        assert_paths!(
            test_root().join("simple").join("actual"),
            test_root().join("simple").join("expected")
        )
    }
}

mod when_dir_contents_do_not_match {
    use cfg_if::cfg_if;
    use dir_assert::assert_paths;
    use std::path::PathBuf;
    use test_case::test_case;

    fn test_root() -> PathBuf {
        super::test_root().join("when_dir_contents_do_not_match")
    }

    fn run(case: TestCase) {
        let actual = assert_paths(
            test_root().join(case.dir).join("actual"),
            test_root().join(case.dir).join("expected"),
        )
        .unwrap_err()
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>();

        let expected = case
            .expected
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|s| s.replace("{SEP}", separator()))
            .collect::<Vec<_>>();

        assert_eq!(actual, expected)
    }

    // For some reason windows wants double dashes
    fn separator() -> &'static str {
        cfg_if! {
            if #[cfg(windows)] {
                "\\\\"
            } else {
                "/"
            }
        }
    }

    struct TestCase {
        dir: &'static str,
        expected: &'static str,
    }

    const CASE_1: TestCase = TestCase {
        dir: "extra_elements",
        expected: r#"
            found expected file "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}extra_elements{SEP}expected{SEP}jupiter.txt" with no counterpart in actual
            found actual file "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}extra_elements{SEP}actual{SEP}saturn.txt" with no counterpart in expected
        "#,
    };

    const CASE_2: TestCase = TestCase {
        dir: "different_kinds",
        expected: r#"
            comparing directories and files will not work with "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}different_kinds{SEP}actual" and "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}different_kinds{SEP}expected"
        "#,
    };

    const CASE_3: TestCase = TestCase {
        dir: "different_contents",
        expected: r#"
            files "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}different_contents{SEP}actual{SEP}sun.txt" and "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}different_contents{SEP}expected{SEP}sun.txt" differ on line 1
        "#,
    };

    const CASE_4: TestCase = TestCase {
        dir: "actual_not_found",
        expected: r#"
            path "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}actual_not_found{SEP}actual" not found
        "#,
    };

    const CASE_5: TestCase = TestCase {
        dir: "expected_not_found",
        expected: r#"
            path "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}expected_not_found{SEP}expected" not found
        "#,
    };

    const CASE_6: TestCase = TestCase {
        dir: "extra_expected",
        expected: r#"
            found expected file "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}extra_expected{SEP}expected{SEP}jupiter.txt" with no counterpart in actual
        "#,
    };

    const CASE_7: TestCase = TestCase {
        dir: "extra_actual",
        expected: r#"
            found actual file "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}extra_actual{SEP}actual{SEP}saturn.txt" with no counterpart in expected
        "#,
    };

    const CASE_8: TestCase = TestCase {
        dir: "different_kinds_deep",
        expected: r#"
            comparing directories and files will not work with "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}different_kinds_deep{SEP}actual{SEP}stars" and "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}different_kinds_deep{SEP}expected{SEP}stars"
        "#,
    };

    const CASE_9: TestCase = TestCase {
        dir: "deep",
        expected: r#"
            found expected file "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}deep{SEP}expected{SEP}planets{SEP}jupiter.txt" with no counterpart in actual
            found actual file "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}deep{SEP}actual{SEP}planets{SEP}saturn.txt" with no counterpart in expected
            files "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}deep{SEP}actual{SEP}sun.txt" and "tests{SEP}data{SEP}when_dir_contents_do_not_match{SEP}deep{SEP}expected{SEP}sun.txt" differ on line 0
        "#,
    };

    #[test_case(CASE_1 ; "when both sides have one extra element")]
    #[test_case(CASE_2 ; "when kinds do not match")]
    #[test_case(CASE_3 ; "when files have different contents")]
    #[test_case(CASE_4 ; "when actual path is missing")]
    #[test_case(CASE_5 ; "when expected path is missing")]
    #[test_case(CASE_8 ; "when compared items inside directory have different kind")]
    #[test_case(CASE_9 ; "when errors come from many levels")]
    fn is_err(case: TestCase) {
        run(case)
    }

    #[test]
    fn when_expected_contains_extra_elements() -> Result<(), std::io::Error> {
        let path = test_root().join(CASE_6.dir).join("actual");
        std::fs::create_dir_all(&path)?;
        run(CASE_6);
        std::fs::remove_dir_all(&path)
    }

    #[test]
    fn when_actual_contains_extra_elements() -> Result<(), std::io::Error> {
        let path = test_root().join(CASE_7.dir).join("expected");
        std::fs::create_dir_all(&path)?;
        run(CASE_7);
        std::fs::remove_dir_all(&path)
    }

    #[test]
    #[should_panic]
    fn panics() {
        assert_paths!(
            test_root().join("extra_elements").join("actual"),
            test_root().join("extra_elements").join("expected")
        );
    }
}
