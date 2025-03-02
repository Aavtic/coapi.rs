use axum::response::Html;
use crate::web_server::database::mongo_funcs::DbAddQuestion;
use crate::WEB_HOST;


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
    let code_template = question.code_template.unwrap_or("# COAPI: https://github.com/aavtic/coapi.rs".to_string());

    let question_html = format!(r#"
                                <div class="question">
                                <h1 name="title">{title}</h1>
                                <div name="description" class="description"> {description} </div>
                                </div>
                                "#, title=title, description=description);
    let html_document = format!(r#"
<html lang="en"><head>
    <meta charset="utf-8">
    <title class="title">{title}</title>
    <link rel="stylesheet" href="http://{WEB_HOST}:8081/styles/view_question.css">
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
{code_template}
     </textarea>

     <div class="function_buttons">
         <button class="runbutton">
             Run
         </button>

         <button class="test_code">
            Test Code
         </button>

         <button class="submit_code">
            Submit
         </button>
     </div>

     </div>
     <div class="output" id="inneroutput" tabindex="0">
<h1 class="output-text" id="nil"></h1>
<h2 class="sub-text" id="nil"></h2>
<div id="output_cursor"><pre id="outputext" class="outputext"><span class="cursor">▊</span></pre></div>
<h3 class="statustext"></h3>
     </div>
     </div>

     <div class="footer">
    <p>
    This is an open source project and is available on <a href="https://github.com/aavtic/coapi.rs">GitHub</a>
    </p>
    </div>
      </div>
      <script type="module" src="http://{WEB_HOST}:8081/scripts/constants.js"></script>
      <script type="module" src="http://{WEB_HOST}:8081/scripts/coide.js"></script>
  </body></html>
"#, title=title, question_html=question_html);
    return Html::from(html_document);
}
//<script>
//hljs.highlightAll();
//
//setInterval(() => {{
//    const codebox = document.querySelector(".codebox");
//
//    codebox.dataset.highlighted = null;
//    hljs.highlightAll();
//
//}}, 100);
//
//
//</script>

pub fn generate_questions_html(questions: Vec<DbAddQuestion>) ->  Html<String> {
    let mut html = String::new();
    for question in questions {
        let title = question.title;
        let id = question.uuid;
        let html_formatted = format!(r#"
<div class="question" id="{id}">
<h2 name="title">{title}</h2>
</div>
"#, id=id, title=title);
        html.push_str(&html_formatted);
    }
    return Html::from(html.to_string());
}
