import React, { useState } from "react";
import "./App.css";

import { scrapboxToMarkdown } from "../main";

interface FormProps {
  value: string;
  onChange?: (event: React.ChangeEvent<HTMLTextAreaElement>) => void;
}

const Form = (props: FormProps) => {
  const { value, onChange } = props;
  if (onChange) {
    return <textarea value={value} onChange={onChange} />;
  } else {
    return <textarea defaultValue={value} />;
  }
};

function App() {
  const [count, setCount] = useState(0);
  const [src, setSrc] = useState("");
  const [dst, setDst] = useState(src);

  const onChange = async (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    const input = event.target.value;
    setSrc(event.target.value);
    const dst = await scrapboxToMarkdown(input);
    setDst(dst);
  };

  return (
    <div className="App">
      <header className="App-header">
        <p>Hello Vite + React!</p>
        <p>
          <button type="button" onClick={() => setCount((count) => count + 1)}>
            count is: {count}
          </button>
        </p>
        <Form value={src} onChange={onChange} />
        <Form value={dst} />
      </header>
    </div>
  );
}

export default App;
