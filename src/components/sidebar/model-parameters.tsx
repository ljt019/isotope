import { Slider } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { debounce } from "lodash";
import { useGetSystemPrompt } from "@/hooks/use_get_system_prompt";

interface ModelParameter {
  name: string;
  min: number;
  max: number;
  step: number;
  defaultValue: number;
}

const modelParameters: ModelParameter[] = [
  { name: "Temperature", min: 0, max: 2, step: 0.1, defaultValue: 1 },
  { name: "Max Tokens", min: 50, max: 2048, step: 1, defaultValue: 1024 },
  { name: "Top P", min: 0, max: 1, step: 0.01, defaultValue: 0.9 },
  { name: "Repeat Penalty", min: 0, max: 2, step: 0.1, defaultValue: 1 },
];

export function ModelParameters() {
  const {
    data: initialSystemPrompt,
    isLoading,
    isError,
    error,
  } = useGetSystemPrompt();

  const [systemPrompt, setSystemPrompt] = useState<string>("");

  useEffect(() => {
    if (initialSystemPrompt) {
      setSystemPrompt(initialSystemPrompt);
    }
  }, [initialSystemPrompt]);

  const debouncedChangeSystemPrompt = useCallback(
    debounce((prompt: string) => {
      invoke("change_system_prompt", { systemPrompt: prompt });
    }, 300),
    []
  );

  const handleSystemPromptChange = (
    e: React.ChangeEvent<HTMLTextAreaElement>
  ) => {
    const newValue = e.target.value;
    setSystemPrompt(newValue);
    debouncedChangeSystemPrompt(newValue);
  };

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (isError) {
    return <div>Error: {error?.message}</div>;
  }

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <Label htmlFor="systemPrompt">System Prompt</Label>
        <Textarea
          id="systemPrompt"
          placeholder="Enter system prompt here..."
          className="min-h-[100px]"
          value={systemPrompt}
          onChange={handleSystemPromptChange}
        />
      </div>
      {modelParameters.map((param) => (
        <div key={param.name} className="space-y-2">
          <div className="flex items-center justify-between">
            <Label htmlFor={param.name}>{param.name}</Label>
            <span className="text-sm text-muted-foreground">
              {param.defaultValue}
            </span>
          </div>
          <Slider
            id={param.name}
            min={param.min}
            max={param.max}
            step={param.step}
            defaultValue={[param.defaultValue]}
            className="w-full"
            disabled={true}
          />
        </div>
      ))}
    </div>
  );
}
