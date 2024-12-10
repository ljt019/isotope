import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect, useRef } from "react";
import { Send, Loader2, User, Bot } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Card, CardContent } from "@/components/ui/card";
import ReactMarkdown from "react-markdown";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { oneDark } from "react-syntax-highlighter/dist/esm/styles/prism";
import { listen } from "@tauri-apps/api/event";
import { useGetCurrentChat } from "@/hooks/use_get_current_chat";

interface Message {
  role: "system" | "user" | "assistant";
  content: string;
}

export function Index() {
  const [prompt, setPrompt] = useState("");
  const [isGenerating, setIsGenerating] = useState(false);
  const [currentResponse, setCurrentResponse] = useState("");
  const scrollAreaRef = useRef<HTMLDivElement>(null);
  const { data: messages, refetch: refetchMessages } = useGetCurrentChat();

  useEffect(() => {
    if (scrollAreaRef.current) {
      scrollAreaRef.current.scrollTop = scrollAreaRef.current.scrollHeight;
    }
  }, [messages, currentResponse]);

  useEffect(() => {
    const unlistenToken = listen<string>("chat-token", (event) => {
      setCurrentResponse((prev) => {
        const newChar = event.payload;
        // If the new character is a space and the previous character is also a space,
        // replace the double space with a single space
        if (newChar === " " && prev.endsWith(" ")) {
          return prev;
        }
        return prev + newChar;
      });
    });

    type ChatEndPayload = {
      text: string;
      tokens: number;
      time: number;
      model: string;
    };

    const unlistenEnd = listen<ChatEndPayload>("chat-end", (event) => {
      console.log("Chat completed:", {
        text: event.payload.text,
        tokens: event.payload.tokens,
        time: event.payload.time,
        model: event.payload.model,
      });
      setIsGenerating(false);
      setCurrentResponse("");
      refetchMessages();
    });

    const unlistenError = listen("chat-error", (event) => {
      console.error("Chat Error:", event.payload);
      setIsGenerating(false);
      setCurrentResponse("");
    });

    return () => {
      unlistenToken.then((f) => f());
      unlistenEnd.then((f) => f());
      unlistenError.then((f) => f());
    };
  }, [refetchMessages]);

  const startChat = async () => {
    if (isGenerating || !prompt.trim()) return;
    setIsGenerating(true);
    setCurrentResponse("");

    try {
      await invoke("chat", { prompt: prompt });
      setPrompt("");
    } catch (error) {
      console.error("Invocation error:", error);
      setIsGenerating(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && e.ctrlKey) {
      e.preventDefault();
      startChat();
    }
  };

  const renderMarkdown = (content: string) => (
    <ReactMarkdown
      className="prose dark:prose-invert max-w-none"
      components={{
        code(props) {
          const { children, className, ...rest } = props;
          const match = /language-(\w+)/.exec(className || "");
          return match ? (
            <SyntaxHighlighter language={match[1]} style={oneDark} PreTag="div">
              {String(children).replace(/\n$/, "")}
            </SyntaxHighlighter>
          ) : (
            <code className={className} {...rest}>
              {children}
            </code>
          );
        },
      }}
    >
      {content}
    </ReactMarkdown>
  );

  const renderMessage = (message: Message, index: number) => (
    <div key={index} className="mb-4">
      <div className="flex items-start">
        <div className="flex-shrink-0 w-6 h-6 mr-2">
          {message.role === "user" ? (
            <User className="w-full h-full text-blue-500" />
          ) : (
            <Bot className="w-full h-full text-green-500" />
          )}
        </div>
        <div className="flex-grow">{renderMarkdown(message.content)}</div>
      </div>
    </div>
  );

  return (
    <div className="flex flex-col h-screen bg-background text-foreground container">
      <Card className="flex flex-col h-full rounded-none border-0">
        <CardContent className="flex-grow overflow-hidden p-6">
          <div className="grid grid-rows-[1fr,auto] gap-6 h-full">
            <ScrollArea className="pr-4" ref={scrollAreaRef}>
              {messages?.filter((m) => m.role !== "system").map(renderMessage)}
              {currentResponse && (
                <div className="mb-4">
                  <div className="flex items-start">
                    <div className="flex-shrink-0 w-6 h-6 mr-2">
                      <Bot className="w-full h-full text-green-500" />
                    </div>
                    <div className="flex-grow">
                      {renderMarkdown(currentResponse)}
                    </div>
                  </div>
                </div>
              )}
            </ScrollArea>
            <div className="space-y-2 lg:space-y-4">
              <Textarea
                value={prompt}
                onChange={(e) => setPrompt(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="Enter your prompt here..."
                className="min-h-[100px] resize-none"
              />
              <div className="flex justify-between items-center">
                <p className="text-sm text-muted-foreground">
                  Press Ctrl + Enter to send
                </p>
                <Button onClick={startChat} disabled={isGenerating}>
                  {isGenerating ? (
                    <>
                      <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                      Generating...
                    </>
                  ) : (
                    <>
                      <Send className="mr-2 h-4 w-4" />
                      Send
                    </>
                  )}
                </Button>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
