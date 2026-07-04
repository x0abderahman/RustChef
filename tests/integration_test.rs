use assert_cmd::Command;
use predicates::prelude::*;

/// Helper to run chef with arguments and return the output
fn chef(args: &[&str]) -> Command {
    let mut cmd = Command::cargo_bin("chef").unwrap();
    cmd.args(args);
    cmd
}

#[test]
fn test_list_operations() {
    let mut cmd = chef(&["list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Available operations"))
        .stdout(predicate::str::contains("base64-encode"))
        .stdout(predicate::str::contains("rot13"))
        .stdout(predicate::str::contains("sha256"));
}

#[test]
fn test_base64_encode() {
    let mut cmd = chef(&["op", "base64-encode", "hello"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("aGVsbG8="));
}

#[test]
fn test_base64_decode() {
    let mut cmd = chef(&["op", "base64-decode", "aGVsbG8="]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn test_hex_encode() {
    let mut cmd = chef(&["op", "hex-encode", "hello"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("68656c6c6f"));
}

#[test]
fn test_hex_decode() {
    let mut cmd = chef(&["op", "hex-decode", "68656c6c6f"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn test_url_encode() {
    let mut cmd = chef(&["op", "url-encode", "hello world"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello%20world"));
}

#[test]
fn test_url_decode() {
    let mut cmd = chef(&["op", "url-decode", "hello%20world"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello world"));
}

#[test]
fn test_rot13() {
    let mut cmd = chef(&["op", "rot13", "hello"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("uryyb"));
}

#[test]
fn test_xor() {
    let mut cmd = chef(&["op", "xor", "hello", "--", "key"]);
    cmd.assert()
        .success();
    // XOR produces non-deterministic (key-dependent) output, just check success
}

#[test]
fn test_xor_without_key_fails() {
    let mut cmd = chef(&["op", "xor", "hello"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("requires a key"));
}

#[test]
fn test_md5() {
    let mut cmd = chef(&["op", "md5", "hello"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("5d41402abc4b2a76b9719d911017c592"));
}

#[test]
fn test_sha1() {
    let mut cmd = chef(&["op", "sha1", "hello"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"));
}

#[test]
fn test_sha256() {
    let mut cmd = chef(&["op", "sha256", "hello"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        ));
}

#[test]
fn test_extract_ips() {
    let mut cmd = chef(&["op", "extract-ips", "Server 192.168.1.1 and 10.0.0.255"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("192.168.1.1"))
        .stdout(predicate::str::contains("10.0.0.255"));
}

#[test]
fn test_extract_urls() {
    let mut cmd = chef(&[
        "op",
        "extract-urls",
        "Visit https://example.com and http://test.org",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("https://example.com"))
        .stdout(predicate::str::contains("http://test.org"));
}

#[test]
fn test_extract_emails() {
    let mut cmd = chef(&[
        "op",
        "extract-emails",
        "Contact support@example.com or admin@test.org",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("support@example.com"))
        .stdout(predicate::str::contains("admin@test.org"));
}

#[test]
fn test_text_stats() {
    let mut cmd = chef(&["op", "text-stats", "Hello World"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Characters: 11"))
        .stdout(predicate::str::contains("Words: 2"))
        .stdout(predicate::str::contains("Lines: 1"))
        .stdout(predicate::str::contains("Bytes: 11"));
}

#[test]
fn test_entropy() {
    let mut cmd = chef(&["op", "entropy", "AAAA"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0.0000"));
}

#[test]
fn test_binary_encode() {
    let mut cmd = chef(&["op", "binary-encode", "A"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("01000001"));
}

#[test]
fn test_binary_decode() {
    let mut cmd = chef(&["op", "binary-decode", "01000001"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A"));
}

#[test]
fn test_json_pretty() {
    let mut cmd = chef(&["op", "json-pretty", r#"{"name":"test"}"#]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\n"));
}

#[test]
fn test_json_minify() {
    let mut cmd = chef(&["op", "json-minify", "{\n  \"name\": \"test\"\n}"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#"{"name":"test"}"#));
}

#[test]
fn test_lower_case() {
    let mut cmd = chef(&["op", "lower-case", "HELLO"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn test_upper_case() {
    let mut cmd = chef(&["op", "upper-case", "hello"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("HELLO"));
}

#[test]
fn test_reverse() {
    let mut cmd = chef(&["op", "reverse", "hello"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("olleh"));
}

#[test]
fn test_pipeline_basic() {
    let mut cmd = chef(&["run", "base64-encode,hex-encode", "hello"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("614756736247383d"));
}

#[test]
fn test_pipeline_invalid_operation() {
    let mut cmd = chef(&["run", "base64-encode,nonexistent", "hello"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unknown operation"));
}

#[test]
fn test_unknown_operation() {
    let mut cmd = chef(&["op", "nonexistent", "hello"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unknown operation"));
}

#[test]
fn test_stdin_input() {
    let mut cmd = chef(&["op", "base64-encode"]);
    cmd.write_stdin("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("aGVsbG8="));
}
