import ast


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
