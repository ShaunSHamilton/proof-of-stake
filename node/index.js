import { handleClientWebSocket } from "./handle-client";
import { handleNodeWebsockets } from "./handle-node";

(async () => {
  await handleNodeWebsockets();
  handleClientWebSocket();
})();
