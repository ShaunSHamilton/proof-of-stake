import MonacoEditor from "react-monaco-editor";
import { useState } from "react";

// import Prism from "prismjs";
// import "prismjs/themes/prism-tomorrow.css";

const Editor = () => {
  const [code, setCode] = useState("");
  const [language, setLanguage] = useState("html");

  function editorDidMount(editor, monaco) {
    // console.log("editorDidMount", editor);
    editor.focus();
  }
  function onChange(newValue, e) {
    // console.log("onChange", newValue, e);
  }
  return (
    <div className="editor">
      <div className="monaco-editor">
        <MonacoEditor
          width="800"
          height="600"
          language={language}
          theme="vs-dark"
          value={code}
          onChange={onChange}
          editorDidMount={editorDidMount}
        />
      </div>
    </div>
  );
};

export default Editor;
