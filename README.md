# json_scanner
Utility to scan json files contents

## Usage
```bash
    json_scanner [OPTIONS] <input> <json>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --extension <extension>    extension filter

ARGS:
    <input>    input folder
    <json>     json snippet
```

## Example
Given file example.json
```json
[
  {
    "a":"b",
    "c":{
      "d":"e",
      "f":"g"
    }
  },
  {
    "a":"h",
    "c":{
      "d":"i",
      "f":"l"
    }
  }
]
```

Example usage
```bash
json_scanner -e json . '{"a":"b","c":{"f":"g"}}'`
./example.json {"a":"b","c":{"d":"e","f":"g"}}
```
