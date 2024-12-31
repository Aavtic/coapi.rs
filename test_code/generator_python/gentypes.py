import ast
import json


def get_checked_type(st: str, typ: str):
    # supported_types = ["str", "int", "float", "bool", "List[int]", "List[str]", "List[bool]", "List[float]"]
    evaluated = ast.literal_eval(st)
    match typ:
        case "str":
            if isinstance(evaluated, str):
                return evaluated
            else:
                raise TypeError("Not expected type")
        case "int":
            if isinstance(evaluated, int):
                return evaluated
            else:
                raise TypeError("Not expected type")
        case "float":
            if isinstance(evaluated, float):
                return evaluated
            else:
                raise TypeError("Not expected type")
        case "bool":
            if isinstance(evaluated, bool):
                return evaluated
            else:
                raise TypeError("Not expected type")
        case "List[int]":
            if isinstance(evaluated, list) and all(isinstance(x, int) for x in evaluated):
                return evaluated
            else:
                raise TypeError("Not expected type")
        case "List[str]":
            if isinstance(evaluated, list) and all(isinstance(x, str) for x in evaluated):
                return evaluated
            else:
                raise TypeError("Not expected type")
        case "List[bool]":
            if isinstance(evaluated, list) and all(isinstance(x, bool) for x in evaluated):
                return evaluated
            else:
                raise TypeError("Not expected type")
        case "List[float]":
            if isinstance(evaluated, list) and all(isinstance(x, float) for x in evaluated):
                return evaluated
            else:
                raise TypeError("Not expected type")
        case _:
            raise TypeError("Unsupported Type")


class GenTypes:
    def __init__(self, filename):
        self.filename = filename
        self.contents = self.get_contents()

    def get_function_name(self):
        return self.parsed["function_name"]

    def parse_contents(self):
        self.parsed = json.loads(self.contents)
        input_output = self.parsed["input_output"]
        input_type_str = self.parsed["input_type"]
        # output_type_str = self.parsed["output_type"]
        input_output_res = []

        for input in input_output:
            try:
                self.input_type = get_checked_type(input, input_type_str)
                print(self.input_type, type(self.input_type), input_output[input], type(input_output[input]))
                # self.output_type = get_checked_type(input_output[input], output_type_str)
                # print(self.output_type)
            except Exception as e:
                raise e

            input_output_res.append((self.input_type, input_output[input]))

        return input_output_res

    def get_contents(self):
        contents = ""
        with open(self.filename, "r") as f:
            contents = f.read()
        return contents

# supported_types = ["str", "int", "float", "bool", "List[int]", "List[str]", "List[bool]", "List[float]"]

def test():
    type1 = get_checked_type("1", "int")
    print(type1, type(type1))

    type1 = get_checked_type("1.13", "int")
    print(type1, type(type1))

    type1 = get_checked_type('"hello there"', "str")
    print(type1, type(type1))

    type1 = get_checked_type("True", "bool")
    print(type1, type(type1))

    type1 = get_checked_type("[1, 2, 3]", "List[int]")
    print(type1, type(type1))

    type1 = get_checked_type("[1.2, 2.3]", "List[float]")
    print(type1, type(type1))

    type1 = get_checked_type('["hello friend", "hello there"]', "List[str]")
    print(type1, type(type1))

    type1 = get_checked_type("[True, False, False]", "List[bool]")
    print(type1, type(type1))
