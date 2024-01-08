Example:

```
mod json_value;
use json_value::JsonValue;

fn main() {
  let mut data_str = r#"{"name": "Billy", "age":42, "hobbies":["Golfing", "Swimming", "Hiking"]}"#;
  let mut value = JsonValue::new();
  value.parse(data_str);
  value["employed"].set_bool(true);
  for item in value["hobbies"].get_array().iter()  {
    println!("{}", item.as_string());
  }
  value["hobbies"][2].set_decimal(12.3);
  println!("{}", value.to_formatted_string());
}
```
Will output:
```
Golfing
Swimming
Hiking
{
  "name": "Billy",
  "age": 42,
  "employed": true,
  "hobbies":
  [
    "Golfing",
    "Swimming",
    12.3
  ]
}
```
