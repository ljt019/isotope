import { appWindow } from "@tauri-apps/api/window";
import { Minus, Maximize, X } from "lucide-react";
import { Button } from "@/components/ui/button";

export default function TitleBar() {
  const handleMinimize = () => appWindow.minimize();
  const handleMaximize = () => appWindow.toggleMaximize();
  const handleClose = () => appWindow.close();

  return (
    <div
      data-tauri-drag-region
      className="h-8 bg-secondary border-b flex justify-end items-center fixed top-0 left-0 right-0 z-50"
    >
      <Button
        variant="ghost"
        size="icon"
        onClick={handleMinimize}
        className="h-[1.5rem] w-8 text-muted-foreground hover:text-foreground hover:bg-zinc-700 rounded-md"
        aria-label="Minimize"
      >
        <Minus className="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        onClick={handleMaximize}
        className="h-[1.5rem] w-8 text-muted-foreground hover:text-foreground hover:bg-zinc-700 rounded-md"
        aria-label="Maximize"
      >
        <Maximize className="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        onClick={handleClose}
        className="h-[1.5rem] w-8 hover:bg-accent text-muted-foreground hover:text-destructive hover:bg-zinc-700 rounded-md mr-1"
        aria-label="Close"
      >
        <X className="h-4 w-4" />
      </Button>
    </div>
  );
}
