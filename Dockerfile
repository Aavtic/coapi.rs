# Use a lightweight base image with Python installed
FROM python:3.10-slim

# Create a non-root user for security
RUN useradd -m sandbox

# Set the working directory
WORKDIR /home/sandbox

# Copy the user's code into the container
COPY ./code/python-code/code.py /home/sandbox/code.py

# Set permissions so the sandbox user can access the files
RUN chown -R sandbox:sandbox /home/sandbox

# Switch to the non-root user
USER sandbox

# Execute the Python script
CMD ["python", "code.py"]
