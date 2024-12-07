import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect, useRef } from "react";
import { Send, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Card, CardContent } from "@/components/ui/card";
import ReactMarkdown from "react-markdown";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { oneDark } from "react-syntax-highlighter/dist/esm/styles/prism";

import { listen } from "@tauri-apps/api/event";

export function Index() {
  const [prompt, setPrompt] = useState("");
  const [response, setResponse] = useState("");
  const [isGenerating, setIsGenerating] = useState(false);
  const responseRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (responseRef.current) {
      responseRef.current.scrollTop = responseRef.current.scrollHeight;
    }
  }, [response]);

  useEffect(() => {
    const unlistenToken = listen("chat-token", (event) => {
      setResponse((prev) => prev + event.payload);
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
    });

    const unlistenError = listen("chat-error", (event) => {
      console.error("Chat Error:", event.payload);
      setIsGenerating(false);
    });

    return () => {
      unlistenToken.then((f) => f());
      unlistenEnd.then((f) => f());
      unlistenError.then((f) => f());
    };
  }, []);

  const startChat = async () => {
    if (isGenerating || !prompt.trim()) return;
    setIsGenerating(true);
    setResponse("");

    try {
      await invoke("chat", { prompt: prompt });
      console.log(await invoke("get_current_chat"));
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

  return (
    <div className="flex flex-col h-screen bg-background text-foreground container">
      <Card className="flex flex-col h-full rounded-none border-0 ">
        <CardContent className="flex-grow overflow-hidden p-6">
          <div className="grid grid-rows-[1fr,auto] gap-6 h-full">
            <ScrollArea className="pr-4" ref={responseRef}>
              {response && (
                <ReactMarkdown
                  className="prose dark:prose-invert max-w-none"
                  components={{
                    code(props) {
                      const { children, className, ...rest } = props;
                      const match = /language-(\w+)/.exec(className || "");
                      return match ? (
                        <SyntaxHighlighter
                          language={match[1]}
                          style={oneDark}
                          PreTag="div"
                        >
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
                  {response}
                </ReactMarkdown>
              )}
            </ScrollArea>
            <div className="space-y-2 lg:space-y-4">
              <Textarea
                value={prompt}
                onChange={(e) => setPrompt(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="Enter your prompt here..."
                className="min-h-[100px] resize-none "
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
                      Generate Response
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

export default Index;
