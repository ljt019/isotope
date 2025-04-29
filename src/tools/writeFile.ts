import { SimplifiedTool } from "../services/ollama";
import { invoke } from "@tauri-apps/api/core";

const SAFE_FILE_DIR = "C:\\Users\\lucie\\Desktop\\Projects\\personal\\isotope\\src\\files_for_ai";
const NAME = "writeFile";
const DESCRIPTION =
  "Writes or overwrites a file in the authorized directory. For security reasons, only files within the authorized directory can be modified. Creates any necessary parent directories if they don't exist. Returns a confirmation message upon successful write.";

// Async function to write a file via Tauri invoke
async function writeFile(args: { filePath: string; fileContent: string }): Promise<string> {
  try {
    // Quick client-side security check
    const normalizedPath = args.filePath; // Optionally add more robust normalization here
    if (!normalizedPath.startsWith(SAFE_FILE_DIR)) {
      throw new Error("Access denied: Cannot write files outside the allowed directory");
    }
    const response = await invoke<string>("write_file", {
      filePath: args.filePath,
      fileContent: args.fileContent,
    });
    return response;
  } catch (error) {
    console.error("Error writing file:", error);
    return "Error writing file";
  }
}

// Export the tool definition
export const WriteFile: SimplifiedTool = {
  name: NAME,
  description: DESCRIPTION,
  parameters: {
    type: "object",
    properties: {
      filePath: {
        type: "string",
        description: "The absolute path to the file you want to write to. (use listDirectory).",
      },
      fileContent: {
        type: "string",
        description:
          "The content to write to the file. This will completely replace any existing content if the file already exists.",
      },
    },
    required: ["filePath", "fileContent"],
  },
  function: writeFile,
};
