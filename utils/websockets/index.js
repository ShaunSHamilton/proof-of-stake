import PerfMetrics from "../perf-metrics.js";

const LEVEL = process.env.LOG_LEVEL || "info";

const perfMetrics = new PerfMetrics();

export async function findPortWebSocketServerListens(
  WebSocketConstructor,
  { timeout = 1500, startPort = 30000, endPort = 65000, numberOfPorts = 1 } = {}
) {
  const listeningPorts = [];
  let batchSize = 10;
  let batch = 0;
  const opening = startPort;
  const closing = endPort - batchSize;

  while (batch * batchSize + opening < closing) {
    try {
      const port = await new Promise(async (resolve, reject) => {
        // Ping websockets until response
        const start = opening + batch * batchSize;
        const end = start + batchSize;
        let closedSockets = 0;
        for (let i = start; i < end; i++) {
          let endTime = 0;
          const socket = new WebSocketConstructor(`ws://localhost:${i}`);
          let startTime = performance.now();

          const time = setTimeout(() => {
            socket.close();
          }, timeout);
          socket.onopen = (event) => {
            socket.send(
              parse({
                data: null,
                name: process.env.NAME || "anon",
                type: "ping",
              })
            );
            socket.close();
            clearTimeout(time);
            listeningPorts.push(i);
            if (numberOfPorts === listeningPorts.length) {
              return resolve(listeningPorts);
            }
          };
          socket.onerror = (_event) => {
            socket.close();
            clearTimeout(time);
          };
          socket.onclose = (_event) => {
            endTime = performance.now();
            perfMetrics.addMetric({
              id: i,
              startTime,
              endTime,
            });
            closedSockets++;
          };
        }
        const interval = setInterval(() => {
          // Check if all sockets are closed
          if (closedSockets >= end - start) {
            clearInterval(interval);
            return reject("No port found: " + start + " - " + end);
          }
          // Poll every 10ms
        }, 10);
      });
      info("Found port: ", port);
      if (numberOfPorts === listeningPorts.length) {
        perfMetrics.calcAverage();
        debug(`Average time socket lived: ${perfMetrics.average()}`);
        return listeningPorts;
      }
      batch++;
    } catch (e) {
      batch++;
      debug(e);
    }
  }
  perfMetrics.calcAverage();
  debug(`Average time socket lived: ${perfMetrics.average()}`);
  return listeningPorts;
}

export function parse(obj) {
  return JSON.stringify(obj);
}

export function parseBuffer(buf) {
  return JSON.parse(buf.toString());
}

const LogLevel = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
};

export function info(...args) {
  if (LogLevel[LEVEL] <= LogLevel.info) {
    console.info("🔵%cINFO: ", "color: blue", ...args);
  }
}
export function warn(...args) {
  if (LogLevel[LEVEL] <= LogLevel.warn) {
    console.warn("🟠%cWARN: ", "color: orange", ...args);
  }
}
export function error(...args) {
  if (LogLevel[LEVEL] <= LogLevel.error) {
    console.error("🔴%cERROR: ", "color: red", ...args);
  }
}
export function debug(...args) {
  if (LogLevel[LEVEL] === LogLevel.debug) {
    console.trace("[DEBUG]");
    console.debug("🟢%cDEBUG: ", "color: green", ...args);
  }
}

export async function findAvailablePort(startPort, endPort) {
  try {
    const availablePort = await new Promise(async (resolve, reject) => {
      for (let port = startPort; port < endPort; port++) {
        const server = (await import("net")).createServer();
        server.listen(port);
        server.on("listening", () => {
          server.close();
          return resolve(port);
        });
        server.on("error", (err) => {
          if (err.code === "EADDRINUSE") {
            debug(`Port in use: ${port}`);
          } else {
            error(err);
          }
        });
      }
    });
    return availablePort;
  } catch (err) {
    error(err);
  }
}
