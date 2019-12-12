use crate::Error;
use std::fs::FileType;
use std::{
    cmp::Ordering,
    fs::{DirEntry, File},
    hash::Hasher,
    io::Read,
    path::Path,
};

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
                } else {
                    errors.push(Error::new_invalid_comparison(a.path(), e.path()))
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
    let e_hash = file_hash(&expected);
    let a_hash = file_hash(&actual);

    if e_hash != a_hash {
        Err(Error::new_file_contents_mismatch(
            expected.as_ref(),
            actual.as_ref(),
        ))
    } else {
        Ok(())
    }
}

fn file_hash<P: AsRef<Path>>(path: P) -> u64 {
    use twox_hash::XxHash;

    let mut hasher = XxHash::default();

    let mut file = File::open(path).expect("expected open");
    let mut buf = [0u8; 1024];
    let mut len: usize;

    while {
        len = file.read(&mut buf).expect("read");
        len != 0
    } {
        hasher.write(&buf[..len]);
    }

    hasher.finish()
}
