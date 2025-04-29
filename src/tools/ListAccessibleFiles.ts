import { SimplifiedTool } from "../services/ollama";
import { invoke } from "@tauri-apps/api/core";

const SAFE_FILE_DIR = "C:\\Users\\lucie\\Desktop\\Projects\\personal\\isotope\\src\\files_for_ai";
const NAME = "listDirectory";
const DESCRIPTION =
  "Lists all files available in the accessible directory. Returns an array of objects containing each file's name and absolute path.";

// Async function to get files through Tauri invoke
async function getFiles(): Promise<string> {
  try {
    const files = await invoke<Array<{ name: string; path: string }>>("list_files", {
      dir: SAFE_FILE_DIR,
    });

    // Example: join file names into a comma-separated string
    const finalResponse = files.map((file) => `${file.name}: ${file.path}`).join(", ");
    console.log(finalResponse);
    return finalResponse;
  } catch (error) {
    console.error("Error reading directory:", error);
    return "Error fetching files";
  }
}

// Export the tool with an async function
export const ListAccessibleFiles: SimplifiedTool = {
  name: NAME,
  description: DESCRIPTION,
  parameters: {},
  function: getFiles,
};
