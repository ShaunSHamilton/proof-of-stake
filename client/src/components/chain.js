import { marked } from "marked";
import Prism from "prismjs";
import "./chain.css";

const Chain = ({ chain }) => {
  return (
    <div
      id="chain-container"
      dangerouslySetInnerHTML={{
        __html: parseMarkdown(
          `## Chain\n\n\`\`\`json\n${JSON.stringify(chain, null, 2)}\n\`\`\``
        ),
      }}
    ></div>
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
