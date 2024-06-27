async function recognize(base64, lang, options) {
  const { config, utils } = options;
  const { tauriFetch: fetch } = utils;
  const { api_key } = config;

  if (!api_key) {
      throw new Error("API key not found!");
  }

  const requestData = {
      requests: [
          {
              image: { content: base64 },
              features: { type: "TEXT_DETECTION" }
          }
      ]
  };

  const res = await fetch(
      `https://vision.googleapis.com/v1/images:annotate?key=${api_key}`, 
      {
          method: 'POST',
          headers: {
              'Content-Type': 'application/json; charset=utf-8'
          },
          body: {
              type: "Json", 
              payload: requestData
          }
      }
  );

  if (res.ok) {
      const result = res.data;
      const text = result?.responses?.[0]?.fullTextAnnotation?.text;
      
      if (text) {
          return text;
      } else {
          throw new Error("Response Parse Error");
      }
  } else {
      throw new Error(`Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`); 
  }
}