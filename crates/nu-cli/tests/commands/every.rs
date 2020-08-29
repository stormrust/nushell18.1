use nu_test_support::fs::Stub::EmptyFile;
use nu_test_support::playground::Playground;
use nu_test_support::{nu, pipeline};

#[test]
fn gets_all_rows_by_every_zero() {
    Playground::setup("every_test_1", |dirs, sandbox| {
        sandbox.with_files(vec![
            EmptyFile("los.txt"),
            EmptyFile("tres.txt"),
            EmptyFile("amigos.txt"),
            EmptyFile("arepas.clu"),
        ]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                ls
                | get name
                | every 0
            "#
        ));

        let expected = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                echo [ amigos.txt arepas.clu los.txt tres.txt ]
            "#
        ));

        assert_eq!(actual.out, expected.out);
    })
}

#[test]
fn gets_no_rows_by_every_skip_zero() {
    Playground::setup("every_test_2", |dirs, sandbox| {
        sandbox.with_files(vec![
            EmptyFile("los.txt"),
            EmptyFile("tres.txt"),
            EmptyFile("amigos.txt"),
            EmptyFile("arepas.clu"),
        ]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                ls
                | get name
                | every 0 --skip
            "#
        ));

        let expected = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                echo [ ]
            "#
        ));

        assert_eq!(actual.out, expected.out);
    })
}

#[test]
fn gets_all_rows_by_every_one() {
    Playground::setup("every_test_3", |dirs, sandbox| {
        sandbox.with_files(vec![
            EmptyFile("los.txt"),
            EmptyFile("tres.txt"),
            EmptyFile("amigos.txt"),
            EmptyFile("arepas.clu"),
        ]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                ls
                | get name
                | every 1
            "#
        ));

        let expected = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                echo [ amigos.txt arepas.clu los.txt tres.txt ]
            "#
        ));

        assert_eq!(actual.out, expected.out);
    })
}

#[test]
fn gets_no_rows_by_every_skip_one() {
    Playground::setup("every_test_4", |dirs, sandbox| {
        sandbox.with_files(vec![
            EmptyFile("los.txt"),
            EmptyFile("tres.txt"),
            EmptyFile("amigos.txt"),
            EmptyFile("arepas.clu"),
        ]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                ls
                | get name
                | every 1 --skip
            "#
        ));

        let expected = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                echo [ ]
            "#
        ));

        assert_eq!(actual.out, expected.out);
    })
}

#[test]
fn gets_first_row_by_every_too_much() {
    Playground::setup("every_test_5", |dirs, sandbox| {
        sandbox.with_files(vec![
            EmptyFile("los.txt"),
            EmptyFile("tres.txt"),
            EmptyFile("amigos.txt"),
            EmptyFile("arepas.clu"),
        ]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                ls
                | get name
                | every 999
            "#
        ));

        let expected = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                echo [ amigos.txt ]
            "#
        ));

        assert_eq!(actual.out, expected.out);
    })
}

#[test]
fn gets_all_rows_except_first_by_every_skip_too_much() {
    Playground::setup("every_test_6", |dirs, sandbox| {
        sandbox.with_files(vec![
            EmptyFile("los.txt"),
            EmptyFile("tres.txt"),
            EmptyFile("amigos.txt"),
            EmptyFile("arepas.clu"),
        ]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                ls
                | get name
                | every 999 --skip
            "#
        ));

        let expected = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                echo [ arepas.clu los.txt tres.txt ]
            "#
        ));

        assert_eq!(actual.out, expected.out);
    })
}

#[test]
fn gets_every_third_row() {
    Playground::setup("every_test_7", |dirs, sandbox| {
        sandbox.with_files(vec![
            EmptyFile("los.txt"),
            EmptyFile("tres.txt"),
            EmptyFile("quatro.txt"),
            EmptyFile("amigos.txt"),
            EmptyFile("arepas.clu"),
        ]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                ls
                | get name
                | every 3
            "#
        ));

        let expected = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                echo [ amigos.txt quatro.txt ]
            "#
        ));

        assert_eq!(actual.out, expected.out);
    })
}

#[test]
fn skips_every_third_row() {
    Playground::setup("every_test_8", |dirs, sandbox| {
        sandbox.with_files(vec![
            EmptyFile("los.txt"),
            EmptyFile("tres.txt"),
            EmptyFile("quatro.txt"),
            EmptyFile("amigos.txt"),
            EmptyFile("arepas.clu"),
        ]);

        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                ls
                | get name
                | every 3 --skip
            "#
        ));

        let expected = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                echo [ arepas.clu los.txt tres.txt ]
            "#
        ));

        assert_eq!(actual.out, expected.out);
    })
}
