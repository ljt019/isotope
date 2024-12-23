import { ChevronDown, MessageCirclePlus } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { invoke } from "@tauri-apps/api/tauri";
import { useGetModelOptions } from "@/hooks/use_get_model_options";
import { useGetSelectedModel } from "@/hooks/use_get_selected_model";
import { Separator } from "@/components/ui/separator";
import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarInset,
  SidebarProvider,
} from "@/components/ui/sidebar";
import { ModelParameters } from "./model-parameters";
import { ChatHistory } from "./chat-history";
import { useGetChatHistory } from "@/hooks/use_get_chat_history";
import { useGetCurrentChat } from "@/hooks/use_get_current_chat";
import { useGetCurrentChatId } from "@/hooks/use_get_current_chat_id";
import { useState } from "react";

export default function AppSidebar({
  children,
}: {
  children: React.ReactNode;
}) {
  const { refetch } = useGetChatHistory();
  const { refetch: refetchCurrentChat } = useGetCurrentChat();
  const { refetch: refetchCurrentChatId } = useGetCurrentChatId();

  const new_chat = async () => {
    await invoke("new_chat");
    refetch();
    refetchCurrentChat();
    refetchCurrentChatId();
  };

  const [modelFetching, setModelFetching] = useState<boolean>(false);

  return (
    <SidebarProvider>
      <Sidebar className="w-[300px]">
        <SidebarHeader className="flex h-[60px] px-4 py-4">
          <div className="flex justify-between items-center w-full">
            <div className="flex items-center">
              <img
                src="/ollama.svg"
                alt="ollama"
                className="h-8 w-8 mr-2 [filter:brightness(0)_invert(1)]"
              />
              <SelectedModelTitle modelFetching={modelFetching} />
            </div>
            <SelectModel setModelFetching={setModelFetching} />
          </div>
        </SidebarHeader>
        <SidebarContent className="flex flex-col">
          <div className="flex-none p-4">
            <ModelParameters />
          </div>
          <Separator className="my-4" />
          <div className="flex-grow overflow-hidden flex flex-col">
            <div className="flex justify-between">
              <h2 className="mb-2 text-lg font-semibold px-4 py-2 bg-background sticky top-0">
                Chat History
              </h2>
              <Button variant="ghost" onClick={new_chat}>
                <MessageCirclePlus />
              </Button>
            </div>
            <div className="overflow-auto flex-grow px-4">
              <ChatHistory />
            </div>
          </div>
        </SidebarContent>
      </Sidebar>
      <SidebarInset>{children}</SidebarInset>
    </SidebarProvider>
  );
}

function SelectedModelTitle({ modelFetching }: { modelFetching: boolean }) {
  const { data: selected_model, isLoading, isError } = useGetSelectedModel();

  if (isLoading || modelFetching) return <div>Loading...</div>;

  if (isError) return <div>Error getting model options</div>;

  if (!selected_model) return null;

  return <div className="text-sm font-medium">{selected_model}</div>;
}

function SelectModel({
  setModelFetching,
}: {
  setModelFetching: (value: boolean) => void;
}) {
  const { data: modelOptions, isLoading, isError } = useGetModelOptions();
  const { data: selectedModel, refetch: refetchSelectedModel } =
    useGetSelectedModel();

  async function setOption(modelName: string) {
    try {
      console.log("Setting model:", modelName);
      setModelFetching(true);
      await invoke("set_model", { modelSelection: modelName });
      setModelFetching(false);
      await refetchSelectedModel();
    } catch (error) {
      console.error("Error setting model:", error);
    }
  }

  if (isLoading)
    return <div className="text-sm text-muted-foreground">Loading...</div>;

  if (isError)
    return (
      <div className="text-sm text-destructive">
        Error getting model options
      </div>
    );

  if (!modelOptions) return null;

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" size="sm">
          <ChevronDown className="h-4 w-4" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent>
        {modelOptions.map((model) => (
          <DropdownMenuItem
            key={model}
            onSelect={() => setOption(model)}
            className={selectedModel === model ? "bg-accent" : ""}
          >
            {model}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
