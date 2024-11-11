
function setUpOnQuestionClick() {
    const divs = document.querySelectorAll('.question');

    divs.forEach(div => {
        div.addEventListener('click', function() {
            window.open(`http://127.0.0.1:8081/pages/question/${this.id}`, '_blank');
        });
    });
}


function load_questions() {
    const request = new Request(
          "http://127.0.0.1:8081/api/v1/get_questions", {
          method: "GET",
          headers: {
            "Content-Type": "text/html",
        },

      }
    )

    fetch(request)
        .then(response => {
        if (!response.ok) {
            throw new Error("Network response was not ok");
        }
        return response.text(); // Expecting an HTML string
    })
    .then(html => {
        const contentContainer = document.querySelector(".questions_container");
        contentContainer.innerHTML = html; // Inject the HTML into the DOM
        setUpOnQuestionClick();
    })
    .catch(error => {
        console.error("Error loading content:", error);
    });

}

load_questions();
