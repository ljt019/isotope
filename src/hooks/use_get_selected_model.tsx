import { invoke } from "@tauri-apps/api/tauri";

import { useQuery } from "@tanstack/react-query";

function getSelectedModel(): Promise<string> {
  let options = invoke("get_selected_model");

  return options as Promise<string>;
}

export function useGetSelectedModel() {
  return useQuery({
    queryKey: ["selected-model"],
    queryFn: getSelectedModel,
  });
}
