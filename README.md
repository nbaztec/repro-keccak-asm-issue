# Summary

When compiling this application with `--features asm-keccak`, the following test fails under Ubuntu and Arch, but passes on Mac:

```rust
fn test_foo_fails() {
    let a = alloy_primitives::keccak256("testFoo()");   // want: 0x79adbd5094e60c1bc2b963678ff44695d1430b8ccff0b1cd57c03a7f63567822
                                                        // got : 0xae33576402a61ff3a4a241b4a91d20ce98b70aa7bf0f388fb857111ec83aef73
    let b = alloy_primitives::keccak256("test_Foo()");
    println!("testFoo() : {:?}", a);
    println!("test_Foo(): {:?}", b);
    assert_eq!(format!("{a:?}"), String::from("0x79adbd5094e60c1bc2b963678ff44695d1430b8ccff0b1cd57c03a7f63567822"));
    assert_eq!(format!("{b:?}"), String::from("0x45c48c2bd4afc6adc7884fe296b9af10e234ddbc44f2f99f40cfb8b6391e9798"));   // this is always correct
}
```

```bash
testFoo() : 0xae33576402a61ff3a4a241b4a91d20ce98b70aa7bf0f388fb857111ec83aef73
test_Foo(): 0x45c48c2bd4afc6adc7884fe296b9af10e234ddbc44f2f99f40cfb8b6391e9798
thread 'tests::test_foo_fails' panicked at src/main.rs:19:9:
assertion `left == right` failed
  left: "0xae33576402a61ff3a4a241b4a91d20ce98b70aa7bf0f388fb857111ec83aef73"
 right: "0x79adbd5094e60c1bc2b963678ff44695d1430b8ccff0b1cd57c03a7f63567822"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::test_foo_fails

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The issue seems to be that the keccak256 hash is incorrectly calculated on linux.

The example has been minimized from a foundry-zksync repository, where these collection of dependencies and their respective imports cause this issue to occur. 
If any one of the deps or their imports is removed, then the issue goes away. 

