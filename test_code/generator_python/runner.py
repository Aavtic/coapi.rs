import json
import gentypes
import os
import sys
import importlib
import argparse


WORKING_DIRECTORY_ROOT = "/home/aad1sh/rust-programming/coapi-rs"

parser = argparse.ArgumentParser(description="Run and test Code with their respective input and outputs")
parser.add_argument("--question_details", required=True, help="JSON question structure.")


# INPUT
# {
#         "question_id": "lksjdfalskjf",
#         "filepath": ""
# }

# OUTPUT

# {
#         "status": Pass | Fail: {"ex": "","got":""} | URCodeErrorLOL | URCodeDontReturnAnything | Cooked
# }

# LOAD <question-id>/qndetails.json
# import pyl23k4j/main.py as code

# PSEUDO CODE
# sol = code.Solution()
# for (input, output) in inputs:
#     try:
#         code_output = sol.function_name(input)
#     except Exception as e:
#         return report(e)
#
#     if code_output == output:
#         return report("Passed")
#     else:
#         return report("Failed")


class URCodeErrorLOL(Exception):
    def to_string(self):
        return '{"status": "URCodeErrorLOL"}'


class Pass:
    def to_string(self):
        return '{"status": "Pass"}'


class Cooked:
    def to_string(self):
        return '{"status": "Cooked"}'


class URCodeDontReturnAnything(Exception):
    def to_string(self):
        return '{"status": "URCodeDontReturnAnything"}'


class Fail(Exception):
    def __init__(self, expected, got):
        self.expected = expected
        self.got = got

    def to_string(self):
        with open("runner.log", "a") as f:
            f.write("result: " + str(self.expected) + "output " + str(self.got) + "\n")
        return '{"status": {"Fail" : {"ex": "' + str(self.expected) + '", "got": "' + str(self.got) + '"}}}'
        # return f'{{"status": "Fail: {"ex": {str(self.expected)}, "got": {str(self.got)}}"}}'


class Loader:
    def load_module(self, code_file):
        directory, filename = os.path.split(code_file)
        module = os.path.splitext(filename)[0]

        sys.path.append(directory)

        spec = importlib.util.spec_from_file_location(module, code_file)
        # print(spec)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)

        return module


class Runner:
    def __init__(self, qn_details: str):
        self.details = qn_details
        self.parse_details(self.details)
        self.code_file = self.parsed["filepath"]
        self.question_id = self.parsed["question_id"]
        self.question_settings = WORKING_DIRECTORY_ROOT + "/question_blueprints/" + self.question_id + "/qnconfig.json"

        self.run()

    def run(self):
        gtypes = gentypes.GenTypes(self.question_settings)
        input_output = gtypes.parse_contents()
        with open("runner.log", "a") as f:
            f.write(f"input_output: {str(input_output)}\n")
        function_name = gtypes.get_function_name()

        try:
            module = Loader().load_module(self.code_file)
        # TODO: Pass error to the client
        except Exception as _e:
            sys.stdout.write(URCodeErrorLOL().to_string())
            sys.exit(0)

        solution_instance = module.Solution()
        function = getattr(solution_instance, function_name)

        # ASSUMING INPUT MUST BE UNIQUE
        cases = {i[0]: None for i in input_output}

        for (input, output) in input_output:
            with open("runner.log", "a") as f:
                f.write(f"input: {input}, input_type: {str(type(input))} " + f"output: {str(output)} output type: {str(type(output))}" "\n")
            try:
                # print(input, type(input))
                result = function(input)
            except Exception as e:
                with open("runner.log", "a") as f:
                    f.write("input: " + str(type(input)) + str(function) + e +"\n")
                error = URCodeErrorLOL().to_string()
                sys.stdout.write(error)
                sys.exit(0)

            if (result is None) and output:
                with open("runner.log", "a") as f:
                    f.write("\n" + str(result) + ", " + str(output) + "\n")
                sys.stdout.write(URCodeDontReturnAnything().to_string())
                sys.exit(0)
            if result == output:
                cases[input] = Pass
            else:
                fail = Fail(output, result)
                sys.stdout.write(fail.to_string())
                sys.exit(0)

        # print(cases)
        sys.stdout.write(Pass().to_string())
        sys.exit(0)

    def parse_details(self, data: str):
        try:
            self.parsed = json.loads(data)
        except Exception as e:
            cooked = Cooked()
            sys.stdout.write(cooked)
            sys.exit(0)


if __name__ == "__main__":
    opts = parser.parse_args()
    details = opts.question_details
    Runner(details)
