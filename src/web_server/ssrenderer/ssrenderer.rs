use axum::response::Html;
use crate::web_server::database::mongo_funcs::DbAddQuestion;


pub fn error_page() -> Html<String> {
    let error_page = format!(r#"

<!DOCTYPE html> 
<html lang="en"> 
  
<head> 
    <meta charset="UTF-8"> 
    <meta name="viewport" 
          content="width=device-width,  
                   initial-scale=1.0"> 
    <title> 
        404 Page Not Found 
    </title> 
    <link rel="stylesheet" 
          href="styles/error_style.css"> 
</head> 
  
<body> 
    <div class="error-container"> 
        <h1> 404 </h1> 
        <p> 
            Oops! The page you're 
            looking for is not here. 
        </p> 
        </a> 
    </div> 
</body> 
  
</html>

"#);
    return Html::from(error_page.to_string());
}


pub fn generate_question_html(question: DbAddQuestion) -> Html<String> {
    let title = question.title;
    let description = question.description;

    let question_html = format!(r#"
                                <div class="question">
                                <h2 name="title">{title}</h2>
                                <h3 name="description">{description} </h3>
                                </div>
                                "#, title=title, description=description);
    let html_document = format!(r#"
<html lang="en"><head>
    <meta charset="utf-8">
    <title class="title">{title}</title>
    <link rel="stylesheet" href="http://127.0.0.1:8081/styles/view_question.css">
  </head>

  <body>
  <div class="lcontainer">
      <div class="header">
          <h1><code class="main_title">CO-IDE<b class="cursor">▊</b></code></h1>
      </div>
      <div class="left">
      <div class="question_container">
      {question_html}
      </div>
  </div>
  </div>
      <div class="right">
           <div class="container">

     <div class="code_box_button">
     <textarea class="codebox" placeholder="Enter your code here..." rows=20 cols=80> 
     </textarea>

     <button class="runbutton">
         Run
     </button>

     </div>

     <div class="output">
         <h2 class="output-text"></h2>
         <p class="outputext">
         <h3 class="statustext"></h3>
         </p>
     </div>

     </div>

     <div class="footer">
    <p>
    This is an open source project and is available on <a href="https://github.com/aavtic/coapi.rs">GitHub</a>
    </p>
    </div>
      </div>
    <script src="http://127.0.0.1:8081/scripts/view_question.js"></script>
  </body></html>
"#, title=title, question_html=question_html);
    return Html::from(html_document);
}

pub fn generate_questions_html(questions: Vec<DbAddQuestion>) ->  Html<String> {
    let mut html = String::new();
    for question in questions {
        let title = question.title;
        let description = question.description;
        let id = question.uuid;
        let html_formatted = format!(r#"
<div class="question" id="{id}">
<h2 name="title">{title}</h2>
<h3 name="description">{description} </h3>
</div>
"#, id=id, title=title, description=description);
        html.push_str(&html_formatted);
    }
    return Html::from(html.to_string());
}
