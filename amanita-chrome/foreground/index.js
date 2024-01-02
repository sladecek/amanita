console.debug('Load WASM background module..');
eval('./pkg/amanita_chrome.js');

wasm_bindgen("./pkg/amanita_chrome_bg.wasm")
  .then(module => {
    console.info('WASM loaded');
    module.start();
  })
  .catch(console.error);
