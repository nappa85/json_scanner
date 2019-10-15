# json_scanner
Utility to scan json files contents

## Why?
Sometimes you need to search inside a big json file, or maybe an entire folder full of big json files.<br />
What you're searching for isn't a simple string occurrence, the json file has a complex and variable structure and `grep` can't help here, more the file is onelineer, therefore a `grep` result would be unusable, because you need only the matching json object, not the entire file contents.<br />
It was in a situation like this that I wrote this program, that permits to find submatches inside json files.

### How?
The program is quite simple, written in Rust using Async/Await to speed it up.<br />
At the moment, it only matches on the root of the json file, if the root is an array, it scans the array and matches on the elements.<br />
It's able to do json objects submatches, that means if on the document you have something like `{"a":"b","c":12}` and you search for `{"c":12}`, it will match.

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
