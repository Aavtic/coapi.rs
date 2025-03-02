import {IP, PORT} from './constants.js'

const web_host = IP;

function setUpOnQuestionClick() {
    const divs = document.querySelectorAll('.question');

    divs.forEach(div => {
        div.addEventListener('click', function() {
            window.open(`http://${web_host}:${PORT}/pages/question/${this.id}`, '_blank');
        });
    });
}


function load_questions() {
    const request = new Request(
          `http://${web_host}:${PORT}/api/v1/get_questions`, {
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
