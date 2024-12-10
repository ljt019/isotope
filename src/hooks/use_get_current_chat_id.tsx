import { invoke } from "@tauri-apps/api/tauri";

import { useQuery } from "@tanstack/react-query";

async function getCurrentChatId(): Promise<string> {
  return invoke<string>("get_current_chat_id");
}

export function useGetCurrentChatId() {
  return useQuery<string, Error>({
    queryKey: ["current-chat-id"],
    queryFn: getCurrentChatId,
  });
}
