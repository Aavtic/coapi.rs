const socket = new WebSocket("ws://127.0.0.1:8081/ws/get_live_output")

socket.addEventListener("open", (event) => {
  const codeBox = document.querySelector(".codebox");
  
  const myJson = {
    "code": codeBox.value,
    "language": "Python",
  };
  
  socket.send(JSON.stringify(myJson));
});

socket.addEventListener("message", (event) => {
  console.log("Message from server ", event.data);
}); 