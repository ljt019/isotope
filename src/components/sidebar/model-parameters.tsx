import { Slider } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";

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
  return (
    <div className="space-y-4">
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
