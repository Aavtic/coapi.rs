import {IP, PORT} from 'constants.js';

const run_button = document.querySelector(".runbutton");
const outputtext = document.querySelector(".outputext");

// enable_tabspace();

const web_host = IP;


function update_output(text) {  
    let curr_text = outputtext.textContent;
    outputtext.textContent = curr_text + text;
}

run_button.onclick = () => {
    const codeBox = document.querySelector(".codebox");

    const url = window.location.href;  // Get the current URL
    const parts = url.split('/');      // Split the URL by '/'

    const id = parts[parts.length - 1]; // Get the last part

    const myJson = {
        "code": codeBox.value,
        "question_id": id, 
        "language": "Python",
    };

    const request = new Request(
          `http://${web_host}:${PORT}/api/v1/test_code`, {
          method: "POST",
          headers: myHeaders,
          body: JSON.stringify(myJson),
      });

    fetch(request)
      .then((response) => {
          if (response.status === 200) {
              return response.text;
          } else {
              throw new Error("API request failed!");
          }
      })
    .then((text) => {
        console.log(text);
    })
    .catch((error) => {
        console.error(error);
    });
}

function live_code_execution() {
    const socket = new WebSocket(`ws://${web_host}:${PORT}/ws/get_live_output`);

    socket.addEventListener("open", (_event) => {
        const codeBox = document.querySelector(".codebox");

        const myJson = {
            "code": codeBox.value,
            "language": "Python",
        };
        console.log("sending", JSON.stringify(myJson));
        socket.send(JSON.stringify(myJson));
    });

    socket.addEventListener("message", (event) => {
        let data = event.data
        console.log("Message from server ", data);
        update_output(data)
    });
}


