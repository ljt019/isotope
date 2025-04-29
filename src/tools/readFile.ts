import { SimplifiedTool } from "../services/ollama";
import { invoke } from "@tauri-apps/api/core";

const SAFE_FILE_DIR = "C:\\Users\\lucie\\Desktop\\Projects\\personal\\isotope\\src\\files_for_ai";
const NAME = "readFile";
const DESCRIPTION =
  "Reads the contents of a file from the authorized directory. For security reasons, only files within the authorized directory can be accessed. Use listdirectory tool first to get the path to files. Returns the file contents as a UTF-8 encoded string.";

// Async function to read a file via Tauri invoke
async function readFile(args: { filePath: string }): Promise<string> {
  try {
    // You might still want to do a quick client-side check
    const normalizedPath = args.filePath; // Replace with your own normalization if needed
    if (!normalizedPath.startsWith(SAFE_FILE_DIR)) {
      throw new Error("Access denied: Cannot read files outside the allowed directory");
    }
    const content = await invoke<string>("read_file", { filePath: args.filePath });
    return content;
  } catch (error) {
    console.error("Error reading file:", error);
    return "Error reading file";
  }
}

// Export the tool definition
export const ReadFile: SimplifiedTool = {
  name: NAME,
  description: DESCRIPTION,
  parameters: {
    type: "object",
    properties: {
      filePath: {
        type: "string",
        description:
          "The absolute path to the file you want to read. (use the listDirectory tool first).",
      },
    },
    required: ["filePath"],
  },
  function: readFile,
};
