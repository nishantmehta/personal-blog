use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::string;
use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use serde_json::{Value, from_reader, to_writer_pretty};
use uuid::Uuid;


// Define the structure of your object
#[derive(Serialize, Deserialize, Debug)]
pub struct MyObject {
    pub date: DateTime<Utc>,
    pub string: String,
    pub id: Uuid,
}

// Function to store a list of objects
pub fn store_objects(file_path: &Path, objects: &[MyObject]) -> std::io::Result<()> {
   // 1. Read existing objects from the file
   let mut file = OpenOptions::new().read(true).open(file_path)?;
   let mut existing_data = String::new();
   file.read_to_string(&mut existing_data)?;

   // 2. Parse JSON and extract the array
   let mut json_data: Value = from_reader(existing_data.as_bytes())?;
   let objects_array = json_data
       .as_array_mut()
       .expect("File should contain a JSON array");

   // 3. Serialize new objects and add them to the array
   for obj in objects {
       let serialized_obj = serde_json::to_value(obj)?;
       objects_array.push(serialized_obj);
   }

   // 4. Write the updated JSON data back to the file
   let mut file = OpenOptions::new()
       .write(true)
       .truncate(true)
       .open(file_path)?;
   to_writer_pretty(file, &json_data)?;

   Ok(())
}

// Function to retrieve a list of objects
pub fn retrieve_objects(file_path: &Path) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let objects: Vec<MyObject> = from_reader(file)?;

    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><body>");
    html.push_str("<h1>My Objects</h1>");
    html.push_str("<ul>");

    for obj in objects {
        html.push_str(&format!(
            "<li>Date: {}, Text: {}, ID: {}</li>",
            obj.date.format("%Y-%m-%d"),
            obj.string,
            obj.id
        ));
    }

    html.push_str("</ul></body></html>");
    Ok(html)
}
