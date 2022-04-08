import { marked } from "marked";
import Prism from "prismjs";
import "./chain.css";

const Chain = ({ chain, transactionPool }) => {
  return (
    <>
      <div
        id="chain-container"
        dangerouslySetInnerHTML={{
          __html: parseMarkdown(
            `## Chain\n\n\`\`\`json\n${JSON.stringify(chain, null, 2)}\n\`\`\``
          ),
        }}
      ></div>
      <div
        id="transaction-pool-container"
        dangerouslySetInnerHTML={{
          __html: parseMarkdown(
            `## Transaction Pool \n\n\`\`\`json\n${JSON.stringify(
              transactionPool,
              null,
              2
            )}\n\`\`\``
          ),
        }}
      ></div>
    </>
  );
};

marked.setOptions({
  highlight: (code, lang) => {
    if (Prism.languages[lang]) {
      return Prism.highlight(code, Prism.languages[lang], lang);
    } else {
      return code;
    }
  },
});

function parseMarkdown(markdown = "") {
  return marked.parse(markdown, { gfm: true });
}

export default Chain;

// .tech
/*
MAIN IDEA: Publicise. Teach/Walk Campers through the first "two drives"
Let's Play: Tom and Shaun pair-dev the first two
Dedicate Thursday and Friday to AV setup, and record Let's Play
Perhaps 90min per drive
Capture self going into this "fresh"
KEEP IT FUN!

Ask friends/family for any external mics


*/
