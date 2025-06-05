const { Validator } = require('./validator_runtime');
const { startWebSocketServer } = require('./web_server');
const { runNetwork } = require('./validator_network');

async function main() {
  // Initialize validator
  const validator = new Validator();
  validator.start();

  // Start WebSocket server for UI
  await startWebSocketServer();

  // Start P2P network
  await runNetwork('127.0.0.1', 9000);

  console.log('Validator node started successfully');
}

main().catch(console.error);