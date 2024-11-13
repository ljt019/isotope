import { invoke } from "@tauri-apps/api/tauri";

import { useQuery } from "@tanstack/react-query";

function getModelOptions(): Promise<string[]> {
  let options = invoke("get_model_options");

  return options as Promise<string[]>;
}

export function useGetModelOptions() {
  return useQuery({
    queryKey: ["model-options"],
    queryFn: getModelOptions,
  });
}
