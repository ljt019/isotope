import { invoke } from "@tauri-apps/api/tauri";
import { useQuery } from "@tanstack/react-query";

async function getSystemPrompt(): Promise<string> {
  return invoke<string>("get_system_prompt");
}

export function useGetSystemPrompt() {
  return useQuery<string, Error>({
    queryKey: ["system-prompt"],
    queryFn: getSystemPrompt,
  });
}
