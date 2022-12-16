#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use crate::delete_ts_ignore_error;

    #[test]
    fn test_delete_ts_ignore_error() {
        // Create a test directory
        let dir = "test_dir";
        fs::create_dir(dir).unwrap();

        // Create a test file with the .ts extension
        let ts_file = "test_dir/test-file.ts";
        let ts_contents = "function hello() {\n  // @ts-ignore\n  return {\n    a: 1,\n  };\n}\n// @ts-expect-error\nexport default hello;";
        fs::write(ts_file, ts_contents).unwrap();

        // Create a test file with the .tsx extension
        let tsx_file = "test_dir/test-file.tsx";
        let tsx_contents = "function hello() {\n  // @ts-ignore\n  return {\n    a: 1,\n  };\n}\n// @ts-expect-error\nexport default hello;";
        fs::write(tsx_file, tsx_contents).unwrap();

        // Call the delete_ts_ignore_error function with the test directory as the input
        let count = delete_ts_ignore_error(Path::new(dir));

        // Assert that the correct number of lines were deleted
        assert_eq!(count, 4);

        // Clean up the test files
        fs::remove_file(ts_file).unwrap();
        fs::remove_file(tsx_file).unwrap();

        // Clean up the test directory
        fs::remove_dir(dir).unwrap();
    }
}
