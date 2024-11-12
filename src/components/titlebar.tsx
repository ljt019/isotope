import { appWindow } from "@tauri-apps/api/window";
import { Minus, Maximize2, X } from "lucide-react";
import { Button } from "@/components/ui/button";

export default function TitleBar() {
  const handleMinimize = () => appWindow.minimize();
  const handleMaximize = () => appWindow.toggleMaximize();
  const handleClose = () => appWindow.close();

  return (
    <div
      data-tauri-drag-region
      className="h-10 bg-secondary border-b flex justify-end items-center fixed top-0 left-0 right-0 z-50"
    >
      <Button
        variant="ghost"
        size="icon"
        onClick={handleMinimize}
        className="rounded-none h-10 w-10 text-muted-foreground hover:text-foreground"
        aria-label="Minimize"
      >
        <Minus className="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        onClick={handleMaximize}
        className="rounded-none h-10 w-10 text-muted-foreground hover:text-foreground"
        aria-label="Maximize"
      >
        <Maximize2 className="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        onClick={handleClose}
        className="rounded-none h-10 w-10 hover:bg-accent text-muted-foreground hover:text-destructive"
        aria-label="Close"
      >
        <X className="h-4 w-4" />
      </Button>
    </div>
  );
}
