import os
import json
import argparse


WORKING_DIRECTORY_ROOT = "/home/aad1sh/rust-programming/coapi-rs"

parser = argparse.ArgumentParser(description="Generate question default code and qnsettings.conf for question")
parser.add_argument("--details", required=True, help="store JSON details of questions in questions/<question-id>/ directory.")

# predefined while creation of question. <question-id>/main.py
# class Solution:
#     def factorial(n: int) -> int:

# <question-id>/qnsettings.conf
# {
#         "function_name": factorial,
#         "input_output": {
#             "5": 120,
#             "3": 6,
#         },
# }


# Execution
# code/py234qlkj/main.py
# class Solution:
#     def factorial(n: int) -> int:
#         ...

# Runner.py
# look into <question-id>/qnsettings.conf



# Supported Types:
#     int,
#     bool,
#     str,
#     List[str],
#     List[int],
#     List[float],
#     List[bool],

# Sample Input 1
# {
#         "question_id": id,
#         "function_name": "factorial",
#         "input_output": {
#             "5": 120,
#             "3": 6,
#         },
#         "input_type": "int",
#         "output_type": "int",
# }
#
# {
#         "question_id": id,
#         "function_name": "gen_range",
#         "input_output": {
#             "5": [1, 2, 3, 4, 5],
#             "3": [1, 2, 3],
#         },
#         "input_type": "int",
#         "output_type": "List[int]",
# }


class Generator:
    def __init__(self, details: str):
        self.details = details
        self.parse_details()
        self.question_id = self.parsed["question_id"]
        self.function_name = self.parsed["function_name"]
        self.input_output = self.parsed["input_output"]
        self.input_type = self.parsed["input_type"]
        self.output_type = self.parsed["output_type"]

        self.default_code_style = """# COIDE: https://github.com/aavtic/coapi.rs

class Solution:
    def {function_name}():
        # Write your code here..."""

        self.folder_path = f"{WORKING_DIRECTORY_ROOT}/question_blueprints/{self.question_id}/"

        self.generate()

    def generate(self):
        self.create_dir_qnid()
        formatted_code = self.default_code_style.format(function_name=self.function_name)
        code_file_path = self.folder_path + "main.py"
        self.create_write_file(formatted_code, code_file_path)
        config_path = self.folder_path + "qnconfig.json"
        self.create_write_file(self.details, config_path)

    def create_dir_qnid(self):
        try:
            os.mkdir(self.folder_path)
        except FileExistsError:
            pass

    def create_write_file(self, text: str, path: str):
        with open(path, "w") as f:
            f.write(text)

    def parse_details(self):
        try:
            self.parsed = json.loads(self.details)
        except Exception as e:
            raise e

if __name__ == "__main__":
    opts = parser.parse_args()
    details = opts.details
    Generator(details)
