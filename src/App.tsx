import type React from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useState, useRef, useCallback, useEffect } from "react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { ScrollArea } from "@/components/ui/scroll-area";
import { OllamaClient, type SimplifiedTool, type OllamaModelName } from "@/services/ollama";
import { PLAIN_SYSTEM_PROMPT } from "@/constants/prompts";
import { OLLAMA_MODELS } from "@/services/ollama";

import { ReadFile, WriteFile, ListAccessibleFiles } from "@/tools";
import { getWeather } from "@/tools/getWeather";
import { AvailableToolsCard } from "@/tool-info-pane";
import { CalledToolsCard, type EnhancedToolCalledUpdate } from "@/tool-call-history";

// Define types for interaction steps
type InteractionStepType = "model_message" | "tool_call" | "tool_result" | "error";

export interface InteractionStep {
  id: string;
  type: InteractionStepType;
  timestamp: number;
  content: string | Record<string, any>; // string for messages/errors, object for tool calls/results
  toolName?: string; // For tool_call and tool_result
}

const AVAILABLE_MODELS: OllamaModelName[] = Object.keys(OLLAMA_MODELS) as OllamaModelName[];
const DEFAULT_MODEL: OllamaModelName = "QWEN_3_14B";

function App() {
  // Replace output state with interactionHistory
  // const [output, setOutput] = useState("");
  const [interactionHistory, setInteractionHistory] = useState<InteractionStep[]>([]);
  const [toolsCalled, setToolsCalled] = useState<EnhancedToolCalledUpdate[]>([]);
  const [selectedModel, setSelectedModel] = useState<OllamaModelName>(DEFAULT_MODEL);
  // Use a ref to track if component is mounted
  const isMountedRef = useRef(true);

  // Store the client instance in a ref to prevent recreation on re-renders
  const ollamaClientRef = useRef<OllamaClient | null>(null);

  // Modified onToolUpdate to use timestamp + random string for truly unique IDs
  const onToolUpdate = useCallback((toolUpdate: { toolName: string; isPending: boolean }) => {
    if (!isMountedRef.current) return;

    setToolsCalled((prevTools) => {
      // Check if this is a new call (isPending: true) or an update to an existing call
      if (toolUpdate.isPending) {
        // Generate a truly unique ID using timestamp + random string
        const uniqueId = `${toolUpdate.toolName}-${Date.now()}-${Math.random()
          .toString(36)
          .substring(2, 9)}`;

        // This is a new tool call, create a new entry with a unique ID
        const newToolCall: EnhancedToolCalledUpdate = {
          id: uniqueId,
          toolName: toolUpdate.toolName,
          isPending: true,
          timestamp: Date.now(),
        };
        return [...prevTools, newToolCall];
      } else {
        // This is an update to the most recent pending call for this tool
        // Find the most recent pending call for this tool
        const pendingToolIndex = prevTools.findIndex(
          (tool) => tool.toolName === toolUpdate.toolName && tool.isPending
        );

        if (pendingToolIndex !== -1) {
          // Create a new array with the updated tool
          const updatedTools = [...prevTools];
          updatedTools[pendingToolIndex] = {
            ...updatedTools[pendingToolIndex],
            isPending: false,
          };
          return updatedTools;
        }

        return prevTools;
      }
    });
  }, []);

  const tools: SimplifiedTool[] = [ReadFile, WriteFile, ListAccessibleFiles, getWeather] as const;

  // Initialize/Re-initialize the client when the selected model changes
  useEffect(() => {
    console.log(`Initializing client with model: ${selectedModel}`);
    ollamaClientRef.current = new OllamaClient(
      selectedModel,
      PLAIN_SYSTEM_PROMPT,
      tools,
      onToolUpdate
    );
    // Optionally clear output/tools when model changes?
    // setOutput("");
    // setToolsCalled([]);
  }, [selectedModel, tools, onToolUpdate]); // Rerun effect if selectedModel changes

  const promptModel = async (prompt: string) => {
    if (!ollamaClientRef.current) {
      console.error("Ollama client not initialized.");
      return;
    }

    setInteractionHistory([]);
    setToolsCalled([]);

    try {
      const response = await ollamaClientRef.current.streamToolPrompt(prompt);

      for await (const part of response) {
        if (isMountedRef.current) {
          setInteractionHistory((prevHistory) => {
            const lastStep = prevHistory[prevHistory.length - 1];

            // Check if the last step is a model message and append
            if (lastStep && lastStep.type === "model_message") {
              const updatedHistory = [...prevHistory];
              updatedHistory[prevHistory.length - 1] = {
                ...lastStep,
                content: lastStep.content + part.message.content, // Append content
              };
              return updatedHistory;
            } else {
              // Otherwise, create a new model message step
              return [
                ...prevHistory,
                {
                  id: `${Date.now()}-${Math.random().toString(36).substring(2, 9)}`,
                  type: "model_message",
                  timestamp: Date.now(),
                  content: part.message.content,
                },
              ];
            }
          });
        } else {
          break; // Stop processing if component unmounted
        }
      }
    } catch (error) {
      console.error("Error streaming response:", error);
      if (isMountedRef.current) {
        setInteractionHistory((prevHistory) => [
          ...prevHistory,
          {
            id: `${Date.now()}-${Math.random().toString(36).substring(2, 9)}`,
            type: "error",
            timestamp: Date.now(),
            content: error instanceof Error ? error.message : String(error),
          },
        ]);
      }
    }
  };

  // Set up cleanup for component unmount
  useEffect(() => {
    // Set isMountedRef to true when the component mounts
    isMountedRef.current = true;
    // Return a cleanup function that sets isMountedRef to false when the component unmounts
    return () => {
      isMountedRef.current = false;
    };
  }, []); // Empty dependency array ensures this runs only on mount and unmount

  return (
    <div className="flex flex-col h-screen w-full p-4 bg-background">
      <div className="flex flex-1 gap-4 h-full">
        <div className="flex flex-col flex-1 gap-4">
          <ModelResponseCard interactionHistory={interactionHistory} />
          <PromptInput promptModel={promptModel} />
        </div>
        <div className="flex flex-col w-[20rem] gap-4">
          <ModelSelectorCard
            selectedModel={selectedModel}
            onModelChange={setSelectedModel}
            availableModels={AVAILABLE_MODELS}
          />
          <AvailableToolsCard availableTools={tools} />
          <CalledToolsCard calledTools={toolsCalled} />
        </div>
      </div>
    </div>
  );
}

