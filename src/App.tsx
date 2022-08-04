import { invoke, process } from "@tauri-apps/api";
import { useState } from "react";
import "./App.css";

function App() {
	const [text, setText] = useState("");

	const handleKeyDown = async (e: { key: string }) => {
		if (e.key === "Enter") {
			await invoke("add_task", { content: text });
			process.exit();
		}
	};

	return (
		<input
			type='text'
			value={text}
			onChange={(e) => setText(e.target.value)}
			onKeyDown={handleKeyDown}
			className='w-[840px] h-[70px] bg-[#1f1e1e] font-mono text-2xl text-white px-6 rounded-lg mx-sm'
		/>
	);
}

export default App;
