function parse(obj) {
  return JSON.stringify(obj);
}

function parseBuffer(buf) {
  return JSON.parse(buf.toString());
}

module.exports = {
  parse,
  parseBuffer,
};
