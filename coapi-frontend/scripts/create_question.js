const add_button = document.querySelector(".add_button");
const submit_button = document.querySelector(".submit_button");
const title = document.querySelector(".title")
const description = document.querySelector(".description")

let output_button_count = 1;



// <label for="argument1">Argument 1:</label>
// <textarea id="arugment-1" name="argument-1" rows="3" placeholder="Enter the expected output" required></textarea>

add_button.addEventListener("click", function () {
    output_button_count++;

    const newLabel = document.createElement("label");
    const newLabel1 = document.createElement("label");

    newLabel1.setAttribute("for", `argument${output_button_count}`)
    newLabel1.textContent = `Argument ${output_button_count}:`

    newLabel.setAttribute("for", `output-${output_button_count}`);
    newLabel.textContent = `Expected Output ${output_button_count}:`;

    const newTextarea1 = document.createElement("textarea");
    newTextarea1.id = `argument-${output_button_count}`;
    newTextarea1.name = "argument";
    newTextarea1.rows = "3";
    newTextarea1.placeholder = "Enter the argument";
    newTextarea1.required = true;

    const newTextarea = document.createElement("textarea");
    newTextarea.id = `expected-output${output_button_count}`;
    newTextarea.name = "expected-output";
    newTextarea.rows = "3";
    newTextarea.placeholder = "Enter the expected output";
    newTextarea.required = true;

    const container = document.getElementById("output-container");
    container.appendChild(newLabel1);
    container.appendChild(newTextarea1);
    container.appendChild(newLabel);
    container.appendChild(newTextarea);
});

// {
//     "title": "",
//     "description": "",
//     "data": {
//         {
//             [
//                 {
//                     "input": "",
//                     "output": "",
//                 }
//             ]
//         }
//     }
// }


submit_button.addEventListener("click", function(event) {
    event.preventDefault();

    let curr_title = document.getElementById("title").value;
    let curr_desc = document.getElementById("description").value;

    let args = [];
    let outs = [];
    let myJson = {
        "title": curr_title,
        "description": curr_desc,
        "data": 
                []
    }
    const myHeaders = new Headers();
    myHeaders.append("content-type", "application/json");


    const arguments = document.querySelectorAll("textarea[name='argument']");
    const outputs = document.querySelectorAll("textarea[name='expected-output']");

    arguments.forEach((argument, index) => {
        console.log(`Argument ${index + 1}: ${argument.value}`);
        args.push(argument.value);
    });

    outputs.forEach((output, index) => {
        console.log(`Output ${index + 1}: ${output.value}`);
        outs.push(output.value);
    });

    console.log("Title:", curr_title);
    console.log("Description:", curr_desc);
    console.log("Arguments:", args);
    console.log("Outputs:", outs);

    if (args.length === outs.length) {
        for (i=0; i<args.length; i++) {
            myJson["data"][i] = {
                "input": args[i],
                "output": outs[i]
            }
        }
    }

    console.log(myJson)

    const request = new Request(
          "http://127.0.0.1:8081/api/v1/create_question", {
          method: "POST",
          headers: myHeaders,
          body: JSON.stringify(myJson),
      }
    )

    fetch(request)
      .then((response) => {
          if (response.status === 200) {
              return 1;
          } else {
              throw new Error("API request failed!");
          }
      })
    .then((_) => {
        const notification = document.getElementById("notification");
        notification.style.display = "block";
        setTimeout(() => {
            notification.style.display = "none";
        }, 3000);

    })
    .catch((error) => {
        console.error(error);
    });
});
