import { Slider } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { debounce } from "lodash";
import { useGetSystemPrompt } from "@/hooks/use_get_system_prompt";
import { useGetCurrentInferenceParams } from "../../hooks/use_get_current_inference_params";

export interface ModelParametersPayload {
  temperature: number;
  maxTokens: number;
  topP: number;
  repeatPenalty: number;
}

interface ModelParameter {
  name: string;
  key: keyof ModelParametersPayload;
  min: number;
  max: number;
  step: number;
  defaultValue: number;
}

const modelParameters: ModelParameter[] = [
  {
    name: "Temperature",
    key: "temperature",
    min: 0,
    max: 2,
    step: 0.1,
    defaultValue: 1,
  },
  {
    name: "Max Tokens",
    key: "maxTokens",
    min: 50,
    max: 2048,
    step: 1,
    defaultValue: 1024,
  },
  {
    name: "Top P",
    key: "topP",
    min: 0,
    max: 1,
    step: 0.01,
    defaultValue: 0.9,
  },
  {
    name: "Repeat Penalty",
    key: "repeatPenalty",
    min: 0,
    max: 2,
    step: 0.1,
    defaultValue: 1,
  },
];

export function ModelParameters() {
  const {
    data: initialSystemPrompt,
    isLoading,
    isError,
    error,
  } = useGetSystemPrompt();

  const {
    data: currentParams,
    isLoading: isLoadingCurrentParams,
    isError: isErrorCurrentParams,
  } = useGetCurrentInferenceParams();

  const [systemPrompt, setSystemPrompt] = useState<string>("");

  // Use defaults initially, will override once currentParams are loaded
  const [params, setParams] = useState<ModelParametersPayload>();

  // Track if we've initialized from currentParams once
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    if (
      currentParams &&
      !isLoadingCurrentParams &&
      !isErrorCurrentParams &&
      !initialized
    ) {
      setParams(currentParams);
      setInitialized(true);
    }
  }, [
    currentParams,
    isLoadingCurrentParams,
    isErrorCurrentParams,
    initialized,
  ]);

  // Debounced function to send parameters to backend
  const debouncedModelParameters = useCallback(
    debounce((updatedParams: ModelParametersPayload) => {
      invoke("set_inference_params", { params: updatedParams });
    }, 300),
    []
  );

  const debouncedChangeSystemPrompt = useCallback(
    debounce((prompt: string) => {
      invoke("change_system_prompt", { systemPrompt: prompt });
    }, 300),
    []
  );

  // Effect to set system prompt once loaded
  useEffect(() => {
    if (initialSystemPrompt) {
      setSystemPrompt(initialSystemPrompt);
    }
  }, [initialSystemPrompt]);

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

  if (isError || !params) {
    return <div>Error: {error?.message}</div>;
  }

  // Handle parameter slider changes
  const handleParamChange = (
    key: keyof ModelParametersPayload,
    value: number[]
  ) => {
    const newParams = { ...params, [key]: value[0] };
    setParams(newParams);
    debouncedModelParameters(newParams);
  };

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
              {params[param.key]}
            </span>
          </div>
          <Slider
            id={param.name}
            min={param.min}
            max={param.max}
            step={param.step}
            value={[params[param.key]]}
            onValueChange={(value) => handleParamChange(param.key, value)}
            className="w-full"
          />
        </div>
      ))}
    </div>
  );
}
