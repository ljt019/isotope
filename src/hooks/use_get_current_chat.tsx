import { invoke } from "@tauri-apps/api/tauri";

import { useQuery } from "@tanstack/react-query";

interface Message {
  role: "system" | "user" | "assistant";
  content: string;
}

async function getCurrentChat(): Promise<Message[]> {
  return invoke<Message[]>("get_current_chat");
}

export function useGetCurrentChat() {
  return useQuery<Message[], Error>({
    queryKey: ["current-chat"],
    queryFn: getCurrentChat,
  });
}
