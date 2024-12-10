import { invoke } from "@tauri-apps/api/tauri";

import { useQuery } from "@tanstack/react-query";

import { ModelParametersPayload } from "@/components/sidebar/model-parameters";

async function getCurrentInferenceParams(): Promise<ModelParametersPayload> {
  return invoke<ModelParametersPayload>("get_current_inference_params");
}

export function useGetCurrentInferenceParams() {
  return useQuery<ModelParametersPayload, Error>({
    queryKey: ["inference-params"],
    queryFn: getCurrentInferenceParams,
  });
}
