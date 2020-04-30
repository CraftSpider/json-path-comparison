Results do not match other implementations

The following queries provide results that do not match those of other implementations of JSONPath
(compare https://cburgmer.github.io/json-path-comparison/):

- [ ] `$[1:3]`
  Input:
  ```
  {":": 42, "more": "string", "a": 1, "b": 2, "c": 3}
  ```
  Expected output:
  ```
  []
  ```
  Error:
  ```
  Filter: [1:3] can only be applied to arrays. Current context is: {
      ":" = 42;
      a = 1;
      b = 2;
      c = 3;
      more = string;
  }
  ```

- [ ] `$[:]`
  Input:
  ```
  ["first", "second"]
  ```
  Expected output:
  ```
  ["first", "second"]
  ```
  Error:
  ```
  Failed to parse SliceOperation: :
  ```

- [ ] `$[::]`
  Input:
  ```
  ["first", "second"]
  ```
  Expected output:
  ```
  ["first", "second"]
  ```
  Error:
  ```
  Failed to parse SliceOperation: ::
  ```

- [ ] `$[0:3:2]`
  Input:
  ```
  ["first", "second", "third", "forth", "fifth"]
  ```
  Expected output:
  ```
  ["first", "third"]
  ```
  Actual output:
  ```
  ["first", "second", "third"]
  ```

- [ ] `$[0:4:2]`
  Input:
  ```
  ["first", "second", "third", "forth", "fifth"]
  ```
  Expected output:
  ```
  ["first", "third"]
  ```
  Actual output:
  ```
  ["first", "second", "third", "forth"]
  ```

- [ ] `$[::2]`
  Input:
  ```
  ["first", "second", "third", "forth", "fifth"]
  ```
  Expected output:
  ```
  ["first", "third", "fifth"]
  ```
  Error:
  ```
  Failed to parse SliceOperation: ::2
  ```

- [ ] `$[',']`
  Input:
  ```
  {",": "value", "another": "entry"}
  ```
  Expected output:
  ```
  "value"
  ```
  Error:
  ```
  Found empty property at index 5
  ```

- [ ] `$.key`
  Input:
  ```
  {"key": null}
  ```
  Expected output:
  ```
  null
  ```
  Error:
  ```
  2020-04-29 23:43:29.858 main[65320:2496627] *** Terminating app due to uncaught exception 'NSInvalidArgumentException', reason: '*** +[NSJSONSerialization dataWithJSONObject:options:error:]: Invalid top-level type in JSON write'
  *** First throw call stack:
  (
  	0   CoreFoundation                      0x00007fff2c3b68ab __exceptionPreprocess + 250
  	1   libobjc.A.dylib                     0x00007fff624d7805 objc_exception_throw + 48
  	2   Foundation                          0x00007fff2ea4e4ab +[NSJSONSerialization dataWithJSONObject:options:error:] + 318
  	3   main                                0x000000010f975a3e main + 2046
  	4   libdyld.dylib                       0x00007fff638457fd start + 1
  	5   ???                                 0x0000000000000002 0x0 + 2
  )
  libc++abi.dylib: terminating with uncaught exception of type NSException
  implementations/Objective-C_SMJJSONPath/run.sh: line 6: 65320 Abort trap: 6           "$script_dir"/build/main "$@"
  ```

- [ ] `$.屬性`
  Input:
  ```
  {"\u5c6c\u6027": "value"}
  ```
  Expected output:
  ```
  "value"
  ```
  Error:
  ```
  No results for path: $['Â±¨ÊÄß']
  ```

- [ ] `$..*`
  Input:
  ```
  42
  ```
  Expected output (in any order as no consensus on ordering exists):
  ```
  []
  ```
  Error:
  ```
  The data couldn’t be read because it isn’t in the correct format.
  ```

- [ ] `$`
  Input:
  ```
  42
  ```
  Expected output:
  ```
  42
  ```
  Error:
  ```
  The data couldn’t be read because it isn’t in the correct format.
  ```

- [ ] `$`
  Input:
  ```
  false
  ```
  Expected output:
  ```
  false
  ```
  Error:
  ```
  The data couldn’t be read because it isn’t in the correct format.
  ```

- [ ] `$`
  Input:
  ```
  true
  ```
  Expected output:
  ```
  true
  ```
  Error:
  ```
  The data couldn’t be read because it isn’t in the correct format.
  ```

- [ ] `$[0]['c','d']`
  Input:
  ```
  [{"c": "cc1", "d": "dd1", "e": "ee1"}, {"c": "cc2", "d": "dd2", "e": "ee2"}]
  ```
  Expected output:
  ```
  ["cc1", "dd1"]
  ```
  Actual output:
  ```
  {"c": "cc1", "d": "dd1"}
  ```


For reference, the output was generated by the program in https://github.com/cburgmer/json-path-comparison/tree/master/implementations/Objective-C_SMJJSONPath.