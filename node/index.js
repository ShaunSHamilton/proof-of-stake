import { handleClientWebSocket } from "./handle-client.js";
import { handleNodeWebsockets } from "./handle-node.js";

(async () => {
  await handleNodeWebsockets();
  handleClientWebSocket();
})();
