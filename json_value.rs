use std::{collections::HashMap, ops::{Index, IndexMut}};

#[derive(Default)]
pub struct JsonValue {
    var_type: VarType,
    value_stored: String,
    content: HashMap<String, JsonValue>,
    array: Vec<JsonValue>,
}
#[derive(PartialEq, Eq)]
enum VarType {
    String,
    Int,
    Decimal,
    Bool,
    Array,
    Object,
    Null,
    Undefined,
}

impl Default for VarType {
    fn default() -> Self {
        VarType::Null
    }
}
#[derive(PartialEq, Eq)]
enum ContainerEnum {
    SettingKey,
    AddingValue
}

impl JsonValue {
    pub fn new() -> Self {
        JsonValue {
            var_type: VarType::Undefined,
            value_stored: String::new(),
            content: HashMap::new(),
            array: Vec::new()
        }
    }
    // Checkers
    pub fn is_undefined(&self) -> bool {
        match self.var_type {
            VarType::Undefined => true,
            _ => false
        }
    }
    pub fn is_null(&self) -> bool {
        match self.var_type {
            VarType::Null => true,
            _ => false
        }
    }
    pub fn is_object(&self) -> bool {
        match self.var_type {
            VarType::Object => true,
            _ => false
        }
    }
    pub fn is_array(&self) -> bool {
        match self.var_type {
            VarType::Array => true,
            _ => false
        }
    }
    pub fn is_value(&self) -> bool {
        match self.var_type {
            VarType::Undefined => false,
            VarType::Array => false,
            VarType::Null => false,
            VarType::Object => false,
            _ => true
        }
    }
    pub fn is_bool(&self) -> bool {
        match self.var_type {
            VarType::Bool => true,
            _ => false
        }
    }
    pub fn is_string(&self) -> bool {
        match self.var_type {
            VarType::String => true,
            _ => false
        }
    }
    pub fn is_int(&self) -> bool {
        match self.var_type {
            VarType::Int => true,
            _ => false
        }
    }
    pub fn is_decimal(&self) -> bool {
        match self.var_type {
            VarType::Decimal => true,
            _ => false
        }
    }
    // Getters
    pub fn get_array(&self) -> &Vec<JsonValue> { &self.array }
    pub fn get_array_mut(&mut self) -> &mut Vec<JsonValue> { &mut self.array }
    pub fn get_object(&self) -> &HashMap<String, JsonValue> { &self.content }
    pub fn get_object_mut(&mut self) -> &mut HashMap<String, JsonValue> { &mut self.content }
    pub fn as_string(&self) -> &str { self.value_stored.as_str() }
    pub fn as_bool(&self) -> bool {
        match self.value_stored.as_str() {
            "true" => true,
            _ => false
        }
    }
    pub fn as_int(&self) -> i128 {
        if let Ok(x) = self.value_stored.parse::<i128>(){
            x
        } else {
            0
        }
    }
    pub fn as_decimal(&self) -> f64 {
        if let Ok(x) = self.value_stored.parse::<f64>(){
            x
        } else {
            0.0
        }
    }
    // Setters
    pub fn set_string(&mut self, value: &str) {
        self.var_type = VarType::String;
        self.value_stored = String::from(value);
    }
    pub fn set_int(&mut self, value: i128) {
        self.var_type = VarType::Int;
        self.value_stored = String::from(value.to_string());
    }
    pub fn set_decimal(&mut self, value: f64) {
        self.var_type = VarType::Decimal;
        self.value_stored = String::from(value.to_string());
    }
    pub fn set_bool(&mut self, value: bool) {
        self.var_type = VarType::Bool;
        self.value_stored = String::from(value.to_string());
    }
    // Adders
    pub fn add_json(&mut self, key: &str, value: JsonValue) {
        self.var_type = VarType::Object;
        self.content.insert(key.to_string(), value);
    }
    pub fn add_string(&mut self, key: &str, value: &str) {
        self.var_type = VarType::Object;
        let mut j_v = JsonValue::new();
        j_v.set_string(value);
        self.content.insert(key.to_string(), j_v);
    }
    pub fn add_int(&mut self, key: &str, value: i128) {
        self.var_type = VarType::Object;
        let mut j_v = JsonValue::new();
        j_v.set_int(value);
        self.content.insert(key.to_string(), j_v);
    }
    pub fn add_decimal(&mut self, key: &str, value: f64) {
        self.var_type = VarType::Object;
        let mut j_v = JsonValue::new();
        j_v.set_decimal(value);
        self.content.insert(key.to_string(), j_v);
    }
    pub fn add_bool(&mut self, key: &str, value: bool) {
        self.var_type = VarType::Object;
        let mut j_v = JsonValue::new();
        j_v.set_bool(value);
        self.content.insert(key.to_string(), j_v);
    }
    // Appenders
    pub fn append_json(&mut self, value: JsonValue) {
        self.var_type = VarType::Array;
        self.array.push(value);
    }
    pub fn append_string(&mut self, value: &str) {
        self.var_type = VarType::Array;
        let mut j_v = JsonValue::new();
        j_v.set_string(value);
        self.array.push(j_v);
    }
    pub fn append_int(&mut self, value: i128) {
        self.var_type = VarType::Array;
        let mut j_v = JsonValue::new();
        j_v.set_int(value);
        self.array.push(j_v);
    }
    pub fn append_decimal(&mut self, value: f64) {
        self.var_type = VarType::Array;
        let mut j_v = JsonValue::new();
        j_v.set_decimal(value);
        self.array.push(j_v);
    }
    pub fn append_bool(&mut self, value: bool) {
        self.var_type = VarType::Array;
        let mut j_v = JsonValue::new();
        j_v.set_bool(value);
        self.array.push(j_v);
    }
    // Serialize
    pub fn to_string(&self) -> String {
        self.serializer()
    }
    pub fn to_formatted_string(&self) -> String {
        let s = self.serializer();
        println!("{}", s);
        self.add_formatting(s.chars().collect())
    }
    fn serializer(&self) -> String {
        let mut result = String::new();

        match self.var_type {
            VarType::Array => {
                result.push_str("[");

                for item in self.array.iter() {
                    result.push_str(item.serializer().as_str());
                    result.push_str(",");
                }
                result.remove(result.len() - 1);
                result.push_str("]");
                return result
            },
            VarType::Object => {
                result.push_str("{");
                for item in self.content.iter() {
                    if item.1.is_null() || item.1.is_undefined() {
                        continue;
                    }
                    result.push_str(("\"".to_owned() + item.0.as_str() + "\":"+item.1.serializer().as_str() + ",").as_str());
                }
                if result.len() > 1{
                    result.remove(result.len()-1);
                } 
                result.push_str("}");
                return result
            },
            _ => {
                if self.var_type == VarType::Null || self.var_type == VarType::Undefined {
                    return String::from("null")
                } 
                if self.var_type == VarType::String {
                    
                    result.push('"');
                    for item in self.value_stored.chars() {
                        match item {
                            '\n' => result.push_str("\\n"),
                            '\t' => result.push_str("\\t"),
                            '\\' => {
                                result.push('\\');
                                result.push('\\');
                            },
                            _ => result.push(item)
                        }
                    }

                    result.push('"');
                    return result
                }
                return self.value_stored.clone()
            }
        }
    } 

