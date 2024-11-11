import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useState, useEffect } from "react";
import { Send, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { ScrollArea } from "@/components/ui/scroll-area";
import ReactMarkdown from "react-markdown";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { gruvboxDark } from "react-syntax-highlighter/dist/esm/styles/prism";

export function Index() {
  const [prompt, setPrompt] = useState("");
  const [response, setResponse] = useState("");
  const [isGenerating, setIsGenerating] = useState(false);

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
    <div className="flex flex-col h-screen bg-background text-foreground">
      <header className="flex-none p-4 border-b">
        <h1 className="text-xl font-semibold text-center">
          Llama Prompt Interface
        </h1>
      </header>
      <main className="flex-grow overflow-hidden flex flex-col p-4 space-y-4">
        <div className="flex-none">
          <label htmlFor="prompt" className="block text-sm font-medium mb-2">
            Your Prompt:
          </label>
          <Textarea
            id="prompt"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder="Enter your prompt here..."
            className="w-full h-32 resize-none"
          />
          <div className="mt-2 flex justify-end">
            <Button onClick={startChat} disabled={isGenerating}>
              {isGenerating ? (
                <>
                  <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  Generating...
                </>
              ) : (
                <>
                  <Send className="h-4 w-4 mr-2" />
                  Generate Response
                </>
              )}
            </Button>
          </div>
        </div>
        {response && (
          <div className="flex-grow overflow-hidden flex flex-col">
            <label className="block text-sm font-medium mb-2">
              Llama Response:
            </label>
            <ScrollArea className="flex-grow border rounded-md bg-muted p-4">
              <ReactMarkdown
                className="prose dark:prose-invert max-w-none"
                components={{
                  code(props) {
                    const { children, className, ...rest } = props;
                    const match = /language-(\w+)/.exec(className || "");
                    return match ? (
                      <SyntaxHighlighter
                        language={match[1]}
                        style={gruvboxDark}
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
            </ScrollArea>
          </div>
        )}
      </main>
    </div>
  );
}