interface ModelSelectorCardProps {
  selectedModel: OllamaModelName;
  onModelChange: (model: OllamaModelName) => void;
  availableModels: OllamaModelName[];
}

function ModelSelectorCard({
  selectedModel,
  onModelChange,
  availableModels,
}: ModelSelectorCardProps) {
  return (
    <Card className="border-border">
      <CardHeader className="pb-2">
        <CardTitle className="text-sm font-medium">Select Model</CardTitle>
      </CardHeader>
      <CardContent>
        <Select
          value={selectedModel}
          onValueChange={(value: string) => onModelChange(value as OllamaModelName)}
        >
          <SelectTrigger className="w-full">
            <SelectValue placeholder="Select a model" />
          </SelectTrigger>
          <SelectContent>
            {availableModels.map((model) => (
              <SelectItem key={model} value={model}>
                {model}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </CardContent>
    </Card>
  );
}

interface ModelResponseCardProps {
  interactionHistory: InteractionStep[];
}

function ModelResponseCard({ interactionHistory }: ModelResponseCardProps) {
  return (
    <Card className="flex-1 overflow-hidden">
      <CardHeader className="pb-2">
        <CardTitle className="text-sm font-medium">Interaction Log</CardTitle>
      </CardHeader>
      <CardContent className="h-[calc(100%-3rem)] p-0">
        <ScrollArea className="h-full w-full p-4">
          {interactionHistory.length > 0 ? (
            <div className="space-y-3">
              {interactionHistory.map((step) => (
                <div key={step.id} className="text-sm border-b pb-2 last:border-b-0">
                  <div className="text-xs text-muted-foreground mb-1">
                    {new Date(step.timestamp).toLocaleTimeString()} - {step.type}
                  </div>
                  {typeof step.content === "string" ? (
                    <div
                      className={
                        step.type === "error"
                          ? "text-red-500 whitespace-pre-wrap"
                          : "whitespace-pre-wrap"
                      }
                    >
                      {step.content}
                    </div>
                  ) : (
                    <pre className="text-xs bg-muted p-2 rounded-md overflow-x-auto">
                      <code>{JSON.stringify(step.content, null, 2)}</code>
                    </pre>
                  )}
                </div>
              ))}
            </div>
          ) : (
            <div className="text-muted-foreground h-full flex items-center justify-center">
              Model output will appear here
            </div>
          )}
        </ScrollArea>
      </CardContent>
    </Card>
  );
}

interface PromptInputProps {
  promptModel: (prompt: string) => void;
}

function PromptInput({ promptModel }: PromptInputProps) {
  const [input, setInput] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (input.trim()) {
      promptModel(input);
      setInput("");
    }
  };

  return (
    <form onSubmit={handleSubmit} className="flex gap-2 h-12">
      <Input
        value={input}
        placeholder="Enter your prompt here..."
        onChange={(e) => setInput(e.target.value)}
        className="flex-1"
      />
      <Button type="submit">Prompt</Button>
    </form>
  );
}

export default App;
