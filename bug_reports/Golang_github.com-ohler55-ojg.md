Results do not match other implementations

The following queries provide results that do not match those of other implementations of JSONPath
(compare https://cburgmer.github.io/json-path-comparison/):

- [ ] `$.key-dash`
  Input:
  ```
  {
    "key": 42,
    "key-": 43,
    "-": 44,
    "dash": 45,
    "-dash": 46,
    "": 47,
    "key-dash": "value",
    "something": "else"
  }
  ```
  Expected output:
  ```
  ["value"]
  ```
  Actual output:
  NOT_SUPPORTED
  ```
  ```

- [ ] `$.-1`
  Input:
  ```
  [
    "first",
    "second",
    "third",
    "forth",
    "fifth"
  ]
  ```
  Expected output:
  ```
  []
  ```
  Actual output:
  NOT_SUPPORTED
  ```
  ```

- [ ] ``
  Input:
  ```
  {
    "a": 42,
    "": 21
  }
  ```
  Expected output:
  ```
  NOT_SUPPORTED
  ```
  Actual output:
  ```
  []
  ```

- [ ] `$[?(@.key1==@.key2)]`
  Input:
  ```
  [
    {
      "key1": 10,
      "key2": 10
    },
    {
      "key1": 42,
      "key2": 50
    },
    {
      "key1": 10
    },
    {
      "key2": 10
    },
    {},
    {
      "key1": null,
      "key2": null
    },
    {
      "key1": null
    },
    {
      "key2": null
    },
    {
      "key1": 0,
      "key2": 0
    },
    {
      "key1": 0
    },
    {
      "key2": 0
    },
    {
      "key1": -1,
      "key2": -1
    },
    {
      "key1": "",
      "key2": ""
    },
    {
      "key1": false,
      "key2": false
    },
    {
      "key1": false
    },
    {
      "key2": false
    },
    {
      "key1": true,
      "key2": true
    },
    {
      "key1": [],
      "key2": []
    },
    {
      "key1": {},
      "key2": {}
    },
    {
      "key1": {
        "a": 1,
        "b": 2
      },
      "key2": {
        "b": 2,
        "a": 1
      }
    }
  ]
  ```
  Error:
  ```
  Exception: runtime error: comparing uncomparable type []interface {}
  ```


For reference, the output was generated by the program in https://github.com/cburgmer/json-path-comparison/tree/master/implementations/Golang_github.com-ohler55-ojg.