    pub fn parse(&mut self, data: &str) {
        let mut index: usize = 1;
        let d = self.remove_formatting(data.chars().collect());
        self.deserializer(&d.chars().collect(), &mut index, VarType::Object);
    }
    fn deserializer(&mut self, data: &Vec<char>, index: &mut usize, var_type: VarType) {
        let mut in_string: bool = false;
        match var_type {
            VarType::Object => {
                let mut state = ContainerEnum::SettingKey;

                let mut key = String::new();
                while !data[*index].eq(&'}') || in_string {
                    if state == ContainerEnum::SettingKey {
                        key = String::from("");
                        let start_index = *index;
                        while !data[*index].eq(&':') || in_string {
                            if *index != start_index {
                                key.push(data[*index]);
                            }
                            self.update_in_string(&mut in_string, data, *index);
                            *index += 1;
                        }
                        key.remove(key.len()-1);
                        state = ContainerEnum::AddingValue;
                    } else {
                        *index += 1;

                        let mut value_to_add = JsonValue::new();

                        if data[*index].eq(&'{') {
                            *index += 1;
                            value_to_add.deserializer(data, index, VarType::Object);
                        } else if data[*index].eq(&'[') {
                            *index += 1;
                            value_to_add.deserializer(data, index, VarType::Array);
                        } else {
                            value_to_add.deserializer(data, index, VarType::Null);
                        }

                        self.add_json(key.as_str(), value_to_add);
                        state = ContainerEnum::SettingKey;
                    }
                    if data[*index].eq(&',') {
                        *index += 1;
                    }
                }
                *index += 1;
            },
            VarType::Array => {
                while !data[*index].eq(&']') {
                    while !data[*index].eq(&',') && !data[*index].eq(&']') {
                        let mut value_to_add = JsonValue::new();
                        if data[*index].eq(&'{') {
                            *index += 1;
                            value_to_add.deserializer(data, index, VarType::Object);
                        } else if data[*index].eq(&'[') {
                            *index += 1;
                            value_to_add.deserializer(data, index, VarType::Array);
                        } else {
                            value_to_add.deserializer(data, index, VarType::Null);
                        }

                        self.append_json(value_to_add);
                    }
                    if data[*index].eq(&',') {
                        *index += 1;
                    }
                }
                *index += 1;
            },
            _ => {
                let mut value = String::new();
                while !data[*index].eq(&',') && !data[*index].eq(&'}') && !data[*index].eq(&']') {
                    self.update_in_string(&mut in_string, data, *index);
                    
                    if in_string {
                        if data[*index].eq(&'\\') {
                            match data[*index + 1] {
                                'n' => {
                                    *index += 2;
                                    value.push('\n');
                                    continue;
                                },
                                't' => {
                                    *index += 2;
                                    value.push('\t');
                                    continue;
                                },
                                '\\' => {
                                    *index += 2;
                                    value.push('\\');
                                    continue;
                                },
                                _ => continue,
                            }
                        }
                    }

                    value.push(data[*index]);
                    *index += 1;
                }
                
                self.value_stored = value.to_string();
                if self.value_stored.as_bytes()[0] == b'\"' {
                    self.value_stored.remove(0);
                    self.value_stored.remove(self.value_stored.len() - 1);
                    self.var_type = VarType::String;
                } else if self.value_stored.eq("null") || self.value_stored.len() == 0 {
                    self.var_type = VarType::Null;
                } else if self.value_stored.contains(".") {
                    self.var_type = VarType::Decimal;
                } else if self.value_stored.eq_ignore_ascii_case("true") || self.value_stored.eq_ignore_ascii_case("false") {
                    self.var_type = VarType::Bool;
                } else {
                    self.var_type = VarType::Int;
                }


            }
        }
    }
    fn update_in_string(&mut self, in_string: &mut bool, data: &Vec<char>, index: usize) {
        if data[index].eq(&'\"') {
            if index > 0 {
                if !data[index-1].eq(&'\\') {
                    *in_string = !*in_string;
                }
            } else {
                *in_string = !*in_string;
            }
        }
    }
    fn add_formatting(&self, data: Vec<char>) -> String {
        let mut result = String::new();
        let mut in_string = false;
        const TAB: &str = "    ";
        let mut tab_depth = 1;
        result.push(data[0]);
        result.push('\n');
        result.push_str(TAB);
        let mut i = 1;

        while i < data.len() {
            if data[i].eq(&'"') {
                if !data[i - 1].eq(&'\\') {
                    in_string = !in_string;
                }
            }

            if in_string {
                result.push(data[i]);
                i += 1;
                continue;
            }

            match data[i] {
                '{' => {
                    if !data[i - 1].eq(&'[') && !data[i - 1].eq(&'{') && !data[i - 1].eq(&',') {
                        result.push('\n');
                        for i in 0..tab_depth {
                            result.push_str(TAB);
                        }
                    }
                    result.push(data[i]);
                    result.push('\n');
                    tab_depth += 1;
                    for i in 0..tab_depth {
                        result.push_str(TAB);
                    }
                },
                '[' => {
                    if !data[i - 1].eq(&'[') && !data[i - 1].eq(&'{') && !data[i - 1].eq(&',') {
                        result.push('\n');
                        for i in 0..tab_depth {
                            result.push_str(TAB);
                        }
                    }
                    result.push(data[i]);
                    result.push('\n');
                    tab_depth += 1;
                    for i in 0..tab_depth {
                        result.push_str(TAB);
                    }
                },
                ':' => {
                    result.push_str(": ");
                },
                ',' => {
                    result.push_str(",\n");
                    for i in 0..tab_depth {
                        result.push_str(TAB);
                    }
                },
                '}' => {
                    tab_depth -= 1;
                    result.push('\n');
                    for i in 0..tab_depth {
                        result.push_str(TAB);
                    }
                    result.push(data[i]);
                },
                ']' => {
                    tab_depth -= 1;
                    result.push('\n');
                    for i in 0..tab_depth {
                        result.push_str(TAB);
                    }
                    result.push(data[i]);
                },
                _ => {
                    result.push(data[i]);
                }
            }

            i += 1;
        }

        return result
    }

