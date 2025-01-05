const run_button = document.querySelector(".runbutton");
const test_button = document.querySelector(".test_code");
const submit_button = document.querySelector(".submit_code");
const inneroutput = document.getElementById("inneroutput");
const outputext = document.querySelector(".outputext");
const cursor = document.querySelector(".cursor");
const statusbar = document.querySelector(".statustext");
const output_text = document.querySelector(".output-text");
const sub_text = document.querySelector(".sub-text");


const web_host = "192.168.13.165"


console.log("coide");
let userInput = '';
let msg_buffer = [];
let keyboardListener;


function clear_msgs() {
    statusbar.textContent = "";
    output_text.textContent = "";
    sub_text.textContent = "";

    output_text.id = "nil";
    sub_text.id = "nil";
}

function update_output(jsonResp) {  
    cursor.remove();
    let curr_text = outputext.textContent;
    outputext.textContent = curr_text + jsonResp["character"];
    outputext.appendChild(cursor);
    
    if (jsonResp["over"] == true) {
        let status = jsonResp["exit_status"];
        console.log("removing event listener");
        inneroutput.removeEventListener("keydown", keyboardListener);
        statusbar.textContent = `Execution complete with status ${status}`
    }
}

function createKeyboardListener(socket) {
    return (e) => {
        // console.log("pressed", e.key);
        if (e.key.length === 1) {
            userInput += e.key;
            console.log("current input ", userInput);
            cursor.remove();
            outputext.textContent += e.key;
            outputext.appendChild(cursor);

        } else if (e.key === 'Enter') {
            let o = document.getElementById("outputext");
            if (o) {
                cursor.remove();
                o.textContent += "\n";
                outputext.appendChild(cursor);
                console.log("Sending", userInput);
                socket.send(userInput);
                userInput = '';
            }

        } else if (e.key === 'Backspace') { 
            cursor.remove();
            let removed = userInput.slice(0, -1);
            let text = outputext.textContent;
            if (userInput.length) {
                userInput = removed;
                console.log(text);
                let whole_removed = text.slice(0, text.length - 1);
                console.log(whole_removed);
                outputext.textContent = whole_removed;
                outputext.appendChild(cursor);
            }
        }
    };
}

function run_button_test() {
    clear_msgs();
    const codeBox = document.querySelector(".codebox");

    const url = window.location.href;  // Get the current URL
    const parts = url.split('/');      // Split the URL by '/'

    const id = parts[parts.length - 1]; // Get the last part

    const myJson = {
        "code": codeBox.value,
        "question_id": id, 
        "language": "Python",
    };

    const myHeaders = new Headers();
    myHeaders.append("content-type", "application/json");

    const request = new Request(
          `http://${web_host}:8081/api/v1/test_code`, {
          method: "POST",
          headers: myHeaders,
          body: JSON.stringify(myJson),
      });

    fetch(request)
      .then((response) => {
          if (response.status === 200) {
              return response.text();
          } else {
              throw new Error("API request failed!");
          }
      })
    .then((text) => {
        console.log("got response: " + text);
        let test_results = JSON.parse(text);
        console.log(test_results);
        let status = test_results["status"];
        clear_msgs();
        
        //Cooked,
        //URCodeErrorLOL,
        //URCodeDontReturnAnything,
        if (status === "Pass") {
            output_text.textContent = "Passed all test cases.";
            output_text.id = "pass_id";
        } else if (status["Fail"] != undefined) {
            let expected = status["Fail"]["ex"];
            let got = status["Fail"]["got"];
            let input = status["Fail"]["input"];
            output_text.textContent = "Test Case Failed";
            sub_text.textContent = `Input: ${input}, Expected :${expected} but got: ${got}.`
            sub_text.id = "fail_id_sub"
            output_text.id = "fail_id";
        }else if (status["URCodeErrorLOL"] != undefined) {
            let error = status["URCodeErrorLOL"]["error"];
            output_text.textContent = "Error:";
            sub_text.textContent = `Error: ${error}`
            output_text.id = "error_id";
            sub_text.id = "error_id_sub";
        } else if (status === "URCodeDontReturnAnything") {
            output_text.textContent = "Your code may not return values in all cases";
            output_text.id = "noreturn_id";
        } else if (status === "Cooked") {
            output_text.textContent = "SERVER FAULT please retry";
            output_text.id = "cooked_id";
        }
    })
    .catch((error) => {
        console.error(error);
    });
}

function run_button_fn_ws() {
    clear_msgs();
    const socket = new WebSocket(`ws://${web_host}:8081/ws/get_live_output`);
    outputext.textContent = "";

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
        let resp  = event.data;
        let data =  resp.split("\u0000").join("");
        let jsonResponse;
        console.log("Message from server ", data);
        try {
            if (msg_buffer.length === 0){
                jsonResponse = JSON.parse(data);
            } else {
                jsonResponse = JSON.parse(buff.join(""));
            }
        }catch {
            msg_buffer.push(data);
            return;
        }
        console.log(jsonResponse);
        update_output(jsonResponse);
    });
    keyboardListener = createKeyboardListener(socket);
    inneroutput.addEventListener('keydown', keyboardListener);
}

test_button.onclick = run_button_test;
run_button.onclick = run_button_fn_ws;
