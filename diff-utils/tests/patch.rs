use anyhow::Result;
use diff_utils::{Comparison, PatchOptions};
use chrono::{DateTime, Local};
use std::borrow::Cow;
use std::io::Write;
use itertools::Itertools;

#[test]
fn test() -> Result<()> {
    let mut failed = false;
    for entry in glob::glob("tests/**/*.actual")? {
        let actual_path = entry?;
        let mut expected_path = actual_path.clone();
        expected_path.set_extension("expected");

        let mut new_path = actual_path.clone();
        new_path.set_extension("new.tmp");

        let mut patch_path = actual_path.clone();
        patch_path.set_extension("patch.tmp");

        dbg!(&actual_path);
        dbg!(&expected_path);
        let expected = std::fs::read_to_string(&expected_path)?;
        let actual = std::fs::read_to_string(&actual_path)?;
        let expected_lines = expected.lines().collect::<Vec<_>>();
        let actual_lines = actual.lines().collect::<Vec<_>>();
        let comparison = Comparison::new(&expected_lines, &actual_lines).compare()?;

        let dt = "2020-06-27 18:10:03 +0200";
        let datetime: DateTime<Local> = dt.parse()?;
        let dt = datetime.format("%F %T %z");

        let entry_basename = Cow::Borrowed("test.ion.snap");
        let snap_basename = Cow::Borrowed("test.ion.ast.new");

        let new = comparison.patch(
                entry_basename,
                &dt,
                snap_basename,
                &dt,
                PatchOptions::default() // 49 in neulang
            );

        let new = new.to_string()
            .lines()
            .skip(2)
            .join("\n");

        std::fs::File::create(&new_path)
            .and_then(|mut file| {
                write!(file, "{}", &new)
            })?;

        use std::process::Command;
        let expected_path = expected_path.display().to_string();
        let actual_path = actual_path.display().to_string();
        let diff_cmd = Command::new("diff")
            .args(&[
                "-u",
                expected_path.as_str(),
                actual_path.as_str(),
            ])
            .output()?;

        let patch = diff_cmd.stdout.as_slice();
        let patch = String::from_utf8_lossy(patch)
            .to_string()
            .lines()
            .skip(2)
            .join("\n");

        std::fs::File::create(&patch_path)
            .and_then(|mut file| {
                write!(file, "{}", &patch)
            })?;

        if patch != new {
            failed = true;
        }
        else {
            std::fs::remove_file(&patch_path)?;
            std::fs::remove_file(&new_path)?;
        }
    }
    if failed {
        panic!("Found difference between .new and .patch");
    }

    Ok(())
}