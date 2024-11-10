use axum::response::Html;
use crate::axum_serve::AddQuestion;


pub fn generate_questions_html(questions: Vec<AddQuestion>) ->  Html<String> {
    let mut html = String::new();
    for question in questions {
        let title = question.title;
        let description = question.description;
        let html_formatted = format!(r#"
<div class="question">
<h2 name="title">{title}</h2>
<h3 name="description">{description} </h3>
</div>
"#, title=title, description=description);
        html.push_str(&html_formatted);
    }
    return Html::from(html.to_string());
}
