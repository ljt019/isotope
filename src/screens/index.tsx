import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useState, useEffect, useRef } from "react";
import { Send, Loader2, ChevronDown } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import ReactMarkdown from "react-markdown";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { oneDark } from "react-syntax-highlighter/dist/esm/styles/prism";

export function Index() {
  const [prompt, setPrompt] = useState("");
  const [response, setResponse] = useState("");
  const [isGenerating, setIsGenerating] = useState(false);
  const responseRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const unlistenToken = listen("chat-token", (event) => {
      setResponse((prev) => prev + event.payload);
    });

    const unlistenEnd = listen("chat-end", () => {
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

  useEffect(() => {
    if (!isGenerating && response) {
      console.log("Raw model response:", response);
    }
  }, [isGenerating, response]);

  useEffect(() => {
    if (responseRef.current) {
      responseRef.current.scrollTop = responseRef.current.scrollHeight;
    }
  }, [response]);

  const startChat = async () => {
    if (isGenerating || !prompt.trim()) return;
    setIsGenerating(true);
    setResponse("");

    try {
      await invoke("chat", { message: prompt });
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
    <div className="flex flex-col h-screen bg-background text-foreground pb-3">
      <Card className="flex flex-col h-full rounded-none border-0">
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-4">
          <CardTitle className="text-2xl font-bold">
            <div className="flex">
              <img
                src="/ollama.svg"
                alt="ollama"
                className="h-8 w-8 mr-2 [filter:brightness(0)_invert(1)]"
              />
              <div>Llama-3.2-1B-Instruct</div>
            </div>
          </CardTitle>
          <Button variant="ghost" size="icon">
            <ChevronDown className="h-4 w-4" />
          </Button>
        </CardHeader>
        <Separator />
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
