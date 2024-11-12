# Isotope

Isotope is an elegant, native application that provides a sleek interface for interacting with the Llama language model. It offers a user-friendly environment for generating AI-powered responses, making it an ideal tool for developers, writers, and AI enthusiasts.

## Features

- **Intuitive Interface**: Clean and professional design for effortless interaction with the Llama model.
- **Real-time Response Generation**: Watch as the AI generates responses token by token.
- **Syntax Highlighting**: Beautifully formatted code snippets in the AI's responses.
- **Keyboard Shortcuts**: Quickly send prompts using Ctrl+Enter.
- **Cross-platform**: Built with Tauri, ensuring compatibility across multiple operating systems.
- **Dark Mode Support**: Seamlessly switch between light and dark themes.

## Installation

To install Isotope, follow these steps:

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/isotope.git
   cd isotope
   ```

2. Install dependencies:

   ```
   npm install
   ```

3. Build the application:

   ```
   npm run tauri build
   ```

4. The built application will be available in the `src-tauri/target/release` folder.

## Usage

1. Launch the Isotope application.
2. Enter your prompt in the text area at the bottom of the interface.
3. Click the "Generate Response" button or press Ctrl+Enter to send your prompt.
4. Watch as the AI generates its response in real-time in the main content area.
5. Scroll through the response and enjoy the formatted text and code snippets.

## Development

To set up the development environment:

1. Follow the installation steps above.
2. Instead of building, run the development server:
   ```
   npm run tauri dev
   ```

This will launch the application in development mode, allowing you to make changes and see them reflected in real-time.

---

For any questions or support, please open an issue on the GitHub repository.
