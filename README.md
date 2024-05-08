# File Sorting Application

This is a simple file sorting application that allows you to organize files into different folders based on their file type. The application supports sorting of images, audio files, videos, documents, and archives.

## User Guide

1. **Installation**: You can download the application from GitHub and install it on your computer by following these instructions:
    - Clone the GitHub repository to your computer: `git clone https://github.com/KimLof/file-sorting-app.git`
    - Navigate into the application directory: `cd file-sorting-app`
    - Build the application: `cargo build --release`

2. **Launching**: Once the application is installed, you can launch it from the command line as follows:
    - Open a command prompt and navigate into the application directory.
    - Start the application with the command: `cargo run --release`

3. **Usage**:
    - First, provide the path to the directory you want to sort.
    - Select the file type(s) you want to sort. You can choose one or all.
    - Choose whether to process subdirectories. `Subdirectory processing not working as intended yet`
    - Optionally, you can choose to preview the found files.
    - Press the "Move Files" button to start the sorting process.

4. **Error Handling**: If the application encounters errors, it will display them on the command line and provide instructions for resolving the issue.

5. **Providing Feedback**: If you encounter any issues or have suggestions for improving the application, you can share your feedback on GitHub by opening a new [issue](https://github.com/KimLof/file-sorting-app/issues) or by sending an email to kim@kimcode.fi.

## Features
- Ability to sort files into different folders based on their file type.
- Support for sorting images, audio files, videos, documents, and archives.
- Process subdirectories if needed.
- Preview found files before starting the sorting process.

## TODO
- Subdirectory processing not working properly.
- Undo and redo functionality.
- Language switching.

### NOTE: THE APPLICATION IS NOT YET FINISHED ###

This README.md file will be continuously updated as the application is further developed. Additional features and improvements are coming in future versions. If you have any questions or need more information, don't hesitate to reach out!
