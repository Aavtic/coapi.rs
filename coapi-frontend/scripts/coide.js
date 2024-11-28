const run_button = document.querySelector(".runbutton");
const inneroutput = document.getElementById("inneroutput");
const outputext = document.querySelector(".outputext");
const cursor = document.querySelector(".cursor");


console.log("coide");
let userInput = '';

function update_output(text) {  
    cursor.remove();
    let curr_text = outputext.textContent;
    outputext.textContent = curr_text + text;
    outputext.appendChild(cursor);
}

run_button.onclick = () => {
  const socket = new WebSocket("ws://127.0.0.1:8081/ws/get_live_output");
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
    let data = event.data
    console.log("Message from server ", data);
    update_output(data)
  });
  inneroutput.addEventListener('keydown', (e) => {
        // console.log("pressed", e.key);
        if (e.key.length === 1) {
            userInput += e.key;
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
    });
}
