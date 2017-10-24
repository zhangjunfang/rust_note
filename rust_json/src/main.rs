extern crate serde;
#[macro_use]  extern crate serde_json;
#[macro_use]  extern crate serde_derive;
use serde_json::Error;
use serde_json::Value;
#[derive(Serialize, Deserialize)]
 struct Person {
     name: String,
     age: u8,
     phones: Vec<String>,
 }


 fn main() {
     typed_example().unwrap();

     untyped_example().unwrap();

     json_values();

     println!("--------------------------------------------");
     print_an_address().unwrap();

}
fn  json_values(){
    let john = json!({
       "name": "John Doe",
       "age": 43,
       "phones": [
         "+44 1234567",
         "+44 2345678"
       ]
     });
    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
fn untyped_example() -> Result<(), Error> {
         // Some JSON input data as a &str. Maybe this comes from the user.
         let data = r#"{
                         "name": "John Doe",
                         "age": 43,
                         "phones": [
                           "+44 1234567",
                           "+44 2345678"
                         ]
                       }"#;
    
         // Parse the string of data into serde_json::Value.
         let v: Value = serde_json::from_str(data)?;
    
         // Access parts of the data by indexing with square brackets.
         println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    
         Ok(())
}

fn typed_example() -> Result<(), Error> {

    let data = r#"{
                     "name": "John Doe",
                     "age": 43,
                     "phones": [
                       "+44 1234567",
                       "+44 2345678"
                     ]
                   }"#;
    let p: Person = serde_json::from_str(data)?;
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}



#[derive(Serialize, Deserialize)]
 struct Address {
     street: String,
     city: String,
 }

 fn print_an_address() -> Result<(), Error> {
     // Some data structure.
     let address = Address {
         street: "10 Downing Street".to_owned(),
         city: "London".to_owned(),
     };

     // Serialize it to a JSON string.
     let j = serde_json::to_string(&address)?;

     // Print, write to a file, or send to an HTTP server.
     println!("{}", j);

     Ok(())
 }































extern crate rustc_serialize;
// 引入rustc_serialize模块
use rustc_serialize::json;
// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
// 定义TestStruct
#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct  {
    pub data_int: u8,
    pub data_str: String,
    pub data_vector: Vec<u8>,
}
#[derive(RustcDecodable, RustcEncodable)]
pub  struct TestA{
    pub data : TestStruct,
    pub a : String,
}


pub fn  old_json_parse(){
    // 初始化TestStruct
    let t = TestStruct {
        data_int: 1,
        data_str: "homura".to_string(),
        data_vector: vec![2,3,4,5],
    };
    let m="fsdfsdfsdf";
    let object=TestA{
        data:t,
        a:m.to_string(),
    };

    // Serialize using `json::encode`
    // 将TestStruct转意为字符串
    let encoded = json::encode(&object).unwrap();
    println!("{}",encoded);
    // Deserialize using `json::decode`
    // 将json字符串中的数据转化成TestStruct对应的数据，相当于初始化
    let decoded: TestA = json::decode(&encoded).unwrap();
    println!("{:?}",decoded.data.data_vector);
}