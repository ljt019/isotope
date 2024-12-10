import { MessageCircle, Loader2, AlertCircle } from "lucide-react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { useGetChatHistory } from "@/hooks/use_get_chat_history";
import { useGetCurrentChat } from "@/hooks/use_get_current_chat";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { invoke } from "@tauri-apps/api/tauri";

export function ChatHistory() {
  const { data: chatHistory, isLoading, isError, error } = useGetChatHistory();
  const { refetch } = useGetCurrentChat();

  const setCurrentChat = (chatId: string) => {
    invoke("set_current_chat", { chatId });
    refetch();
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <Loader2 className="h-6 w-6 animate-spin text-primary" />
        <span className="ml-2 text-sm text-muted-foreground">
          Loading chat history...
        </span>
      </div>
    );
  }

  if (isError) {
    return (
      <Alert variant="destructive">
        <AlertCircle className="h-4 w-4" />
        <AlertTitle>Error</AlertTitle>
        <AlertDescription>
          {error instanceof Error
            ? error.message
            : "Failed to load chat history"}
        </AlertDescription>
      </Alert>
    );
  }

  if (!chatHistory || chatHistory.length === 0) {
    return (
      <div className="text-center text-muted-foreground p-4">
        No chat history available.
      </div>
    );
  }

  return (
    <ScrollArea className="h-full">
      <div className="space-y-2 pr-4">
        {chatHistory.map((chat) => (
          <button
            key={chat.id}
            onClick={() => setCurrentChat(chat.id)}
            className="flex items-center space-x-2 rounded-md p-2 hover:bg-accent hover:text-accent-foreground w-full text-left"
          >
            <MessageCircle className="h-4 w-4" />
            <div className="flex-1 overflow-hidden">
              <p className="truncate text-sm font-medium">{chat.name}</p>
              <p className="text-xs text-muted-foreground">
                {new Date(chat.created_at).toLocaleString(undefined, {
                  year: "numeric",
                  month: "short",
                  day: "numeric",
                  hour: "2-digit",
                  minute: "2-digit",
                })}
              </p>
            </div>
          </button>
        ))}
      </div>
    </ScrollArea>
  );
}
