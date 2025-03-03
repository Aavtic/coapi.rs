import {IP, PORT} from "./constants.js";

const run_button = document.querySelector(".runbutton");
const inneroutput = document.getElementById("inneroutput");
const outputext = document.querySelector(".outputext");
const cursor = document.querySelector(".cursor");
const statusbar = document.querySelector(".statustext");
const output_text = document.querySelector(".output-text");
const sub_text = document.querySelector(".sub-text");


enable_tabspace();


console.log("Main");
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

function run_button_fn_ws() {
    clear_msgs();
    const socket = new WebSocket(`ws://${IP}:${PORT}/ws/get_live_output`);
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

run_button.onclick = run_button_fn_ws;


function enable_tabspace() {
    console.log("Running...");

    function keyHandler(e) {
        var TABKEY = 9;
        if(e.keyCode == TABKEY) {
            this.value += "    ";
            if(e.preventDefault) {
                e.preventDefault();
            }
            return false;
        }
    }
    var myInput = document.querySelector(".codebox");
    if(myInput.addEventListener ) {
        myInput.addEventListener('keydown', keyHandler, false);
    } else if(myInput.attachEvent ) {
        myInput.attachEvent('onkeydown', keyHandler); /* damn IE hack */
    }
}
