import { useEffect } from "react";

import { UnlistenFn, listen } from "@tauri-apps/api/event";

interface UseListenProps {
  event: string;
  callback: (event: any) => void;
  deps?: any[];
}

export default function useListen({
  event,
  callback,
  deps = [],
}: UseListenProps) {
  useEffect(() => {
    let unlisten: UnlistenFn;

    const setupListener = async () => {
      unlisten = await listen(event, callback);
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, deps);
}