    fn remove_formatting(&self, data: Vec<char>) -> String {
        let mut result = String::new();
        result.reserve(data.len());

        let mut in_string = false;
        for i in 0..data.len() {
            if data[i].eq(&'\"') {
                if !data[i-1].eq(&'\\') {
                    in_string = !in_string;
                }
            }
            if !in_string {
                if !data[i].eq(&' ') && !data[i].eq(&'\n') && !data[i].eq(&'\r') && !data[i].eq(&'\t') {
                    result.push(data[i]);
                }
            } else {
                result.push(data[i]);
            }
        }
        return result
    }

}

impl Index<usize> for JsonValue {
    type Output = JsonValue;
    fn index(&self, i: usize) -> &JsonValue {
        &self.array[i]
    }
}

impl IndexMut<usize> for JsonValue {
    fn index_mut(&mut self, i: usize) -> &mut JsonValue {
        if i >= self.array.len() {
            for _x in self.array.len()..i+1 {
                self.array.push(JsonValue::new());
            }
            self.var_type = VarType::Array;
        } 
        &mut self.array[i]
    }
}

impl Index<&str> for JsonValue {
    type Output = JsonValue;
    fn index(&self, i: &str) -> &JsonValue {
        &self.content[i]
    }
}

impl IndexMut<&str> for JsonValue {
    fn index_mut(&mut self, i: &str) -> &mut JsonValue {
        if !self.content.contains_key(i) {
            self.content.insert(i.to_string(), JsonValue::new());
            self.var_type = VarType::Object;
        }
        self.content.get_mut(i).unwrap()
    }
}
