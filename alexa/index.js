const Alexa = require("ask-sdk-core");
const axios = require("axios");

const prompt = "Ti ascolto, figliolo";

const getPadrePioResponse = async (query) => {
  const payload = {
    model: "gpt-3.5-turbo",
    messages: [
      {
        role: "system",
        content:
          "Sei Padre Pio, noto anche come Pio da Pietrelcina. Il tuo vero nome è Francesco Forgione, nato il 25 maggio 1887 e morto il 23 settembre 1968. Sei stato un presbitero e mistico italiano. Nel 2002 sei stato proclamato santo da Giovanni Paolo II. Ti piace ringraziare gli utenti per le domande e pregare, mentre ti rifiuti di rispondere a chi ti bestemmia contro.",
      },
      {
        role: "user",
        content: `Rispondi a "${query}" considerando l'informazione data, in italiano come se fossi Padre Pio parlando in prima persona.`,
      },
    ],
    temperature: 0.2,
    max_tokens: 2000,
  };
  const response = await axios.post(
    "https://www.gianroberto.io",
    JSON.stringify(payload),
    {
      headers: {
        authority: "www.gianroberto.io",
        accept: "*/*",
        "accept-language": "en-GB,en;q=0.9",
        "cache-control": "no-cache",
        "content-type": "application/json",
        origin: "https://www.gianroberto.io",
        pragma: "no-cache",
        referer: "https://www.gianroberto.io/",
        "sec-ch-ua":
          '"Chromium";v="112", "Brave";v="112", "Not:A-Brand";v="99"',
        "sec-ch-ua-mobile": "?0",
        "sec-ch-ua-platform": '"macOS"',
        "sec-fetch-dest": "empty",
        "sec-fetch-mode": "cors",
        "sec-fetch-site": "same-origin",
        "sec-gpc": "1",
        "user-agent":
          "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36",
      },
      responseType: "json",
    }
  );

  return response.data.choices[0].message.content;
};

const LaunchRequestHandler = {
  canHandle(handlerInput) {
    return (
      Alexa.getRequestType(handlerInput.requestEnvelope) === "LaunchRequest"
    );
  },
  async handle(handlerInput) {
    const speakOutput = prompt;
    return handlerInput.responseBuilder
      .speak(speakOutput)
      .reprompt(speakOutput)
      .getResponse();
  },
};

const PadrePioIntentHandler = {
  canHandle(handlerInput) {
    return (
      Alexa.getRequestType(handlerInput.requestEnvelope) === "IntentRequest" &&
      Alexa.getIntentName(handlerInput.requestEnvelope) === "PadrePioIntent"
    );
  },
  async handle(handlerInput) {
    const userInput =
      handlerInput.requestEnvelope.request.intent.slots.userInput.value;
    const response = await getPadrePioResponse(userInput);
    return handlerInput.responseBuilder
      .speak(response)
      .withShouldEndSession(true)
      .getResponse();
  },
};

const ErrorHandler = {
  canHandle() {
    return true;
  },
  handle(handlerInput, error) {
    console.error(`Error: ${error.message}`);
    const speakOutput = `Error: ${error.message}`;
    return handlerInput.responseBuilder
      .speak(speakOutput)
      .reprompt(speakOutput)
      .getResponse();
  },
};

const HelpIntentHandler = {
  canHandle(handlerInput) {
    return (
      Alexa.getRequestType(handlerInput.requestEnvelope) === "IntentRequest" &&
      Alexa.getIntentName(handlerInput.requestEnvelope) === "AMAZON.HelpIntent"
    );
  },
  handle(handlerInput) {
    const speakOutput = "Puoi chiedere qualcosa! Come posso aiutare?";

    return handlerInput.responseBuilder
      .speak(speakOutput)
      .reprompt(speakOutput)
      .getResponse();
  },
};

const CancelAndStopIntentHandler = {
  canHandle(handlerInput) {
    return (
      Alexa.getRequestType(handlerInput.requestEnvelope) === "IntentRequest" &&
      (Alexa.getIntentName(handlerInput.requestEnvelope) ===
        "AMAZON.CancelIntent" ||
        Alexa.getIntentName(handlerInput.requestEnvelope) ===
          "AMAZON.StopIntent" ||
        Alexa.getIntentName(handlerInput.requestEnvelope) === "AMAZON.NoIntent")
    );
  },
  handle(handlerInput) {
    const speakOutput = "Mandi!";

    return handlerInput.responseBuilder.speak(speakOutput).getResponse();
  },
};

/**
 * This handler acts as the entry point for your skill, routing all request and response
 * payloads to the handlers above. Make sure any new handlers or interceptors you've
 * defined are included below. The order matters - they're processed top to bottom
 * */
exports.handler = Alexa.SkillBuilders.custom()
  .addRequestHandlers(
    LaunchRequestHandler,
    HelpIntentHandler,
    CancelAndStopIntentHandler,
    PadrePioIntentHandler
  )
  .addErrorHandlers(ErrorHandler)
  .withCustomUserAgent("sample/hello-world/v1.2")
  .lambda();
