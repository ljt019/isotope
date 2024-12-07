import { invoke } from "@tauri-apps/api/tauri";

import { useQuery } from "@tanstack/react-query";

interface ChatItem {
  id: string;
  name: string;
  created_at: string;
}

async function getChatHistory(): Promise<ChatItem[]> {
  return invoke<ChatItem[]>("get_chat_history");
}

export function useGetChatHistory() {
  return useQuery<ChatItem[], Error>({
    queryKey: ["chat-history"],
    queryFn: getChatHistory,
  });
}
