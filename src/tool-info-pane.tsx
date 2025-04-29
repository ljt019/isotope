import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Tooltip, TooltipProvider, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { Wrench, Info } from "lucide-react";
import { type SimplifiedTool } from "@/services/ollama";

interface AvailableToolsCardProps {
  availableTools: SimplifiedTool[];
}

export function AvailableToolsCard({ availableTools }: AvailableToolsCardProps) {
  return (
    <Card className="border-border flex-1">
      <CardHeader className="pb-2">
        <CardTitle className="text-sm font-medium">Available Tools</CardTitle>
      </CardHeader>
      <CardContent className="p-0 h-[calc(100%-3rem)]">
        <ScrollArea className="h-full w-full px-4 pb-4">
          {availableTools.length === 0 ? (
            <div className="flex h-full items-center justify-center text-muted-foreground">
              No tools available
            </div>
          ) : (
            <div className="space-y-2">
              {availableTools.map((tool, index) => (
                <ToolEntry key={`tool-${tool.name}-${index}`} tool={tool} />
              ))}
            </div>
          )}
        </ScrollArea>
      </CardContent>
    </Card>
  );
}

interface ToolEntryProps {
  tool: SimplifiedTool;
}

function ToolEntry({ tool }: ToolEntryProps) {
  return (
    <div className="flex items-center justify-between rounded-md border border-border bg-card p-2 text-card-foreground">
      <div className="flex items-center gap-2">
        <Wrench className="size-4 text-primary" />
        <span className="text-sm font-medium truncate max-w-[8rem]">{tool.name}</span>
      </div>
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <Info className="size-3.5 text-muted-foreground hover:text-foreground cursor-help" />
          </TooltipTrigger>
          <TooltipContent className="max-w-[15rem]">
            <p className="text-xs">{tool.description}</p>
            <div className="mt-1 text-xs text-muted-foreground">
              <p>Parameters: {Object.keys(tool.parameters).length}</p>
            </div>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </div>
  );
}
