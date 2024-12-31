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
    pass


class Pass:
    pass


class Fail:
    def __init__(self, expected, got):
        self.expected = expected
        self.got = got


class Runner:
    def __init__(self, qn_details: str):
        self.details = qn_details
        self.parse_details(self.details)
        self.code_file = self.parsed["filepath"]
        self.question_id = self.parsed["question_id"]
        self.function_name = self.parsed["function_name"]
        self.question_settings = WORKING_DIRECTORY_ROOT + "/question_blueprints/" + self.question_id + "/qnconfig.json"

        self.run()

    def run(self):
        gtypes = gentypes.GenTypes(self.question_settings)
        input_output = gtypes.parse_contents()
        function_name = gtypes.get_function_name()

        directory, filename = os.path.split(self.code_file)
        module = os.path.splitext(filename)

        sys.path.append(directory)

        spec = importlib.util.spec_from_file_location(module, self.code_file)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)

        solution_instance = module.Solution()
        function = getattr(solution_instance, self.function_name)

        no_test_cases = len(input_output)

        # ASSUMING INPUT MUST BE UNIQUE
        cases = {i: None for i in input_output}

        for (input, output) in input_output:
            try:
                result = function(input)
            except:
                raise URCodeErrorLOL

            if result == output:
                cases[input] = Pass
            else:
                fail = Fail(output, result)
                raise Fail


    def parse_details(self, data: str):
        try:
            self.parsed = json.loads(data)
        except Exception as e:
            raise e


if __name__ == "__main__":
    opts = parser.parse_args()
    details = opts.details
    Runner(details)
