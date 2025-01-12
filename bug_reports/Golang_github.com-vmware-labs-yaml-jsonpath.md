Results do not match other implementations

The following queries provide results that do not match those of other implementations of JSONPath
(compare https://cburgmer.github.io/json-path-comparison/):

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
  [
    {
      "": 21,
      "a": 42
    }
  ]
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
  Exception: invalid type of value passed to compareNodeValues
  ```

- [ ] `$[*,1]`
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
  NOT_SUPPORTED
  ```
  Actual output:
  ```
  [
    "first",
    "second",
    "third",
    "forth",
    "fifth",
    "second"
  ]
  ```


For reference, the output was generated by the program in https://github.com/cburgmer/json-path-comparison/tree/master/implementations/Golang_github.com-vmware-labs-yaml-jsonpath.
