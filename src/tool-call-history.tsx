import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { CheckCircle, Clock } from "lucide-react";

// Enhanced ToolCalledUpdate with a unique ID
export interface EnhancedToolCalledUpdate {
  id: string;
  toolName: string;
  isPending: boolean;
  timestamp: number;
  // Optional additional properties
  args?: Record<string, any>;
  result?: any;
}

interface CalledToolsCardProps {
  calledTools: EnhancedToolCalledUpdate[];
}

export function CalledToolsCard({ calledTools }: CalledToolsCardProps) {
  return (
    <Card className="border-border flex-1">
      <CardHeader className="pb-2">
        <CardTitle className="text-sm font-medium">Called Tools</CardTitle>
      </CardHeader>
      <CardContent className="p-0 h-[calc(100%-3rem)]">
        <ScrollArea className="h-full w-full px-4 pb-4">
          {calledTools.length === 0 ? (
            <div className="flex h-full items-center justify-center text-muted-foreground">
              No tools called yet
            </div>
          ) : (
            <div className="space-y-2">
              {/* Sort by timestamp to show most recent calls first */}
              {calledTools
                .sort((a, b) => b.timestamp - a.timestamp)
                .map((tool) => (
                  <ToolCalledEntry key={tool.id} calledTool={tool} />
                ))}
            </div>
          )}
        </ScrollArea>
      </CardContent>
    </Card>
  );
}

interface ToolCalledEntryProps {
  calledTool: EnhancedToolCalledUpdate;
}

function ToolCalledEntry({ calledTool }: ToolCalledEntryProps) {
  const toolName = calledTool.toolName;
  const isPending = calledTool.isPending;

  return (
    <div className="flex items-center justify-between rounded-md border border-border bg-card p-2 text-card-foreground">
      <div className="flex items-center gap-2">
        {isPending ? (
          <Clock className="size-4 text-muted-foreground" />
        ) : (
          <CheckCircle className="size-4 text-primary" />
        )}
        <span className="text-sm font-medium">{toolName}</span>
      </div>
      <span className="text-xs text-muted-foreground">
        {new Date(calledTool.timestamp).toLocaleTimeString([], {
          hour: "2-digit",
          minute: "2-digit",
          second: "2-digit",
        })}
      </span>
    </div>
  );
}
