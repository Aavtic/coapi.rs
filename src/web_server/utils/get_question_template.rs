use std::fs::File;
use std::io::Read;


pub fn get_template_question(uuid: String) -> Result<String, std::io::Error>{
    let file = File::open(format!("./question_blueprints/{}/main.py", uuid));
    let mut file_contents = String::new();
    file?.read_to_string(&mut file_contents)?;

    return Ok(file_contents)
}
