const button = document.querySelector("button");
const codeBox = document.querySelector("textarea");
const output = document.querySelector(".outputext");
const output_heading  = document.querySelector(".output-text");
const status_text  = document.querySelector(".statustext");

const web_host = "192.168.13.165";


button.onclick = () => {
const myHeaders = new Headers();
    myHeaders.append("content-type", "application/json");


    const myCode = codeBox.value;

    const myJson = {
        "code" : myCode,
        "language": "Python"
    };

    const request = new Request(
          `http://${web_host}:8081/api/v1`, {
          method: "POST",
          headers: myHeaders,
          body: JSON.stringify(myJson),
      }
    )

    fetch(request)
      .then((response) => {
          if (response.status === 200) {
              return response.json();
          } else {
              throw new Error("API request failed!");
          }
      })
    .then((json) => {
        console.log(json);
        displayOutput(json);
    })
    .catch((error) => {
        console.error(error);
    });
}

function displayOutput(output_json) {
    output_heading.textContent = "Output";
    if (output_json.status === 0) {
        output.style.color = "grey";
        status_text.style.color = "green"
        output.textContent = output_json.output;
        status_text.textContent = `STATUS: ${output_json.status}`
    } else {
        output.textContent = output_json.output;
        output.style.color = "red";
        status_text.style.color = "red"
        status_text.textContent = `STATUS: ${output_json.status}`
    }

}
