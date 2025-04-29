import ollama, { Tool, Message, AbortableAsyncIterator, ChatResponse } from "ollama";

export const OLLAMA_MODELS = {
  QWEN_3_0_6B: "qwen3:0.6b",
  QWEN_3_1_7B: "qwen3:1.7b",
  QWEN_3_4B: "qwen3:4b",
  QWEN_3_8B: "qwen3:8b",
  QWEN_3_14B: "qwen3:14b",
  QWEN_3_30B3A: "qwen3:30b",
  COGITO_3B: "cogito:3b",
  COGITO_14B: "cogito:14b",
  LLAMA_3_8B_GROQ: "llama3-groq-tool-use:latest",
} as const;

export type OllamaModelName = keyof typeof OLLAMA_MODELS;

export interface SimplifiedTool {
  name: string;
  description: string;
  parameters: Record<string, any>;
  function: (args: any) => any;
}

export interface ToolCalledUpdate {
  toolName: string;
  isPending: boolean;
}

export interface StreamUpdate {
  newStream: AbortableAsyncIterator<ChatResponse>;
}

export class OllamaClient {
  system_prompt: string;
  model: string;
  api_url: string;
  tools: SimplifiedTool[];
  onToolCalled: (toolCalledUpdate: ToolCalledUpdate) => void;

  constructor(
    modelName: OllamaModelName,
    system_prompt: string,
    tools: SimplifiedTool[] = [],
    onToolCalled: (toolCalledUpdate: ToolCalledUpdate) => void,
    api_url: string = "http://localhost:11434"
  ) {
    this.system_prompt = system_prompt;
    this.model = OLLAMA_MODELS[modelName];
    this.api_url = api_url;
    this.tools = tools;
    this.onToolCalled = onToolCalled;
  }

  // Convert SimplifiedTool array to Tool array expected by Ollama
  private getOllamaTools(): Tool[] {
    return this.tools.map((tool) => ({
      type: "function",
      function: {
        name: tool.name,
        description: tool.description,
        parameters: {
          type: "object",
          required: tool.parameters.required,
          properties: tool.parameters.properties,
        },
      },
    }));
  }

  // Find the matching SimplifiedTool by name
  private findToolByName(name: string): SimplifiedTool | undefined {
    return this.tools.find((tool) => tool.name === name);
  }

  // Updated: Execute a tool call and await if the tool function returns a promise.
  private async executeToolCall(toolCall: any): Promise<string> {
    try {
      const tool = this.findToolByName(toolCall.function.name);
      if (!tool) {
        return `Tool "${toolCall.function.name}" not found`;
      }

      const args =
        typeof toolCall.function.arguments === "string"
          ? JSON.parse(toolCall.function.arguments)
          : toolCall.function.arguments;

      const result = tool.function(args);
      const resolvedResult = result instanceof Promise ? await result : result;
      return resolvedResult ? resolvedResult.toString() : "Execution completed with no output";
    } catch (error) {
      return `Error executing tool: ${error instanceof Error ? error.message : String(error)}`;
    }
  }

  async prompt(prompt: string) {
    const response = await ollama.chat({
      model: this.model,
      messages: [
        { role: "system", content: this.system_prompt },
        { role: "user", content: prompt },
      ],
    });
    return response;
  }

  async streamPrompt(prompt: string) {
    const response = await ollama.chat({
      model: this.model,
      messages: [
        { role: "system", content: this.system_prompt },
        { role: "user", content: prompt },
      ],
      stream: true,
    });
    return response;
  }

  async toolPrompt(prompt: string, maxIterations: number = 5) {
    const messages: Message[] = [
      { role: "system", content: this.system_prompt },
      { role: "user", content: prompt },
    ];

    let hasToolCalls = true;
    let iterationCount = 0;
    let finalResponse: any;

    // Continue processing as long as the model wants to use tools and we haven't exceeded max iterations
    while (hasToolCalls && iterationCount < maxIterations) {
      iterationCount++;

      const response = await ollama.chat({
        model: this.model,
        messages: messages,
        tools: this.getOllamaTools(),
      });

      if (response.message.tool_calls && response.message.tool_calls.length > 0) {
        for (const toolCall of response.message.tool_calls) {
          this.onToolCalled({ toolName: toolCall.function.name, isPending: true });
          const output = await this.executeToolCall(toolCall);
          this.onToolCalled({ toolName: toolCall.function.name, isPending: false });

          messages.push({
            role: "tool",
            content: output,
          });
        }
        finalResponse = null;
      } else {
        hasToolCalls = false;
        finalResponse = response;
      }
    }

    if (!finalResponse) {
      finalResponse = await ollama.chat({
        model: this.model,
        messages: messages,
      });
    }

    return finalResponse;
  }

  async streamToolPrompt(prompt: string, maxIterations: number = 5) {
    const messages: Message[] = [
      { role: "system", content: this.system_prompt },
      { role: "user", content: prompt },
    ];

    let hasToolCalls = true;
    let iterationCount = 0;

    while (hasToolCalls && iterationCount < maxIterations) {
      iterationCount++;

      const response = await ollama.chat({
        model: this.model,
        messages: messages,
        tools: this.getOllamaTools(),
      });

      if (response.message.tool_calls && response.message.tool_calls.length > 0) {
        // Add the model's response with tool calls to the messages
        messages.push(response.message);

        for (const toolCall of response.message.tool_calls) {
          this.onToolCalled({ toolName: toolCall.function.name, isPending: true });
          const output = await this.executeToolCall(toolCall);
          this.onToolCalled({ toolName: toolCall.function.name, isPending: false });

          messages.push({
            role: "tool",
            content: output,
          });
        }
      } else {
        hasToolCalls = false;
      }
    }

    const finalMessages = [
      {
        role: "assistant",
        content: `Alright, i've used the tools above based on the humans original query, I should now share my insights and findings based on these tools in a thoughtful, complete way that directly addresses the human's original question. If the tool failed I should let the user know instead of making up information, or trying a new way without the humans feedback. The next token I output will be seen by the user so I should respond as such: `,
      },
    ];

    messages.push(...finalMessages);

    const finalResponse = await ollama.chat({
      model: this.model,
      messages: messages,
      stream: true,
    });

    console.log(messages);

    return finalResponse;
  }
}
