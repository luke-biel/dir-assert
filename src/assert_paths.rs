use crate::Error;
use std::fs::FileType;
use std::io::{BufRead, BufReader};
use std::{
    cmp::Ordering,
    fs::{DirEntry, File},
    path::Path,
};

/// Recursively scan contents of two directories and find differences.
///
/// eg.:
/// ```rust,ignore
/// #[test]
/// fn should_directories_be_equal() {
///     let result = assert_paths("actual", "expected");
///     assert!(result.is_ok());
/// }
/// ```
///
/// This function is called inside `assert_paths` macro invocation.
/// It can be used to verify types of errors returned when types differ.
pub fn assert_paths<PE: AsRef<Path>, PA: AsRef<Path>>(
    actual: PA,
    expected: PE,
) -> Result<(), Vec<Error>> {
    let expected = expected.as_ref();
    let actual = actual.as_ref();

    if !expected.exists() {
        return Err(vec![Error::new_missing_path(expected)]);
    }

    if !actual.exists() {
        return Err(vec![Error::new_missing_path(actual)]);
    }

    if expected.is_file() && actual.is_file() {
        compare_file(expected, actual).map_err(|err| vec![err])
    } else if expected.is_dir() && actual.is_dir() {
        compare_dir_recursive(expected, actual)
    } else {
        Err(vec![Error::new_invalid_comparison(expected, actual)])
    }
}

fn compare_dir_recursive<PE: AsRef<Path>, PA: AsRef<Path>>(
    expected: PE,
    actual: PA,
) -> Result<(), Vec<Error>> {
    let mut expected = dir_contents_sorted(&expected)
        .map_err(|err| vec![err])?
        .into_iter();
    let mut actual = dir_contents_sorted(&actual)
        .map_err(|err| vec![err])?
        .into_iter();

    let mut errors = Vec::new();

    let mut expected_entry = expected.next();
    let mut actual_entry = actual.next();

    loop {
        let (e, a) = match (&expected_entry, &actual_entry) {
            (None, None) => break,
            (Some(e), Some(a)) => (e, a),
            (Some(e), None) => {
                errors.push(Error::new_extra_expected(e.path()));
                expected_entry = expected.next();
                continue;
            }
            (None, Some(a)) => {
                errors.push(Error::new_extra_actual(a.path()));
                actual_entry = actual.next();
                continue;
            }
        };

        match e.path().file_name().cmp(&a.path().file_name()) {
            Ordering::Less => {
                errors.push(Error::new_extra_expected(e.path()));
                expected_entry = expected.next();
                continue;
            }
            Ordering::Equal => {
                let e_ft = get_file_type(e).map_err(|err| vec![err])?;
                let a_ft = get_file_type(a).map_err(|err| vec![err])?;

                if e_ft.is_file() && a_ft.is_file() {
                    if let Err(err) = compare_file(e.path(), a.path()) {
                        errors.push(err);
                    }
                } else if e_ft.is_dir() && a_ft.is_dir() {
                    if let Err(err) = compare_dir_recursive(e.path(), a.path()) {
                        errors.extend_from_slice(&err);
                    }
                } else if e_ft.is_symlink() && a_ft.is_symlink() {
                    // do nothing
                } else {
                    errors.push(Error::new_invalid_comparison(e.path(), a.path()))
                }
            }
            Ordering::Greater => {
                errors.push(Error::new_extra_actual(a.path()));
                actual_entry = actual.next();
                continue;
            }
        }

        expected_entry = expected.next();
        actual_entry = actual.next();
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn get_file_type(path: &DirEntry) -> Result<FileType, Error> {
    path.file_type().map_err(|err| {
        Error::new_critical(format!(
            "unable to retrieve file type from {:?}, {}",
            path, err
        ))
    })
}

fn dir_contents_sorted<P: AsRef<Path>>(dir: &P) -> Result<Vec<DirEntry>, Error> {
    let mut dir_contents = std::fs::read_dir(&dir)
        .map_err(|err| {
            Error::new_critical(format!("failed reading dir {:?}, {}", dir.as_ref(), err))
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| {
            Error::new_critical(format!(
                "an IO error occurred when reading dir, {:?}, {}",
                dir.as_ref(),
                err
            ))
        })?;

    dir_contents.sort_by(|left, right| left.file_name().cmp(&right.file_name()));

    Ok(dir_contents)
}

fn compare_file<PE: AsRef<Path>, PA: AsRef<Path>>(expected: PE, actual: PA) -> Result<(), Error> {
    let expected = expected.as_ref();
    let actual = actual.as_ref();

    let file_e = File::open(expected).map_err(|e| {
        Error::new_critical(format!(
            "cannot open expected file {:?}, reason: {}",
            expected, e
        ))
    })?;
    let file_a = File::open(actual).map_err(|e| {
        Error::new_critical(format!(
            "cannot open actual file {:?}, reason: {}",
            actual, e
        ))
    })?;

    let reader_e = BufReader::new(file_e);
    let reader_a = BufReader::new(file_a);

    for (idx, lines) in reader_e.lines().zip(reader_a.lines()).enumerate() {
        let (line_e, line_a) = match lines {
            (Ok(line_e), Ok(line_a)) => (line_e, line_a),
            (Err(err), _) => {
                return Err(Error::new_critical(format!(
                    "failed reading line from {:?}, reason: {}",
                    expected, err
                )))
            }
            (_, Err(err)) => {
                return Err(Error::new_critical(format!(
                    "failed reading line from {:?}, reason: {}",
                    actual, err
                )))
            }
        };

        if line_e != line_a {
            return Err(Error::new_file_contents_mismatch(expected, actual, idx));
        }
    }

    Ok(())
}
