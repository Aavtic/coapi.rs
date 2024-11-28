const run_button = document.querySelector(".runbutton");
const outputtext = document.querySelector(".outputext");

function update_output(text) {  
    let curr_text = outputtext.textContent;
    outputtext.textContent = curr_text + text;
}

run_button.onclick = () => {
  const socket = new WebSocket("ws://127.0.0.1:8081/ws/get_live_output");

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
