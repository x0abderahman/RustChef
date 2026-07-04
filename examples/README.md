# Example Usage

## Basic Operations

### Base64 Encode
```bash
$ chef op base64-encode "hello"
aGVsbG8=
```

### Base64 Decode
```bash
$ chef op base64-decode "aGVsbG8="
hello
```

### Hex Encode
```bash
$ chef op hex-encode "hello"
68656c6c6f
```

### URL Encode
```bash
$ chef op url-encode "hello world"
hello%20world
```

### URL Decode
```bash
$ chef op url-decode "hello%20world"
hello world
```

### ROT13
```bash
$ chef op rot13 "hello"
uryyb
```

### XOR (with key)
```bash
$ chef op xor "hello" -- mykey
# (non-printable output)
```

### MD5
```bash
$ chef op md5 "hello"
5d41402abc4b2a76b9719d911017c592
```

### SHA256
```bash
$ chef op sha256 "hello"
2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
```

## Extraction Operations

### Extract IP Addresses
```bash
$ chef op extract-ips "Server 192.168.1.1 and 10.0.0.1"
192.168.1.1
10.0.0.1
```

### Extract URLs
```bash
$ chef op extract-urls "Visit https://example.com/path or http://test.org"
https://example.com/path
http://test.org
```

### Extract Emails
```bash
$ chef op extract-emails "Contact admin@test.com or support@example.org"
admin@test.com
support@example.org
```

## Text Analysis

### Text Statistics
```bash
$ chef op text-stats "Hello World"
Characters: 11
Words: 2
Lines: 1
Bytes: 11
```

### Entropy Estimation
```bash
$ chef op entropy "Hello World"
2.8454
```

## JSON Operations

### Pretty Print JSON
```bash
$ chef op json-pretty '{"name":"test","value":123}'
{
  "name": "test",
  "value": 123
}
```

### Minify JSON
```bash
$ chef op json-minify '{"name":"test","value":123}'
{"name":"test","value":123}
```

## Case Operations

### Lower Case
```bash
$ chef op lower-case "HELLO"
hello
```

### Upper Case
```bash
$ chef op upper-case "hello"
HELLO
```

### Reverse
```bash
$ chef op reverse "hello"
olleh
```

## Binary Operations

### Binary Encode
```bash
$ chef op binary-encode "A"
01000001
```

### Binary Decode
```bash
$ chef op binary-decode "01000001"
A
```

## Pipeline (Chaining Operations)

### Base64 then Hex
```bash
$ chef run "base64-encode,hex-encode" "hello"
614756736247383d
```

### Hex then Reverse
```bash
$ chef run "hex-encode,reverse" "hello"
f6c6c65686
```

### URL Decode then ROT13
```bash
$ chef run "url-decode,rot13" "%66%75%6e%6b%79"
shaxl
```

## Reading from stdin

```bash
$ echo "hello" | chef op base64-encode
aGVsbG8=
```

## Using sample files

```bash
$ chef op extract-emails < samples/input.txt
support@example.com
admin@test.org
```
