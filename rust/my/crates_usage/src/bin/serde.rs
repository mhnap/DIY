use serde::{Deserialize, Serialize};

// Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.

// The Serde ecosystem consists of data structures that know how to serialize and deserialize themselves along with data formats that know how to serialize and deserialize other things.
// Serde provides the layer by which these two groups interact with each other, allowing any supported data structure to be serialized and deserialized using any supported data format.

fn main() {
    #[derive(Debug, Deserialize, Serialize)]
    struct MyData {
        number: i32,
        string: String,
        flag: bool,
        option: Option<String>,
        vec: Vec<u32>,
    }

    let data = MyData {
        number: 42,
        string: "hello".into(),
        flag: true,
        option: Some("value".into()),
        vec: vec![1, 2, 3],
    };

    // Serialize `data` to JSON as a String.
    let json = serde_json::to_string(&data).unwrap();
    dbg!(&json);

    // Deserialize `MyData` from `json`.
    let data: MyData = serde_json::from_str(&json).unwrap();
    dbg!(data);

    // Can construct a `serde_json::Value` from a JSON literal.
    let json = serde_json::json!({
        "number": 50,
        "string": "hi",
        "flag": false,
        "option": "new_value",
        "vec": [4, 5, 6]
    });
    dbg!(&json);

    let data: MyData = serde_json::from_value(json).unwrap();
    dbg!(data);

    //

    // But deserialization can fail if wrong format.
    let json = serde_json::json!({
        "number": 50,
        "string": "hi",
        "flag": false,
        // "vec": [1] // Will cause Error("missing field `vec`", line: 0, column: 0)
    });
    dbg!(&json);

    match serde_json::from_value::<MyData>(json.clone()) {
        Ok(data) => {
            dbg!(data);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    //

    // Or we can use the `#[serde(default)]` attribute on the field to use its `Default::default()` during deserialization.
    #[derive(Debug, Deserialize, Serialize)]
    struct MyData2 {
        number: i32,
        string: String,
        flag: bool,
        // Note, we don't need to specify `#[serde(default)]` for `Option`, it's already by default.
        // https://github.com/serde-rs/serde/issues/1728#issuecomment-1627829894
        option: Option<String>,
        #[serde(default)]
        vec: Vec<u32>,
    }

    match serde_json::from_value::<MyData2>(json.clone()) {
        Ok(data) => {
            dbg!(data);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    //

    // Or we can specify a default value for some field used during deserializing, but only via function.
    // There is an open issue for supporting default literals, see <https://github.com/serde-rs/serde/issues/368>.
    #[derive(Debug, Deserialize, Serialize)]
    struct ShortMyData {
        number: i32,
        #[serde(default = "default_string")]
        string: String,
    }

    fn default_string() -> String {
        "Mike".into()
    }

    let json = serde_json::json!({
        "number": 50,
    });
    dbg!(&json);

    match serde_json::from_value::<ShortMyData>(json.clone()) {
        Ok(data) => {
            dbg!(data);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    //

    // But `#[serde(default)]` attribute won't help if we encontered JSON `null` value.
    // https://github.com/serde-rs/serde/issues/1098#issuecomment-1333665788
    let json = serde_json::json!({
        "number": 50,
        "string": "hi",
        "flag": false,
        "vec": null // Will cause Error("invalid type: null, expected a sequence", line: 0, column: 0)
    });
    dbg!(&json);

    match serde_json::from_value::<MyData2>(json.clone()) {
        Ok(data) => {
            dbg!(data);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    //

    // But `Option` will handle JSON `null` value gracefully.
    let json = serde_json::json!({
        "number": 50,
        "string": "hi",
        "flag": false,
        "option": null
    });
    dbg!(&json);

    match serde_json::from_value::<MyData2>(json.clone()) {
        Ok(data) => {
            dbg!(data);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    //

    // Unknown fields are not denied by default.
    let json = serde_json::json!({
        "number": 50,
        "string": "hi",
        "flag": false,
        "some_field": 3
    });
    dbg!(&json);

    match serde_json::from_value::<MyData2>(json.clone()) {
        Ok(data) => {
            dbg!(data);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    //

    // But can specify `#[serde(deny_unknown_fields)]` attribute to deny unknown fields.
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)] // Will cause Error("unknown field `some_field`, expected one of `number`, `string`, `flag`, `option`, `vec`", line: 0, column: 0)
    struct MyData3 {
        number: i32,
        string: String,
        flag: bool,
        option: Option<String>,
        #[serde(default)]
        vec: Vec<u32>,
    }

    match serde_json::from_value::<MyData3>(json.clone()) {
        Ok(data) => {
            dbg!(data);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    //

    // Can use untagged enum representation.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Plugins {
        plugins: Vec<Plugin>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Plugin {
        pub check_delay_sec: i64,
        pub plugin_config: PluginConfig,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum PluginConfig {
        Empty,
        Ping(String),
        SystemdUnitsChecker(Vec<String>),
    }

    let json = serde_json::json!({
        "plugins":
            [
             {
                "check_delay_sec": 6,
                "plugin_config": null
             },
             {
                "check_delay_sec": 7,
                "plugin_config": "8.8.8.8"
             },
             {
                "check_delay_sec": 3,
                "plugin_config": ["0.0.0.0", "1.1.1.1"]
             },
            ]
    });

    let data: Plugins = serde_json::from_value(json).unwrap();
    dbg!(data);

    //

    // By default serde do not change enum variants names.
    #[derive(Debug, Deserialize, Serialize)]
    enum MyEnum {
        VariantOne(i32),
        VariantTwo { name: String },
        VariantThree,
    }

    let vec = vec![
        MyEnum::VariantOne(42),
        MyEnum::VariantTwo {
            name: "Mike".to_string(),
        },
        MyEnum::VariantThree,
    ];
    dbg!(serde_json::to_string(&vec));

    //

    // But can rename all of them.
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    enum MyEnum2 {
        VariantOne(i32),
        VariantTwo { name: String },
        VariantThree,
    }

    let vec = vec![
        MyEnum2::VariantOne(42),
        MyEnum2::VariantTwo {
            name: "Mike".to_string(),
        },
        MyEnum2::VariantThree,
    ];
    dbg!(serde_json::to_string(&vec));
}
