use crate::{Error, TestResult};
use std::{
    cmp::Ordering,
    fs::{DirEntry, File},
    hash::Hasher,
    io::Read,
    path::Path,
};

pub fn assert_dirs<PE: AsRef<Path>, PA: AsRef<Path>>(expected: PE, actual: PA) -> TestResult {
    let mut errors = Vec::new();

    let expected = expected
        .as_ref()
        .canonicalize()
        .map_err(|e| format!("cannot canonicalize expected path, {}", e))?;
    let actual = actual
        .as_ref()
        .canonicalize()
        .map_err(|e| format!("cannot canonicalize actual path, {}", e))?;

    if expected.is_file() && actual.is_file() {
        compare_file(expected, actual, &mut errors);
    } else if expected.is_dir() && actual.is_dir() {
        compare_dir_recursive(expected, actual, &mut errors);
    } else {
        Err(Error::ContentMismatch(format!(
            "comparing {:?} to {:?}",
            expected, actual
        )))?
    }

    TestResult {
        result: !errors.is_empty(),
        errors,
    }
}

fn compare_dir_recursive<PE: AsRef<Path>, PA: AsRef<Path>>(
    expected: PE,
    actual: PA,
    result: &mut Vec<Error>,
) {
    let expected = std::fs::read_dir(expected)
        .expect("read_dir expected")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect expected");
    let actual = std::fs::read_dir(actual)
        .expect("read_dir actual")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect actual");

    let (exp_dirs, exp_files) = expected
        .into_iter()
        .partition(|entry| entry.metadata().expect("exp entry metadata").is_dir());
    let (act_dirs, act_files) = actual
        .into_iter()
        .partition(|entry| entry.metadata().expect("act entry metadata").is_dir());

    compare_files(exp_files, act_files, result);
    compare_directories(exp_dirs, act_dirs, result);
}

fn compare_files(
    mut expected_files: Vec<DirEntry>,
    mut actual_files: Vec<DirEntry>,
    result: &mut Vec<Error>,
) {
    expected_files.sort_by(|l, r| l.path().cmp(&r.path()));
    actual_files.sort_by(|l, r| l.path().cmp(&r.path()));

    let mut exp_index = 0usize;
    let mut act_index = 0usize;

    loop {
        match (expected_files.get(exp_index), actual_files.get(act_index)) {
            (Some(exp_entry), Some(act_entry)) => {
                match exp_entry.file_name().cmp(&act_entry.file_name()) {
                    Ordering::Less => {
                        result.push(Error::ExtraExpected(
                            exp_entry.path().to_string_lossy().to_string(),
                        ));
                        exp_index += 1;
                    }
                    Ordering::Equal => {
                        compare_file(exp_entry.path(), act_entry.path(), result);
                        exp_index += 1;
                        act_index += 1;
                    }
                    Ordering::Greater => {
                        result.push(Error::ExtraActual(
                            exp_entry.path().to_string_lossy().to_string(),
                        ));
                        act_index += 1;
                    }
                }
            }
            (Some(exp_entry), _) => {
                result.push(Error::ExtraExpected(
                    exp_entry.path().to_string_lossy().to_string(),
                ));
                exp_index += 1;
            }
            (_, Some(act_entry)) => {
                result.push(Error::ExtraActual(
                    act_entry.path().to_string_lossy().to_string(),
                ));
                act_index += 1;
            }
            (None, None) => break,
        }
    }
}

fn compare_directories(
    mut expected_directory: Vec<DirEntry>,
    mut actual_directory: Vec<DirEntry>,
    result: &mut Vec<Error>,
) {
    expected_directory.sort_by(|l, r| l.path().cmp(&r.path()));
    actual_directory.sort_by(|l, r| l.path().cmp(&r.path()));

    let mut exp_index = 0usize;
    let mut act_index = 0usize;

    loop {
        match (
            expected_directory.get(exp_index),
            actual_directory.get(act_index),
        ) {
            (Some(exp_entry), Some(act_entry)) => {
                match exp_entry.file_name().cmp(&act_entry.file_name()) {
                    Ordering::Less => {
                        result.push(Error::ExtraExpected(
                            exp_entry.path().to_string_lossy().to_string(),
                        ));
                        exp_index += 1;
                    }
                    Ordering::Equal => {
                        compare_dir_recursive(exp_entry.path(), act_entry.path(), result);
                        exp_index += 1;
                        act_index += 1;
                    }
                    Ordering::Greater => {
                        result.push(Error::ExtraActual(
                            exp_entry.path().to_string_lossy().to_string(),
                        ));
                        act_index += 1;
                    }
                }
            }
            (Some(exp_entry), _) => {
                result.push(Error::ExtraExpected(
                    exp_entry.path().to_string_lossy().to_string(),
                ));
                exp_index += 1;
            }
            (_, Some(act_entry)) => {
                result.push(Error::ExtraActual(
                    act_entry.path().to_string_lossy().to_string(),
                ));
                act_index += 1;
            }
            (None, None) => break,
        }
    }
}

fn compare_file<PE: AsRef<Path>, PA: AsRef<Path>>(
    expected: PE,
    actual: PA,
    result: &mut Vec<Error>,
) {
    let e_hash = file_hash(expected.as_ref());
    let a_hash = file_hash(actual);

    if e_hash != a_hash {
        result.push(Error::ContentMismatch(
            expected.as_ref().to_string_lossy().to_string(),
        ))
    }
}

fn file_hash<P: AsRef<Path>>(path: P) -> u64 {
    use twox_hash::XxHash;

    let mut hasher = XxHash::default();

    let mut file = File::open(path).expect("expected open");
    let mut buf = [0u8; 1024];

    loop {
        let len = file.read(&mut buf).expect("read");
        if len == 0 {
            break;
        }
        hasher.write(&buf[..len]);
    }

    hasher.finish()
}
