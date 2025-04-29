import { type SimplifiedTool } from "@/services/ollama";

export const getWeather: SimplifiedTool = {
  name: "getWeather",
  description: "Gets the current temperature and humidity for a given city and country.",
  parameters: {
    type: "object",
    properties: {
      city: {
        type: "string",
        description: "The city name",
      },
      country: {
        type: "string",
        description: "The country name",
      },
    },
    required: ["city", "country"],
  },
  function: async (args: { city: string; country: string }) => {
    console.log(`Tool 'getWeather' called with args:`, args);
    // Simulate API call delay
    await new Promise((resolve) => setTimeout(resolve, 500));

    // Return dummy data
    const dummyData = {
      temperature: Math.floor(Math.random() * 30) + 5, // Random temp between 5 and 35 C
      humidity: Math.floor(Math.random() * 50) + 40, // Random humidity between 40% and 90%
      unit: "Celsius",
      location: `${args.city}, ${args.country}`,
    };

    // The function should likely return the result object directly, not stringified JSON
    // The client might handle the stringification if needed for the prompt
    return dummyData;
  },
};
