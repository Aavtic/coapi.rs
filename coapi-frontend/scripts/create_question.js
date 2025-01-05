const add_button = document.querySelector(".add_button");
const submit_button = document.querySelector(".submit_button");
const title = document.querySelector(".title")
const description = document.querySelector(".description")

const types = ["int", "float", "bool", "str", "List[int]", "List[float]", "List[int]", "List[bool]", "List[str]"];
const types_str = ["int", "float", "bool", "str", "list_int", "list_float", "list_int", "list_bool", "list_str"];

let options = [];
let output_button_count = 2;


const web_host = "192.168.13.165";


// <label for="argument1">Argument 1:</label>
// <textarea id="arugment-1" name="argument-1" rows="3" placeholder="Enter the expected output" required></textarea>


function getSelectBoxValues() {
    const selectBoxes = document.querySelectorAll("select");
    let values = []; 

    selectBoxes.forEach((select, _index) => {
        values.push(select.value);
    });

    return values;
}

add_button.addEventListener("click", function () {
    output_button_count++;
    const newdiv = document.createElement("div");
    const newdiv1 = document.createElement("div");
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


    newdiv.appendChild(newLabel);
    newdiv.appendChild(newTextarea);

    newdiv1.appendChild(newLabel1);
    newdiv1.appendChild(newTextarea1);

    const container = document.getElementById("output-container");
    container.appendChild(newdiv);
    container.appendChild(newdiv1);
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
    let function_name = document.getElementById("function_name").value;
    let input_name = document.getElementById("input_name").value;

    console.log(function_name, input_name);

    let io_types = getSelectBoxValues();
    console.log("io types: " + io_types);
    let output_type, input_type;

    switch (io_types[0]) {
        case "int":
            input_type = "Int";
            break;

        case "float":
            input_type = "Float";
            break;

        case "str":
            input_type = "String";
            break;

        case "bool":
            input_type = "Bool";
            break;

        case "List[int]":
            input_type = "VecInt";
            break;

        case "List[float]":
            input_type = "VecFloat";
            break;

        case "List[str]":
            input_type = "VecString";
            break;

        case "List[bool]":
            input_type = "VecBool";
            break;
    }
    switch (io_types[1]) {
            case "int":
                output_type = "Int";
                break;

            case "float":
                output_type = "Float";
                break;

            case "str":
                output_type = "String";
                break;

            case "bool":
                output_type = "Bool";
                break;

            case "List[int]":
                output_type = "VecInt";
                break;

            case "List[float]":
                output_type = "VecFloat";
                break;

            case "List[str]":
                output_type = "VecString";
                break;

            case "List[bool]":
                output_type = "VecBool";
                break;
        }

    console.log("input type" + input_type + "output type" + output_type);

    let args = [];
    let outs = [];
    let myJson = {
        "title": curr_title,
        "description": curr_desc,
        "data": 
                [],
        "function_name": function_name,
        "input_name": input_name,
        "input_type": input_type,
        "output_type": output_type
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
          `http://${web_host}:8081/api/v1/create_question`, {
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
